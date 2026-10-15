//! Scraggy's normals and aerials (Squirtle's slot c01). Frames are the frames of Scraggy's own animations
//! (see SCRAGGY_FRAME_DATA.txt in the mod folder); hitboxes ride the bone that does the hitting.
//! Effects: orange trails, red impact flashes (yellow is kept for the Focus Punch charge glow).
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

pub unsafe fn orange(agent: &mut L2CAgentBase) { tint(agent, 1.0, 0.55, 0.15); }
pub unsafe fn yellow(agent: &mut L2CAgentBase) { tint(agent, 1.0, 0.9, 0.3); }
/// hit flashes are red
pub unsafe fn red(agent: &mut L2CAgentBase) { tint(agent, 1.0, 0.18, 0.12); }
unsafe fn landing_on(agent: &mut L2CAgentBase) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
unsafe fn landing_off(agent: &mut L2CAgentBase) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
unsafe fn clear(agent: &mut L2CAgentBase) { AttackModule::clear_all(agent.module_accessor); }
/// punch/kick hitbox on a bone (POS = away from Scraggy, F = the way he faces)
unsafe fn b(agent: &mut L2CAgentBase, id: u64, bone: &str, dmg: f32, angle: u64, kbg: i32, bkb: i32, size: f32, hitlag: f32,
            lr: i32, snd: i32, region: i32) {
    hit(agent, id, 0, bone, dmg, angle, kbg, 0, bkb, size, 0.0, 0.0, 0.0, hitlag, lr, "collision_attr_normal",
        if dmg >= 10.0 { *ATTACK_SOUND_LEVEL_L } else if dmg >= 6.0 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_S }, snd, region);
}

// ===================================================================== JAB: left jab, right jab (2 hits - no jab 3)
pub unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) {
        b(agent, 0, "handl", 2.5, 361, 25, 20, 2.4, 1.0, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 2.5, 361, 25, 0, 20, 2.0, 0.0, 5.5, 6.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 7);
    if now!(agent) { clear(agent); WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO); }
    at!(agent, 9);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART); }
}
pub unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 6, 1, 0, 0, 0, 0.75, true, *EF_FLIP_YZ); orange(agent); }
    at!(agent, 5);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 6.0, 9.5, 0.0, 0.0, 0.0, 0.6); red(agent); }
}
pub unsafe extern "C" fn sound_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}
pub unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) {
        // the follow-up: no combo flag, so there's no third jab
        b(agent, 0, "handr", 4.0, 50, 90, 45, 2.8, 1.3, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 4.0, 50, 90, 0, 45, 2.2, 0.0, 5.5, 7.5, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 6);
    if now!(agent) { clear(agent); }
}
pub unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 6, 1, 0, 0, 0, 0.85, true, *EF_FLIP_YZ); orange(agent); }
    at!(agent, 4);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 5.5, 11.0, 0.0, 0.0, 0.0, 0.8); red(agent); }
}
pub unsafe extern "C" fn sound_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); se(agent, "vc_pzenigame_attack05"); }
}
/// jab 3 can't be reached any more, but keep it harmless just in case
pub unsafe extern "C" fn game_attack13(_agent: &mut L2CAgentBase) {}

// ===================================================================== DASH ATTACK: trips into a belly-flop slide
pub unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) {
        b(agent, 0, "head", 9.0, 65, 70, 60, 3.6, 1.2, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        b(agent, 1, "bust", 9.0, 65, 70, 60, 3.2, 1.2, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 13);
    if now!(agent) {
        b(agent, 0, "head", 6.0, 70, 60, 50, 3.2, 1.0, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        b(agent, 1, "bust", 6.0, 70, 60, 50, 2.8, 1.0, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 18);
    if now!(agent) { clear(agent); }
}
pub unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 10);
    if now!(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        fx_follow(agent, "sys_attack_speedline", "head", 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 0.7); orange(agent);
    }
}
pub unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_common_slip_01"); se(agent, "vc_pzenigame_attack06"); }
    at!(agent, 11);
    if now!(agent) { se(agent, "se_common_down_soil_s"); }
}

