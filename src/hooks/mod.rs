pub use add_dobj_surfaces_to_scene::*;
pub use is_item_unlocked::*;

use crate::managers::PatternManager;

use anyhow::Result;
use std::mem;

mod add_dobj_surfaces_to_scene;
mod is_item_unlocked;

pub fn setup() -> Result<()> {
    unsafe {
        IsItemUnlockedHook
            .initialize(
                mem::transmute::<_, IsItemUnlockedFn>(
                    PatternManager::instance()
                        .address("IsItemUnlocked")
                        .unwrap(),
                ),
                hooked_is_item_unlocked,
            )?
            .enable()?;

        RAddDObjSurfacesToSceneHook
            .initialize(
                mem::transmute::<_, RAddDObjSurfacesToSceneFn>(
                    PatternManager::instance()
                        .address("R_AddDObjSurfacesToScene")
                        .unwrap(),
                ),
                hooked_add_dobj_surfaces_to_scene,
            )?
            .enable()?;
    }

    Ok(())
}

pub fn restore() {
    unsafe {
        IsItemUnlockedHook.disable().ok();
        RAddDObjSurfacesToSceneHook.disable().ok();
    }
}
