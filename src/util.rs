//! Short helpers so move scripts read like frame data instead of 37-argument calls.
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;

/// Standard hitbox. `lr` = ATTACK_LR_CHECK_* (F = launches the way he faces, B = backwards, POS = away from him).
pub unsafe fn hit(
    agent: &mut L2CAgentBase, id: u64, part: u64, bone: &str, dmg: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32,
    size: f32, x: f32, y: f32, z: f32, hitlag: f32, lr: i32, attr: &str, lvl: i32, snd: i32, region: i32,
) {
    macros::ATTACK(
        agent, id, part, Hash40::new(bone), dmg, angle, kbg, fkb, bkb, size, x, y, z, None, None, None,
        hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, lr, false, 0, 0.0, 0, false, false, false, false, true,
        *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,
        Hash40::new(attr), lvl, snd, region,
    );
}

/// Stretched (capsule) hitbox from (y,z) to (y2,z2), for sweeping limbs and beams.
pub unsafe fn hit_cap(
    agent: &mut L2CAgentBase, id: u64, part: u64, bone: &str, dmg: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32,
    size: f32, y: f32, z: f32, y2: f32, z2: f32, hitlag: f32, lr: i32, attr: &str, lvl: i32, snd: i32, region: i32,
) {
    macros::ATTACK(
        agent, id, part, Hash40::new(bone), dmg, angle, kbg, fkb, bkb, size, 0.0, y, z, Some(0.0), Some(y2), Some(z2),
        hitlag, 1.0, *ATTACK_SETOFF_KIND_ON, lr, false, 0, 0.0, 0, false, false, false, false, true,
        *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,
        Hash40::new(attr), lvl, snd, region,
    );
}

/// One-shot effect at a bone (offset x,y,z / rotation in degrees / scale)
pub unsafe fn fx(agent: &mut L2CAgentBase, name: &str, bone: &str, x: f32, y: f32, z: f32, rx: f32, ry: f32, rz: f32, scale: f32) {
    macros::EFFECT(agent, Hash40::new(name), Hash40::new(bone), x, y, z, rx, ry, rz, scale, 0, 0, 0, 0, 0, 0, true);
}
/// Effect that stays attached to a bone
pub unsafe fn fx_follow(agent: &mut L2CAgentBase, name: &str, bone: &str, x: f32, y: f32, z: f32, rx: f32, ry: f32, rz: f32, scale: f32) {
    macros::EFFECT_FOLLOW(agent, Hash40::new(name), Hash40::new(bone), x, y, z, rx, ry, rz, scale, true);
}
/// Tint the last spawned effect (0-1 rgb, values above 1 glow)
pub unsafe fn tint(agent: &mut L2CAgentBase, r: f32, g: f32, b: f32) {
    macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
}
pub unsafe fn purple(agent: &mut L2CAgentBase) { tint(agent, 0.65, 0.2, 1.0); }
pub unsafe fn se(agent: &mut L2CAgentBase, name: &str) { macros::PLAY_SE(agent, Hash40::new(name)); }