// ===================================================================== F-TILT: tail whip (spins round, tail lashes in front)
pub unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) {
        b(agent, 0, "tail2", 8.0, 361, 90, 30, 3.0, 1.1, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        b(agent, 1, "tail1", 8.0, 361, 90, 30, 2.6, 1.1, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 2, 0, "top", 8.0, 361, 90, 0, 30, 2.6, 0.0, 4.0, 6.0, 1.1, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    at!(agent, 12);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.7); }   // less endlag
}
pub unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 4, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ); orange(agent); }
}
pub unsafe extern "C" fn sound_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}

// ===================================================================== U-TILT: head thrust straight up
pub unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) {
        b(agent, 0, "head", 6.0, 90, 110, 40, 3.4, 1.0, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "top", 6.0, 90, 110, 0, 40, 2.6, 0.0, 9.0, 1.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    at!(agent, 12);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.7); }   // less endlag
}
pub unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { fx_follow(agent, "sys_attack_line", "head", 0.0, 0.0, 0.0, -90.0, 0.0, 0.0, 0.6); orange(agent); }
    at!(agent, 8);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 17.0, 0.5, 0.0, 0.0, 0.0, 0.6); red(agent); }
}
pub unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}

// ===================================================================== D-TILT: ball roll along the floor
pub unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) {
        b(agent, 0, "bust", 6.0, 75, 60, 50, 3.4, 1.0, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    at!(agent, 12);
    if now!(agent) {
        b(agent, 0, "bust", 4.0, 80, 55, 45, 3.0, 0.8, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    at!(agent, 17);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.7); }   // less endlag
}
pub unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 9);
    if now!(agent) { fx_follow(agent, "sys_spin_wind_s", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.5); orange(agent); }
}
pub unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_pzenigame_escape"); }
}

// ===================================================================== F-SMASH: long-neck lunging headbutt (charge holds at f14)
pub unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    at!(agent, 14);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD); }
    at!(agent, 18);
    if now!(agent) {
        b(agent, 0, "head", 15.0, 361, 95, 32, 4.0, 1.4, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "top", 13.0, 361, 95, 0, 30, 3.0, 0.0, 5.0, 4.0, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 22);
    if now!(agent) { clear(agent); }
}
pub unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    at!(agent, 13);
    if now!(agent) { macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ); }
    at!(agent, 17);
    if now!(agent) {
        fx_follow(agent, "sys_attack_speedline", "head", 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 0.8); orange(agent);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 18);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 5.5, 8.5, 0.0, 0.0, 0.0, 1.1); red(agent); }
}
pub unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    at!(agent, 15);
    if now!(agent) { macros::STOP_SE(agent, Hash40::new("se_common_smash_start")); }
    at!(agent, 17);
    if now!(agent) { se(agent, "se_pzenigame_smash_s01"); se(agent, "vc_pzenigame_attack06"); }
}

// ===================================================================== U-SMASH: coils leaning back, hops and arcs the head back -> over -> down in front (charge holds at f6)
pub unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD); }
    at!(agent, 8);
    if now!(agent) {
        b(agent, 0, "head", 14.0, 85, 95, 35, 3.8, 1.3, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "top", 12.0, 90, 95, 0, 32, 3.0, 0.0, 7.0, 0.0, 1.1, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 11);
    if now!(agent) { AttackModule::clear(agent.module_accessor, 1, false); }
    at!(agent, 14);
    if now!(agent) { clear(agent); }
}
pub unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ); }
    at!(agent, 8);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, 0, 0, -52, -88, 0.9, true, *EF_FLIP_YZ); orange(agent);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { macros::STOP_SE(agent, Hash40::new("se_common_smash_start")); se(agent, "se_pzenigame_jump01"); }
    at!(agent, 8);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); se(agent, "vc_pzenigame_attack05"); }
}

