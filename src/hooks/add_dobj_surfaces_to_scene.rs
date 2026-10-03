use std::ffi::c_void;

use retour::static_detour;

pub type RAddDObjSurfacesToSceneFn =
    unsafe extern "C" fn(*const c_void, *const c_void, u32, u32, *const f32, f32, i32);

static_detour! {
    pub static RAddDObjSurfacesToSceneHook: unsafe extern "C" fn(
        *const c_void, *const c_void, u32, u32, *const f32, f32, i32
    );
}

pub fn hooked_add_dobj_surfaces_to_scene(
    dobj: *const c_void,
    centity: *const c_void,
    entity_number: u32,
    mut render_flags: u32,
    lighting_origin: *const f32,
    alpha: f32,
    unk: i32,
) {
    // comment these lines out to remove the wallhack
    if entity_number <= 18 {
        render_flags = (render_flags | 0xFFFF) & !0x1000;
    }

    unsafe {
        RAddDObjSurfacesToSceneHook.call(
            dobj,
            centity,
            entity_number,
            render_flags,
            lighting_origin,
            alpha,
            unk,
        )
    }
}
