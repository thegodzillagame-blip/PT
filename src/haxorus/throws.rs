//! Haxorus' custom throws. The opponent rides the "throw" bone of the animation until ATK_HIT_ABS releases them,
//! so the release frame here must match the animation (see the v45 throw re-timing).
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

unsafe fn release(agent: &mut L2CAgentBase) {
    let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    let group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
    let no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, group, no);
}

// ===================================================================== UP THROW: heave sky-high, Dragon Pulse on the way down (85f, release f50)
pub unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    if now!(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 88, 75, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    at!(agent, 49);
    if now!(agent) {
        // the beam column itself, for anyone standing in it
        hit_cap(agent, 0, 0, "top", 8.0, 85, 80, 0, 60, 4.5, 20.0, 1.0, 34.0, 3.5, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_purple", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 1, 30);
    }
    at!(agent, 50);
    if now!(agent) { release(agent); }
    at!(agent, 51);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 40);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2); purple(agent); }
    at!(agent, 46);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5); purple(agent); }
    at!(agent, 49);
    if now!(agent) {
        // Dragon Pulse: purple beam straight up out of his mouth
        fx(agent, "sys_attack_line", "top", 0.0, 20.0, 1.0, -90.0, 0.0, 0.0, 2.2);
        purple(agent);
        fx(agent, "sys_attack_line", "top", 0.0, 26.0, 2.0, -90.0, 0.0, 0.0, 1.8);
        tint(agent, 0.95, 0.75, 1.0);
        fx(agent, "sys_crown", "top", 0.0, 21.0, 1.0, 0.0, 0.0, 0.0, 0.5);
        purple(agent);
    }
    at!(agent, 50);
    if now!(agent) { fx(agent, "sys_attack_impact", "throw", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.6); purple(agent); }
}
pub unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { se(agent, "se_common_throw_02"); se(agent, "vc_plizardon_attack05"); }
    at!(agent, 40);
    if now!(agent) { se(agent, "se_plizardon_special_s01"); }
    at!(agent, 49);
    if now!(agent) { se(agent, "vc_plizardon_attack07"); se(agent, "se_plizardon_smash_h02"); }
}
pub unsafe extern "C" fn expression_throwhi(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0); }
    at!(agent, 50);
    if now!(agent) { macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M); }
}

// ===================================================================== DOWN THROW: Body Press (pin, hop, belly-flop) - release f55
pub unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    if now!(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 70, 70, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    at!(agent, 54);
    if now!(agent) {
        // the belly-flop shockwave clips anyone standing next to the pin
        hit(agent, 0, 0, "top", 5.0, 70, 70, 0, 60, 6.5, 0.0, 3.0, 9.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 8, 0);
    }
    at!(agent, 55);
    if now!(agent) { release(agent); }
    at!(agent, 56);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 20);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 55);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 3.0, 9.0, 0.0, 0.0, 0.0, 1.5);
        fx(agent, "sys_crown", "top", 0.0, 0.0, 6.0, 0.0, 0.0, 0.0, 1.1);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 6, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 20);
    if now!(agent) { se(agent, "se_plizardon_jump01"); }
    at!(agent, 54);
    if now!(agent) { se(agent, "se_plizardon_landing02"); se(agent, "vc_plizardon_attack06"); }
}
pub unsafe extern "C" fn expression_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 55);
    if now!(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}
