use std::collections::HashMap;
use std::sync::LazyLock;

use pelite::pattern;
use pelite::pattern::Atom;

use phf::phf_map;

use crate::utils::{Address, Fnv1aHash, Module, Peb};

type AddressTransformFn = fn(usize) -> usize;

pub struct Pattern {
    pub signature: &'static [Atom],
    pub module: Option<&'static str>,
    pub cursor: Option<usize>,
    pub transform: Option<AddressTransformFn>,
}

impl Pattern {
    pub const fn new(signature: &'static [Atom]) -> Self {
        Self {
            signature,
            module: None,
            cursor: None,
            transform: None,
        }
    }

    pub const fn in_module(mut self, module: &'static str) -> Self {
        self.module = Some(module);
        self
    }

    pub const fn cursor(mut self, cursor: usize) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub const fn transform(mut self, transform: AddressTransformFn) -> Self {
        self.transform = Some(transform);
        self
    }
}

static PATTERNS: phf::Map<&'static str, Pattern> = phf_map! {
    "IsItemUnlocked" =>
        Pattern::new(pattern!("48895c24? 48896c24? 48897424? 57 4883ec? 8be9 418bf8")),
    "R_AddDObjSurfacesToScene" => Pattern::new(pattern!("4056 4154 4155 4157 4883ec48")),
    "Dvar_FindVar" => Pattern::new(pattern!("48895c24? 57 4883ec? 48896c24")),
    "Present" => Pattern::new(pattern!("48895c24? 48896c24? 56 57 4154 4156 4157 4883ec? 418bf0"))
        .in_module("GameOverlayRenderer64.dll"),
};

pub struct PatternManager {
    patterns: HashMap<Fnv1aHash, &'static Pattern>,
    addresses: HashMap<Fnv1aHash, Address>,
}

impl PatternManager {
    pub fn new() -> Self {
        let peb = Peb::instance();

        let mut patterns = HashMap::new();
        let mut addresses = HashMap::new();

        for (name, pattern) in PATTERNS.entries() {
            let key = (*name).into();

            let module = match pattern.module {
                Some(module) => peb.module(module),
                None => peb.main_module(),
            };

            let module = match module {
                Some(module) => module,
                None => {
                    log::error!(
                        "{name}: module {} is not loaded",
                        pattern.module.unwrap_or("<main>")
                    );

                    continue;
                }
            };

            if let Some(address) = Self::scan(module, name, pattern) {
                addresses.insert(key, address);
            }

            patterns.insert(key, pattern);
        }

        Self {
            patterns,
            addresses,
        }
    }

    #[inline]
    pub fn instance() -> &'static Self {
        static INSTANCE: LazyLock<PatternManager> = LazyLock::new(PatternManager::new);
        &INSTANCE
    }

    #[inline]
    pub fn get(&self, key: impl Into<Fnv1aHash>) -> Option<&'static Pattern> {
        self.patterns.get(&key.into()).copied()
    }

    #[inline]
    pub fn address(&self, key: impl Into<Fnv1aHash>) -> Option<Address> {
        self.addresses.get(&key.into()).copied()
    }

    fn scan(module: &Module, name: &str, pattern: &'static Pattern) -> Option<Address> {
        let mut signature = Vec::with_capacity(pattern.signature.len() + 1);

        signature.push(Atom::Save(0));
        signature.extend_from_slice(pattern.signature);

        let mut rvas = vec![0; pattern.signature.len() + 1];

        if !module.find_pattern(&signature, &mut rvas) {
            log::error!(
                "{name}: signature is missing or ambiguous in {}",
                module.name
            );

            return None;
        }

        let mut address = module.base.add(rvas[0] as usize);

        if let Some(cursor) = pattern.cursor {
            address = address.add(cursor);
        }

        if let Some(transform) = pattern.transform {
            address = Address::new(transform(address.get()));
        }

        log::info!("{name}: {address}");

        Some(address)
    }
}
