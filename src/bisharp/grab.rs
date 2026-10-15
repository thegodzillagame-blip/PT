//! Bisharp grabs (Captain Falcon's reach instead of Ivysaur's vine) and throws.
//! The opponent rides the Throw bone; release frames match the throw animations (BISHARP_FRAME_DATA.txt).
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;
use super::{steel, dark, red};

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

unsafe fn grab_box(agent: &mut L2CAgentBase, y: f32, z1: f32, z2: f32, size: f32) {
    macros::CATCH(agent, 0, Hash40::new("top"), size, 0.0, y, z1, Some(0.0), Some(y), Some(z2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
}
unsafe fn grab_end(agent: &mut L2CAgentBase) {
    grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    GrabModule::set_rebound(agent.module_accessor, false);
}
pub unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { GrabModule::set_rebound(agent.module_accessor, true); }
    at!(agent, 7);
    if now!(agent) { grab_box(agent, 11.5, 4.0, 11.5, 3.6); }
    macros::game_CaptureCutCommon(agent);
    at!(agent, 9);
    if now!(agent) { grab_end(agent); }
}
pub unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { GrabModule::set_rebound(agent.module_accessor, true); }
    at!(agent, 10);
    if now!(agent) { grab_box(agent, 10.0, 4.0, 13.5, 3.2); }
    macros::game_CaptureCutCommon(agent);
    at!(agent, 12);
    if now!(agent) { grab_end(agent); }
}
pub unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { GrabModule::set_rebound(agent.module_accessor, true); }
    at!(agent, 9);
    if now!(agent) { grab_box(agent, 9.0, -4.0, -14.5, 3.4); }
    macros::game_CaptureCutCommon(agent);
    at!(agent, 11);
    if now!(agent) { grab_end(agent); }
}
pub unsafe extern "C" fn sound_catch(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { se(agent, "se_common_swing_05"); }
}

unsafe fn release(agent: &mut L2CAgentBase) {
    let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    let group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
    let no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
    macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, group, no);
}
unsafe fn throw_abs(agent: &mut L2CAgentBase, dmg: f32, angle: u64, kbg: i32, bkb: i32) {
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, dmg, angle, kbg, 0, bkb, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
}
// F-THROW Night Slash: cut on 9, release 9
pub unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 8.0, 40, 70, 60); }
    at!(agent, 8);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, 14, 8); }
    at!(agent, 9);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwf(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 12, 4, 0, 0, 0, 1.2, true, *EF_FLIP_YZ); dark(agent); }
    at!(agent, 9);
    if now!(agent) { fx(agent, "sys_attack_impact", "throw", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0); red(agent); }
}
pub unsafe extern "C" fn sound_throwf(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_common_swing_06"); se(agent, "se_common_throw_01"); }
}
// B-THROW Foul Play: swing and fling behind, release 10
pub unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 10.0, 135, 70, 60); }
    at!(agent, 9);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, -12, 10); }
    at!(agent, 10);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwb(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { fx(agent, "sys_attack_impact", "throw", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9); red(agent); }
}
pub unsafe extern "C" fn sound_throwb(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "se_common_swing_05"); }
    at!(agent, 10);
    if now!(agent) { se(agent, "se_common_throw_02"); }
}
// U-THROW Iron Head: tossed up, crest hit on 10, release 10
pub unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 9.0, 90, 85, 60); }
    at!(agent, 9);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, 2, 22); }
    at!(agent, 10);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 19.0, 5.5, 0.0, 0.0, 0.0, 1.0); red(agent);
        fx_follow(agent, "sys_attack_line", "top", 0.0, 16.0, 5.5, -90.0, 0.0, 0.0, 1.0); steel(agent);
    }
}
pub unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { se(agent, "se_common_swing_05"); }
    at!(agent, 10);
    if now!(agent) { se(agent, "se_common_throw_01"); }
}
// D-THROW Retaliate: slam then stab, release 12
pub unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    if now!(agent) { throw_abs(agent, 7.0, 75, 60, 70); }
    at!(agent, 11);
    if now!(agent) { macros::CHECK_FINISH_CAMERA(agent, 6, 0); }
    at!(agent, 12);
    if now!(agent) { release(agent); }
}
pub unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 12);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "throw", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0); red(agent);
        fx_follow(agent, "sys_attack_line", "top", 0.0, 20.0, 4.0, 90.0, 0.0, 0.0, 1.0); dark(agent);
    }
}
pub unsafe extern "C" fn sound_throwlw(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_common_down_soil_s"); }
    at!(agent, 12);
    if now!(agent) { se(agent, "se_common_swing_06"); }
}