// ===================================================================== D-SMASH: ground pound - fist raised overhead (charge holds at f6),
// slams the floor on 12 and the quake hits all around him
pub unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD); }
    at!(agent, 11);
    if now!(agent) {
        // the fist on the way down
        b(agent, 0, "handr", 15.0, 60, 92, 35, 4.2, 1.3, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 12);
    if now!(agent) {
        // the quake: a wide low band both sides of him
        hit_cap(agent, 1, 0, "top", 13.0, 70, 90, 0, 40, 4.0, 2.0, -13.0, 2.0, 13.0, 1.2, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    at!(agent, 13);
    if now!(agent) { AttackModule::clear(agent.module_accessor, 0, false); }
    at!(agent, 16);
    if now!(agent) { clear(agent); }
}
pub unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ); }
    at!(agent, 10);
    if now!(agent) { fx_follow(agent, "sys_attack_line", "handr", 0.0, 0.0, 0.0, 90.0, 0.0, 0.0, 0.7); orange(agent); }
    at!(agent, 12);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 1.5, 1.0, 0.0, 0.0, 0.0, 1.2); red(agent);
        fx(agent, "sys_quake", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.3);
        fx(agent, "sys_bomb_b", "top", 0.0, 0.5, 1.0, 0.0, 0.0, 0.0, 0.5); tint(agent, 0.55, 0.42, 0.3);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -7, 0, 180, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    at!(agent, 14);
    if now!(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, -11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { macros::STOP_SE(agent, Hash40::new("se_common_smash_start")); se(agent, "se_pzenigame_swing_l"); }
    at!(agent, 11);
    if now!(agent) { se(agent, "vc_pzenigame_attack06"); }
    at!(agent, 12);
    if now!(agent) { se(agent, "se_pzenigame_smash_s01"); se(agent, "se_common_down_soil_s"); }
}

// ===================================================================== NAIR: curled spin
pub unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { landing_on(agent); }
    at!(agent, 7);
    if now!(agent) { b(agent, 0, "bust", 9.0, 40, 90, 25, 4.8, 1.0, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY); }
    at!(agent, 12);
    if now!(agent) { b(agent, 0, "bust", 5.0, 50, 80, 20, 4.2, 0.8, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY); }
    at!(agent, 29);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.75); }   // less endlag
    at!(agent, 42);
    if now!(agent) { landing_off(agent); }
}
pub unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { fx_follow(agent, "sys_spin_wind", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.6); orange(agent); }
    at!(agent, 16);
    if now!(agent) { fx_follow(agent, "sys_spin_wind_s", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 0.6); orange(agent); }
}
pub unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
    at!(agent, 16);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}

// ===================================================================== FAIR: two-footed dropkick
pub unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { landing_on(agent); }
    at!(agent, 9);
    if now!(agent) {
        b(agent, 0, "footl", 11.0, 40, 95, 30, 3.2, 1.2, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        b(agent, 1, "footr", 11.0, 40, 95, 30, 3.2, 1.2, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    at!(agent, 13);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.75); }   // less endlag
    at!(agent, 32);
    if now!(agent) { landing_off(agent); }
}
pub unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5, 2, 0, 0, 0, 0.8, true, *EF_FLIP_YZ); orange(agent); }
    at!(agent, 9);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 5.0, 8.0, 0.0, 0.0, 0.0, 0.8); red(agent); }
}
pub unsafe extern "C" fn sound_attackairf(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); se(agent, "vc_pzenigame_attack05"); }
}

// ===================================================================== BAIR: head spike (swings the head down behind/under him)
pub unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { landing_on(agent); }
    at!(agent, 9);
    if now!(agent) {
        b(agent, 0, "head", 10.0, 270, 80, 25, 3.6, 1.3, *ATTACK_LR_CHECK_F, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    at!(agent, 12);
    if now!(agent) { clear(agent); }
    at!(agent, 30);
    if now!(agent) { landing_off(agent); }
}
pub unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    // the swing happens behind him: arc sits behind/below, then a downward follow-through line as the head spikes
    at!(agent, 8);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3.5, -3, 0, 52, 88, 0.8, true, *EF_FLIP_YZ); orange(agent); }
    at!(agent, 9);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 4.0, -3.2, 0.0, 0.0, 0.0, 0.9); red(agent); }
    at!(agent, 10);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 1.5, -1.5, 0, 52, 88, 0.6, true, *EF_FLIP_YZ); orange(agent);
        fx(agent, "sys_attack_line", "top", 0.0, 2.0, -2.6, 90.0, 0.0, 0.0, 0.7); orange(agent);
    }
}
pub unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); }
}

