//! Scraggy specials (Squirtle's statuses underneath, re-scripted):
//!   Neutral B  FOCUS PUNCH  - Water Gun's charge (hold B, storable): the release is a lunging punch whose power scales
//!                             with the charge instead of a water stream.
//!   Side B     HEADBUTT     - Withdraw's launch: leaps head-first; on hit he bounces off (Withdraw's hit status),
//!                             on a miss he belly-flops and rolls (mod.rs cuts the slide short).
//!   Up B       BRICK BREAK  - Waterfall's rise: fist smashes the ground first (spike), then he springs up.
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;
use super::normals::{orange, yellow};

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

// ======================================================================= NEUTRAL B: FOCUS PUNCH
pub unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6); yellow(agent); }
}
pub unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    at!(agent, 2);
    if now!(agent) { se(agent, "se_pzenigame_squat"); }
}
pub unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    // pulsing yellow glow on the right fist through the whole charge loop
    for f in [1.0f32, 9.0, 17.0, 25.0, 33.0, 41.0] {
        frame(agent.lua_state_agent, f);
        if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7); yellow(agent); }
    }
}
pub unsafe extern "C" fn sound_specialnhold(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { se(agent, "se_pzenigame_special_n01"); }
}
pub unsafe extern "C" fn game_specialnshot(agent: &mut L2CAgentBase) {
    // 0..1 charge (counted in mod.rs while he holds the charge)
    let e = (WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
    let c = super::focus_charge(e);
    let dmg = 6.0 + 16.0 * c;
    let bkb = (30.0 + 40.0 * c) as i32;
    at!(agent, 3);
    if now!(agent) {
        super::focus_spend(e);
        hit(agent, 0, 0, "handr", dmg, 361, 85, 0, bkb, 3.6, 0.0, 0.0, 0.0, 1.0 + 0.8 * c, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        hit_cap(agent, 1, 0, "top", dmg * 0.85, 361, 85, 0, bkb, 2.8, 5.0, 4.0, 5.0, 12.0, 1.0 + 0.6 * c, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 7);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_specialnshot(agent: &mut L2CAgentBase) {
    at!(agent, 2);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 3);
    if now!(agent) {
        fx(agent, "sys_smash_flash", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9); yellow(agent);
        fx(agent, "sys_crown", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.5); tint(agent, 1.0, 1.0, 1.0);
    }
}
pub unsafe extern "C" fn sound_specialnshot(agent: &mut L2CAgentBase) {
    at!(agent, 2);
    if now!(agent) { se(agent, "se_pzenigame_smash_s01"); se(agent, "vc_pzenigame_attack06"); }
}

// ======================================================================= SIDE B: HEADBUTT
unsafe fn headbutt(agent: &mut L2CAgentBase) {
    JostleModule::set_status(agent.module_accessor, false);
    hit(agent, 0, 0, "head", 10.0, 50, 70, 0, 55, 3.6, 0.0, 0.0, 0.0, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
}
pub unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { headbutt(agent); }
}
pub unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 7);
    if now!(agent) { fx_follow(agent, "sys_attack_speedline", "head", 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 0.7); orange(agent); }
}
pub unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { se(agent, "se_pzenigame_jump01"); se(agent, "vc_pzenigame_attack05"); }
}
pub unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    if now!(agent) { headbutt(agent); }
}
pub unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { fx_follow(agent, "sys_attack_speedline", "head", 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 0.7); orange(agent); }
}
pub unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}
pub unsafe extern "C" fn effect_specialshit(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0); tint(agent, 1.0, 1.0, 1.0);
        fx(agent, "sys_crown", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.5); yellow(agent);
    }
}
pub unsafe extern "C" fn sound_specialshit(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { se(agent, "se_pzenigame_landing02"); }
    at!(agent, 4);
    if now!(agent) { se(agent, "se_pzenigame_jump02"); }
}
pub unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 12);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false); }
}
pub unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { se(agent, "se_common_down_soil_s"); }
    at!(agent, 12);
    if now!(agent) { se(agent, "se_pzenigame_escape"); }
}

// ======================================================================= UP B: BRICK BREAK
unsafe extern "C" fn brick_game(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR); }
    at!(agent, 10);
    if now!(agent) {
        // the rock smash: fist and the shock under it spike anything in front/below
        hit(agent, 0, 0, "handr", 12.0, 270, 80, 0, 30, 3.8, 0.0, 0.0, 0.0, 1.5, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 10.0, 270, 80, 0, 30, 4.2, 0.0, 2.0, 3.5, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 12);
    if now!(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        // springing up off the smash: a weaker rising hit
        hit(agent, 0, 0, "head", 5.0, 80, 90, 0, 45, 4.0, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "handr", 5.0, 80, 90, 0, 45, 3.2, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 18);
    if now!(agent) { notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); }
    at!(agent, 21);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
    at!(agent, 40);
    if now!(agent) { notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); }
}
pub unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) { brick_game(agent) }
pub unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) { brick_game(agent) }
unsafe extern "C" fn brick_effect(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6); yellow(agent); }
    at!(agent, 10);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 1.0, 3.0, 0.0, 0.0, 0.0, 1.1); yellow(agent);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 12);
    if now!(agent) { fx_follow(agent, "sys_attack_speedline", "top", 0.0, 4.0, 0.0, -90.0, 0.0, 0.0, 0.7); orange(agent); }
}
pub unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) { brick_effect(agent) }
pub unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) { brick_effect(agent) }
unsafe extern "C" fn brick_sound(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { se(agent, "se_pzenigame_squat"); }
    at!(agent, 9);
    if now!(agent) { se(agent, "vc_pzenigame_attack06"); se(agent, "se_pzenigame_smash_s01"); }
    at!(agent, 10);
    if now!(agent) { se(agent, "se_common_down_soil_s"); }
    at!(agent, 12);
    if now!(agent) { se(agent, "se_pzenigame_jump02"); }
}
pub unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) { brick_sound(agent) }
pub unsafe extern "C" fn sound_specialairhi(agent: &mut L2CAgentBase) { brick_sound(agent) }
