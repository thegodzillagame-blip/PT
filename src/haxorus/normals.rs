//! Haxorus' custom normals. Frame numbers are the animation frames of the v47 anims
//! (hitbox positions measured from those anims, model origin space: y up, z forward).
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

// ===================================================================== JAB 1/2: Scratch (claw rakes)
pub unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) {
        hit(agent, 0, 0, "top", 3.0, 361, 30, 0, 22, 3.2, 0.0, 7.5, 6.0, 1.4, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 3.0, 361, 25, 0, 22, 3.6, 0.0, 7.0, 11.0, 1.4, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if now!(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    at!(agent, 14);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO); }
}
pub unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), -2, 9, 7, 20, 10, -25, 0.9, true, *EF_FLIP_YZ);
        purple(agent);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}
pub unsafe extern "C" fn sound_attack11(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { se(agent, "se_plizardon_swing_s"); }
}

pub unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) {
        hit(agent, 0, 0, "top", 3.0, 361, 30, 0, 22, 3.2, 0.0, 7.5, 5.5, 1.4, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 3.0, 361, 25, 0, 22, 3.6, 0.0, 7.0, 10.5, 1.4, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if now!(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    at!(agent, 15);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO); }
}
pub unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 2, 9, 7, 20, -10, 25, 0.9, true, *EF_FLIP_YZ);
        purple(agent);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}
pub unsafe extern "C" fn sound_attack12(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { se(agent, "se_plizardon_swing_s"); }
}

// ===================================================================== JAB 3: X-Scissor (both claws cross down)
pub unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) {
        hit_cap(agent, 0, 0, "top", 6.0, 50, 120, 0, 60, 4.8, 15.0, 14.0, 3.5, 15.5, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "top", 6.0, 50, 120, 0, 60, 4.0, 0.0, 9.0, 7.0, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 0, 10, 12, 0, 0, 45, 1.2, true, *EF_FLIP_YZ);
        purple(agent);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 0, 10, 12, 0, 0, -45, 1.2, true, *EF_FLIP_YZ);
        purple(agent);
    }
    at!(agent, 10);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 9.0, 14.0, 0.0, 0.0, 0.0, 1.1); purple(agent); }
}
pub unsafe extern "C" fn sound_attack13(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); se(agent, "vc_plizardon_attack02"); }
}

// ===================================================================== DASH ATTACK: Take Down (shoulder tackle)
pub unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) {
        hit(agent, 0, 0, "trans", 13.0, 45, 60, 0, 85, 4.8, 0.0, 10.5, 9.0, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        hit(agent, 1, 0, "trans", 13.0, 50, 60, 0, 85, 4.2, 0.0, 10.0, 4.5, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 11);
    if now!(agent) {
        hit(agent, 0, 0, "trans", 9.0, 60, 40, 0, 85, 4.5, 0.0, 10.5, 9.0, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        hit(agent, 1, 0, "trans", 9.0, 65, 40, 0, 85, 4.0, 0.0, 10.0, 4.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 23);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 7);
    if now!(agent) {
        fx_follow(agent, "sys_attack_speedline", "trans", 0.0, 10.5, 2.0, 0.0, 0.0, 0.0, 1.2);
        tint(agent, 1.0, 0.6, 0.2);
        fx(agent, "sys_attack_impact", "trans", 0.0, 10.5, 9.0, 0.0, 0.0, 0.0, 1.0);
    }
    at!(agent, 23);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("trans"), 0, 0, 6, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false); }
}
pub unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "se_plizardon_attackdash"); se(agent, "vc_plizardon_attack05"); }
    at!(agent, 23);
    if now!(agent) { se(agent, "se_plizardon_landing02"); }
}

