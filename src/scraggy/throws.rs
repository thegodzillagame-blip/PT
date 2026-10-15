//! Scraggy's throws. Up throw stays Squirtle's. Release frames match the throw animations.
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;
use super::normals::{orange, red};

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

unsafe fn release(agent: &mut L2CAgentBase) {
    let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    let group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
    let no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, group, no);
}
unsafe fn throw_abs(agent: &mut L2CAgentBase, dmg: f32, angle: u64, kbg: i32, bkb: i32, attr: &str) {
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, angle, kbg, 0, bkb, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new(attr), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
}

// ===================================================================== F-THROW: headbutt (release f11)
pub unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 8.0, 45, 75, 60, "collision_attr_normal"); }
    at!(agent, 10);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, 12, 6); }
    at!(agent, 11);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
    at!(agent, 11);
    if now!(agent) { fx(agent, "sys_attack_impact", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9); red(agent); }
}
pub unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); se(agent, "vc_pzenigame_attack05"); }
    at!(agent, 11);
    if now!(agent) { se(agent, "se_common_throw_01"); }
}

// ===================================================================== D-THROW: tail trip - slam down, tail sweeps their legs: knockdown, no bounce (release f17)
pub unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 6.0, 361, 50, 60, "collision_attr_fist_down"); }
    at!(agent, 16);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, 6, 0); }
    at!(agent, 17);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 12);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 1.5, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ); orange(agent); }
}
pub unsafe extern "C" fn sound_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { se(agent, "se_common_down_soil_s"); }
    at!(agent, 12);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}

// ===================================================================== B-THROW: back roll, tossed overhead behind him (release f14)
pub unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 9.0, 135, 70, 60, "collision_attr_normal"); }
    at!(agent, 13);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, -8, 8); }
    at!(agent, 14);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 7, -2, 0, -52, -88, 0.9, true, *EF_FLIP_YZ); tint(agent, 1.0, 1.0, 1.0); }
    at!(agent, 14);
    if now!(agent) { fx(agent, "sys_attack_impact", "throw", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8); red(agent); }
}
pub unsafe extern "C" fn sound_throwb(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_pzenigame_escape"); }
    at!(agent, 14);
    if now!(agent) { se(agent, "se_common_throw_01"); se(agent, "vc_pzenigame_attack06"); }
}
