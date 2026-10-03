use std::fmt;
use std::slice;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Address {
    address: usize,
}

impl Address {
    const DEFAULT_SCAN_RANGE: usize = 4096;

    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    #[inline]
    pub const fn get(&self) -> usize {
        self.address
    }

    #[inline]
    pub const fn as_ptr<T>(&self) -> *const T {
        self.address as *const T
    }

    #[inline]
    pub const fn as_mut_ptr<T>(&self) -> *mut T {
        self.address as *mut T
    }

    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.address >= 0x1000 && self.address <= 0x7FFF_FFFF_FFFF
    }

    #[inline]
    pub const fn add(&self, offset: usize) -> Self {
        Self::new(self.address.wrapping_add(offset))
    }

    #[inline]
    pub const fn sub(&self, offset: usize) -> Self {
        Self::new(self.address.wrapping_sub(offset))
    }

    #[inline]
    pub fn jmp(&self) -> Self {
        self.follow_rel(1)
    }

    #[inline]
    pub fn rip(&self) -> Self {
        self.follow_rel(3)
    }

    pub fn find_opcode(&self, opcode: u8, range: Option<usize>) -> Self {
        let range = range.unwrap_or(Self::DEFAULT_SCAN_RANGE);

        if !self.is_valid() {
            return Self::new(0);
        }

        let bytes = unsafe { slice::from_raw_parts(self.address as *const u8, range) };

        match bytes.iter().position(|&b| b == opcode) {
            Some(index) => Self::new(self.address + index),
            None => Self::new(0),
        }
    }

    pub fn find_opcode_sequence(&self, sequence: &[u8], range: Option<usize>) -> Self {
        let range = range.unwrap_or(Self::DEFAULT_SCAN_RANGE);

        if !self.is_valid() {
            return Self::new(0);
        }

        if sequence.is_empty() {
            return *self;
        }

        let bytes = unsafe { slice::from_raw_parts(self.address as *const u8, range) };

        match bytes
            .windows(sequence.len())
            .position(|window| window == sequence)
        {
            Some(index) => Self::new(self.address + index),
            None => Self::new(0),
        }
    }

    pub fn read(&self, deref_count: usize) -> Self {
        if !self.is_valid() {
            return Self::new(0);
        }

        let mut address = self.address;

        for _ in 0..deref_count {
            address = unsafe { (address as *const usize).read() };

            if address == 0 {
                return Self::new(0);
            }
        }

        Self::new(address)
    }

    pub fn follow_rel(&self, disp_offset: usize) -> Self {
        if !self.is_valid() {
            return Self::new(0);
        }

        let disp_ptr = self.address.wrapping_add(disp_offset);
        let disp = unsafe { (disp_ptr as *const i32).read_unaligned() } as isize;

        let mut target = self.address;

        target = target.wrapping_add(disp_offset);
        target = target.wrapping_add(size_of::<i32>());
        target = target.wrapping_add_signed(disp);

        Self::new(target)
    }
}

impl From<usize> for Address {
    #[inline]
    fn from(address: usize) -> Self {
        Self::new(address)
    }
}

impl From<Address> for usize {
    #[inline]
    fn from(address: Address) -> Self {
        address.address
    }
}

impl fmt::Display for Address {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.address)
    }
}

impl fmt::Debug for Address {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.address)
    }
}