// ===================================================================== F-TILT: Crunch - lunging bite (Piranha Plant style), fangs flash over his face
pub unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) {
        // the snap: jaws/tusk tips are the sweetspot, the neck behind them sours
        hit(agent, 0, 0, "top", 13.0, 40, 92, 0, 48, 4.0, 0.0, 12.8, 19.0, 1.3, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        hit_cap(agent, 1, 0, "top", 10.0, 45, 85, 0, 45, 3.6, 13.8, 11.0, 13.2, 16.0, 1.1, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
    }
    at!(agent, 12);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        fx(agent, "sys_attack_line", "top", 0.0, 13.5, 8.0, 0.0, 0.0, 0.0, 1.0);
        purple(agent);
    }
    at!(agent, 8);
    if now!(agent) {
        // the fangs: a ghostly upper and lower jaw that clamp shut over his real ones (anime Crunch / Fire Fang)
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 16.0, 17.0, 0, 0, 200, 0.9, true, *EF_FLIP_YZ);
        purple(agent);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9.5, 17.0, 0, 0, -20, 0.9, true, *EF_FLIP_YZ);
        purple(agent);
        macros::LAST_EFFECT_SET_RATE(agent, 1.6);
        // Piranha Plant's real chomp effect - only shows if ef_packun.eff is transplanted into effect/fighter/plizardon/transplant/
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), 0, 12.8, 18.0, 0, -90, 0, 1.4, true, *EF_FLIP_YZ);
        purple(agent);
    }
    at!(agent, 10);
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 12.8, 19.5, 0.0, 0.0, 0.0, 1.3);
        purple(agent);
        fx(agent, "sys_hit_cut", "top", 0.0, 12.8, 19.5, 0.0, 0.0, 0.0, 0.8);
    }
}
pub unsafe extern "C" fn sound_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); se(agent, "vc_plizardon_attack02"); }
    at!(agent, 10);
    if now!(agent) { se(agent, "se_common_swing_04"); }
}
pub unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0); }
}

// ===================================================================== U-TILT: tail flip (Yoshi style) - tail whips up from behind, over his head
pub unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) {
        hit(agent, 0, 0, "tail4", 8.0, 95, 75, 0, 62, 5.0, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 1, 0, "tail3", 8.0, 95, 75, 0, 62, 4.5, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 2, 0, "tail2", 8.0, 95, 75, 0, 62, 4.0, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    at!(agent, 15);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 7);
    if now!(agent) {
        // Yoshi's own up-tilt arc, scaled up for the bigger tail
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 14, -2, 0, 52, 88, 1.6, true, *EF_FLIP_YZ);
        purple(agent);
    }
}
pub unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "se_plizardon_tailswing"); }
}