// ===================================================================== UAIR: tail flip overhead
pub unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 2);
    if now!(agent) { landing_on(agent); }
    at!(agent, 6);
    if now!(agent) {
        b(agent, 0, "tail2", 8.0, 80, 100, 30, 3.2, 1.0, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        b(agent, 1, "tail1", 8.0, 80, 100, 30, 2.8, 1.0, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 2, 0, "top", 8.0, 80, 100, 0, 30, 3.0, 0.0, 10.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    at!(agent, 12);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.75); }   // less endlag
    at!(agent, 24);
    if now!(agent) { landing_off(agent); }
}
pub unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 9, 0, 0, -52, -88, 0.8, true, *EF_FLIP_YZ); orange(agent); }
}
pub unsafe extern "C" fn sound_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { se(agent, "se_pzenigame_swing_m"); }
}

// ===================================================================== DAIR: double stomp (meteor on the clean hit)
pub unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { landing_on(agent); }
    at!(agent, 8);
    if now!(agent) {
        b(agent, 0, "footl", 12.0, 270, 85, 20, 3.0, 1.3, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        b(agent, 1, "footr", 12.0, 270, 85, 20, 3.0, 1.3, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    at!(agent, 12);
    if now!(agent) {
        b(agent, 0, "footl", 6.0, 361, 60, 20, 2.6, 0.9, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        b(agent, 1, "footr", 6.0, 361, 60, 20, 2.6, 0.9, *ATTACK_LR_CHECK_POS, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    at!(agent, 21);
    if now!(agent) { clear(agent); macros::FT_MOTION_RATE(agent, 0.75); }   // less endlag
    at!(agent, 44);
    if now!(agent) { landing_off(agent); }
}
pub unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, -2.0, 0.0, 0.0, 0.0, 0.0, 1.0); red(agent);
        fx_follow(agent, "sys_attack_speedline", "top", 0.0, 4.0, 0.0, 90.0, 0.0, 0.0, 0.6); orange(agent);
    }
}
pub unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_pzenigame_swing_l"); se(agent, "vc_pzenigame_attack06"); }
}

// ===================================================================== MOVEMENT: plain dust instead of Squirtle's water splashes
unsafe fn dust(agent: &mut L2CAgentBase, kind: &str, x: f32, z: f32, scale: f32) {
    macros::FOOT_EFFECT(agent, Hash40::new(kind), Hash40::new("top"), x, 0, z, 0, 0, 0, scale, 0, 0, 0, 0, 0, 0, false);
}
pub unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    if now!(agent) { dust(agent, "sys_dash_smoke", -2.0, 0.0, 0.7); }
    at!(agent, 12);
    if now!(agent) { dust(agent, "sys_run_smoke", 0.0, 0.0, 0.6); }
}
pub unsafe extern "C" fn effect_turndash(agent: &mut L2CAgentBase) {
    at!(agent, 3);
    if now!(agent) { dust(agent, "sys_turn_smoke", 0.0, 0.0, 0.6); }
    at!(agent, 12);
    if now!(agent) { dust(agent, "sys_run_smoke", 0.0, 0.0, 0.6); }
}
pub unsafe extern "C" fn effect_run(agent: &mut L2CAgentBase) {
    for f in [2.0f32, 10.0] {
        frame(agent.lua_state_agent, f);
        if now!(agent) { dust(agent, "sys_run_smoke", 0.0, -1.0, 0.5); }
    }
}
pub unsafe extern "C" fn effect_runbrake(agent: &mut L2CAgentBase) {
    if now!(agent) { dust(agent, "sys_turn_smoke", 0.0, 2.0, 0.6); }
}
pub unsafe extern "C" fn effect_turnrun(agent: &mut L2CAgentBase) {
    if now!(agent) { dust(agent, "sys_turn_smoke", 0.0, 0.0, 0.6); }
    at!(agent, 10);
    if now!(agent) { dust(agent, "sys_run_smoke", 0.0, 1.5, 0.5); }
}