// ===================================================================== U-SMASH: Guillotine (tusk axe chop, front -> up -> back)
pub unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 11);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD); }
    at!(agent, 15);
    if now!(agent) {
        hit(agent, 0, 0, "top", 16.0, 85, 95, 0, 38, 5.5, 0.0, 14.0, 9.5, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "top", 16.0, 85, 95, 0, 38, 5.0, 0.0, 9.0, 5.0, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    at!(agent, 16);
    if now!(agent) {
        hit(agent, 0, 0, "top", 17.0, 88, 95, 0, 38, 6.0, 0.0, 24.0, 4.5, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        hit(agent, 1, 0, "top", 17.0, 88, 95, 0, 38, 5.0, 0.0, 18.5, 2.0, 1.2, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    at!(agent, 17);
    if now!(agent) {
        hit(agent, 0, 0, "top", 14.0, 95, 90, 0, 35, 6.0, 0.0, 22.0, -1.5, 1.1, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    at!(agent, 20);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ); }
    at!(agent, 15);
    if now!(agent) {
        // Charizard's own up-smash sweep (front -> overhead -> behind), the same path as the tusk chop
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_smash_arc"), Hash40::new("plizardon_smash_arc"), Hash40::new("top"), -1, 18, 1, -178, -20, 98, 1.35, true, *EF_FLIP_YZ);
        tint(agent, 0.9, 0.85, 1.25);
    }
    at!(agent, 16);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 24.0, 4.5, 0.0, 0.0, 0.0, 1.3); }
    at!(agent, 19);
    if now!(agent) { macros::EFFECT_DETACH_KIND(agent, Hash40::new("plizardon_smash_arc"), -1); }
}
pub unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    at!(agent, 12);
    if now!(agent) { macros::STOP_SE(agent, Hash40::new("se_common_smash_start")); }
    at!(agent, 14);
    if now!(agent) { se(agent, "se_plizardon_smash_h01"); se(agent, "vc_plizardon_attack06"); }
}

// ===================================================================== D-SMASH: Dragon Tail (low 360 tail sweep, then a stomp)
pub unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD); }
    at!(agent, 14);
    if now!(agent) {
        // the tail itself sweeps all the way round: ride its bones, launch away from him, low and horizontal
        hit(agent, 0, 0, "tail4", 15.0, 30, 88, 0, 38, 5.0, 0.0, 0.0, 0.0, 1.1, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 1, 0, "tail3", 15.0, 30, 88, 0, 38, 4.5, 0.0, 0.0, 0.0, 1.1, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        hit(agent, 2, 0, "tail2", 13.0, 35, 85, 0, 35, 4.0, 0.0, 0.0, 0.0, 1.1, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    at!(agent, 26);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
    at!(agent, 29);
    if now!(agent) {
        // the stomp that ends the spin: small pop-up shockwave around his feet and tail
        hit(agent, 0, 1, "top", 6.0, 80, 60, 0, 55, 6.0, 0.0, 2.5, 0.0, 0.8, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        hit(agent, 1, 1, "top", 6.0, 80, 60, 0, 55, 5.0, 0.0, 2.5, -13.0, 0.8, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ); }
    at!(agent, 13);
    if now!(agent) {
        fx_follow(agent, "sys_spin_wind", "top", 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.6);
        purple(agent);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 18);
    if now!(agent) { fx_follow(agent, "sys_attack_arc_d", "top", 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.5); purple(agent); }
    at!(agent, 29);
    if now!(agent) {
        fx(agent, "sys_crown", "top", 0.0, 0.0, -10.0, 0.0, 0.0, 0.0, 1.0);
        purple(agent);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -12, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::STOP_SE(agent, Hash40::new("se_common_smash_start")); }
    at!(agent, 13);
    if now!(agent) { se(agent, "se_plizardon_tailswing"); se(agent, "vc_plizardon_attack07"); }
    at!(agent, 29);
    if now!(agent) { se(agent, "se_plizardon_smash_l01"); }
}
pub unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    at!(agent, 14);
    if now!(agent) { macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0); }
    at!(agent, 29);
    if now!(agent) { macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S); }
}

// ===================================================================== NAIR: Roar (sound-wave burst, then a lingering ring)
pub unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
    at!(agent, 8);
    if now!(agent) {
        // one round burst centred on his chest: hits the same all the way around him, launches up and away
        hit(agent, 0, 0, "top", 11.0, 55, 85, 0, 45, 10.5, 0.0, 12.0, 0.0, 1.2, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 12);
    if now!(agent) {
        hit(agent, 0, 0, "top", 6.0, 70, 70, 0, 35, 12.0, 0.0, 12.0, 0.0, 0.8, *ATTACK_LR_CHECK_POS, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 21);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
    at!(agent, 40);
    if now!(agent) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
}
pub unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) {
        fx_follow(agent, "plizardon_roar", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        purple(agent);
    }
    // rings placed on his origin (not the twisting chest bone) and turned flat to the screen
    at!(agent, 8);
    if now!(agent) { fx(agent, "sys_crown", "top", 0.0, 13.0, 0.0, 0.0, 0.0, 90.0, 0.8); purple(agent); }
    at!(agent, 12);
    if now!(agent) { fx(agent, "sys_crown", "top", 0.0, 13.0, 0.0, 0.0, 0.0, 90.0, 1.1); purple(agent); }
    at!(agent, 16);
    if now!(agent) { fx(agent, "sys_crown", "top", 0.0, 13.0, 0.0, 0.0, 0.0, 90.0, 1.4); purple(agent); }
}
pub unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "vc_plizardon_attack07"); se(agent, "se_plizardon_attackair_l01"); }
}

// ===================================================================== BAIR: double dropkick backwards (Bowser style)
pub unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
    at!(agent, 8);
    if now!(agent) {
        hit(agent, 0, 0, "top", 15.0, 361, 92, 0, 40, 4.8, 0.0, 13.5, -12.0, 1.2, *ATTACK_LR_CHECK_B, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        hit(agent, 1, 0, "top", 15.0, 361, 92, 0, 40, 4.8, 0.0, 5.5, -12.0, 1.2, *ATTACK_LR_CHECK_B, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        hit(agent, 2, 0, "top", 12.0, 361, 90, 0, 40, 4.0, 0.0, 9.5, -6.5, 1.1, *ATTACK_LR_CHECK_B, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    at!(agent, 10);
    if now!(agent) {
        hit(agent, 0, 0, "top", 11.0, 361, 85, 0, 30, 4.5, 0.0, 13.5, -12.0, 1.0, *ATTACK_LR_CHECK_B, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        hit(agent, 1, 0, "top", 11.0, 361, 85, 0, 30, 4.5, 0.0, 5.5, -12.0, 1.0, *ATTACK_LR_CHECK_B, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    at!(agent, 13);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
    at!(agent, 30);
    if now!(agent) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
}
pub unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) {
        // twin speed streaks along both legs as they shoot out
        fx(agent, "sys_attack_line", "top", 0.0, 13.5, -3.0, 0.0, 180.0, 0.0, 1.6);
        tint(agent, 1.0, 0.55, 0.15);
        fx(agent, "sys_attack_line", "top", 0.0, 5.5, -3.0, 0.0, 180.0, 0.0, 1.6);
        tint(agent, 1.0, 0.55, 0.15);
    }
    at!(agent, 8);
    if now!(agent) {
        // double impact burst at the soles + a shockwave ring blasting backwards
        fx(agent, "sys_attack_impact", "top", 0.0, 13.8, -13.0, 0.0, 0.0, 0.0, 2.0);
        fx(agent, "sys_attack_impact", "top", 0.0, 5.4, -13.0, 0.0, 0.0, 0.0, 2.0);
        fx(agent, "sys_crown", "top", 0.0, 9.5, -13.5, 0.0, 0.0, 90.0, 1.0);
        tint(agent, 1.0, 0.7, 0.3);
        fx(agent, "sys_smash_flash_s", "top", 0.0, 9.5, -13.0, 0.0, 0.0, 0.0, 1.6);
    }
}
pub unsafe extern "C" fn expression_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S); macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0); }
}
pub unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_plizardon_attackair_b01"); }
}

// ===================================================================== UAIR: tusk swipe overhead (front -> up -> back)
pub unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
    at!(agent, 10);
    if now!(agent) {
        hit(agent, 0, 0, "top", 13.0, 85, 105, 0, 30, 5.5, 0.0, 18.0, 3.0, 1.1, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    at!(agent, 12);
    if now!(agent) {
        hit(agent, 0, 0, "top", 13.0, 85, 105, 0, 30, 6.5, 0.0, 26.0, -2.5, 1.1, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    at!(agent, 14);
    if now!(agent) {
        hit(agent, 0, 0, "top", 10.0, 100, 100, 0, 30, 5.5, 0.0, 19.5, -10.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    at!(agent, 15);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
    at!(agent, 38);
    if now!(agent) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
}
pub unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_smash_arc"), Hash40::new("plizardon_smash_arc"), Hash40::new("top"), 0, 20, -2, 0, -90, -90, 1.0, true, *EF_FLIP_YZ);
        tint(agent, 0.9, 0.9, 1.2);
    }
}
pub unsafe extern "C" fn sound_attackairhi(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) { se(agent, "se_plizardon_attackair_h01"); }
}

// ===================================================================== FAIR: Charizard's claw swipe, faster recovery
pub unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
    at!(agent, 8);
    if now!(agent) {
        hit(agent, 0, 0, "arml", 12.0, 361, 92, 0, 35, 4.6, 6.0, 0.0, 1.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        hit(agent, 1, 0, "arml", 12.0, 361, 92, 0, 35, 3.8, 0.0, 0.0, 1.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        hit(agent, 2, 0, "shoulderl", 12.0, 361, 92, 0, 35, 3.0, 0.0, 0.0, 0.0, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if now!(agent) {
        hit(agent, 0, 0, "arml", 13.0, 34, 97, 0, 32, 4.8, 6.0, 0.0, 1.5, 1.0, *ATTACK_LR_CHECK_F, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if now!(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::FT_MOTION_RATE(agent, 0.6);     // recovery plays 1.67x faster (~16 fewer frames of endlag)
    }
    at!(agent, 28);
    if now!(agent) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING); }
}
