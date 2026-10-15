//! Haxorus specials.
//!   Neutral B  DRAGON DANCE  - hold B to stomp out a war-dance; dance long enough and the roar gives a short speed
//!                              boost and powers up his NEXT attack only.
//!   Side B     OUTRAGE       - a short charge of autolinking claw rakes ending in a finisher swipe; never blows him back.
//!   Up B       GIGA IMPACT   - spinning launch that carries them up, then a belly-first dive that drags them into the crash.
//! The status code underneath is still Charizard's (that's what moves him); these scripts swap the hits/effects/sounds,
//! and `frame()` in mod.rs removes Flamethrower's breath and runs Dragon Dance / Outrage / the dive.
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

/// multi-hit / autolink hitbox (rehit = frames between hits)
unsafe fn hitm(agent: &mut L2CAgentBase, id: u64, bone: &str, dmg: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32, size: f32,
               y: f32, z: f32, y2: f32, z2: f32, rehit: i32, attr: &str, lvl: i32, snd: i32, region: i32) {
    macros::ATTACK(agent, id, 0, Hash40::new(bone), dmg, angle, kbg, fkb, bkb, size, 0.0, y, z, Some(0.0), Some(y2), Some(z2),
        0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, rehit, false, false, false, false, true,
        *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(attr), lvl, snd, region);
}

// ======================================================================= NEUTRAL B: DRAGON DANCE
pub unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0); purple(agent); }
    at!(agent, 11);
    if now!(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        fx(agent, "sys_crown", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7); purple(agent);
    }
}
pub unsafe extern "C" fn sound_specialnstart(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "vc_plizardon_appeal01"); }
    at!(agent, 11);
    if now!(agent) { se(agent, "se_plizardon_landing01"); }
}
pub unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    // one knee-up stomp per foot each loop: dust + a purple pulse
    at!(agent, 7);
    if now!(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        fx_follow(agent, "sys_hit_aura_s", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2); purple(agent);
    }
    at!(agent, 14);
    if now!(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        fx_follow(agent, "sys_hit_aura_s", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2); purple(agent);
    }
}
pub unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    at!(agent, 7);
    if now!(agent) { se(agent, "se_plizardon_step_left_m"); }
    at!(agent, 14);
    if now!(agent) { se(agent, "se_plizardon_step_right_m"); }
}
pub unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    at!(agent, 11);
    if now!(agent) {
        fx_follow(agent, "plizardon_roar", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0); purple(agent);
        fx(agent, "sys_crown", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.2); purple(agent);
        fx_follow(agent, "sys_smash_flash", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.6); purple(agent);
    }
}
pub unsafe extern "C" fn sound_specialnend(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { se(agent, "vc_plizardon_attack07"); se(agent, "se_plizardon_smash_h02"); }
}
/// air version keeps Charizard's air Flamethrower poses - just strip the fire
pub unsafe extern "C" fn effect_none(_agent: &mut L2CAgentBase) {}
pub unsafe extern "C" fn sound_specialairnend(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { se(agent, "vc_plizardon_attack05"); }
}

// ======================================================================= SIDE B: OUTRAGE
pub unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { fx_follow(agent, "plizardon_roar", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9); tint(agent, 1.0, 0.25, 0.35); }
    at!(agent, 20);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false); }
}
pub unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "vc_plizardon_attack07"); }
}
pub unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    if now!(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        outrage_claws(agent);
    }
}
/// Outrage's rampage: autolink claw rakes (367 drags them along with him). mod.rs ends the rush in the finisher
/// after 4 rakes, or after a short run if he whiffs.
unsafe fn outrage_claws(agent: &mut L2CAgentBase) {
    hitm(agent, 0, "top", 2.0, 367, 100, 45, 0, 6.0, 9.5, 4.0, 9.5, 15.0, 4, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    hitm(agent, 1, "top", 2.0, 367, 100, 45, 0, 5.0, 4.0, 3.0, 4.0, 12.0, 4, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
}
pub unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 2, 9, 12, 20, -10, 25, 1.1, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 16);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), -2, 9, 12, 20, 10, -25, 1.1, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); }
    at!(agent, 16);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); }
}
pub unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    // whiffed rampage ends in the two-claw overhead smash
    at!(agent, 11);
    if now!(agent) {
        // finisher: weaker than Flare Blitz, and no recoil on him
        hitm(agent, 0, "top", 7.0, 45, 95, 0, 60, 7.0, 10.0, 14.0, 3.0, 6.0, 0, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 14);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 0, 10, 10, -60, 0, 0, 1.4, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
    }
    at!(agent, 13);
    if now!(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        fx(agent, "sys_attack_impact", "top", 0.0, 3.0, 5.0, 0.0, 0.0, 0.0, 1.3);
    }
}
pub unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    at!(agent, 10);
    if now!(agent) { se(agent, "se_plizardon_swing_l"); se(agent, "vc_plizardon_attack06"); }
    at!(agent, 13);
    if now!(agent) { se(agent, "se_plizardon_landing02"); }
}
pub unsafe extern "C" fn effect_specialsblown(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { fx(agent, "sys_hit_aura_s", "bust", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5); tint(agent, 1.0, 0.3, 0.6); }
}
// --- in the air: the same claw rampage (keeps air side B's travel/recovery)
pub unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    if now!(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        JostleModule::set_status(agent.module_accessor, false);
        outrage_claws(agent);
    }
    wait(agent.lua_state_agent, 22.0);
    if now!(agent) { notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); }
}
pub unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    at!(agent, 9);
    if now!(agent) {
        hitm(agent, 0, "top", 7.0, 45, 95, 0, 60, 7.0, 10.0, 14.0, 3.0, 6.0, 0, "collision_attr_cutup", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    at!(agent, 12);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_specialairsend(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 0, 10, 10, -60, 0, 0, 1.4, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
    }
}
pub unsafe extern "C" fn sound_specialairsend(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_plizardon_swing_l"); se(agent, "vc_plizardon_attack06"); }
}
pub unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), 2, 9, 12, 20, -10, 25, 1.1, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
    }
    at!(agent, 16);
    if now!(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("plizardon_scratch"), Hash40::new("plizardon_scratch"), Hash40::new("top"), -2, 9, 12, 20, 10, -25, 1.1, true, *EF_FLIP_YZ);
        tint(agent, 1.0, 0.3, 0.6);
    }
}
pub unsafe extern "C" fn sound_specialairs(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); se(agent, "vc_plizardon_attack07"); }
    at!(agent, 16);
    if now!(agent) { se(agent, "se_plizardon_swing_m"); }
}
// --- the rage burst (Flare Blitz's explosion article)
pub unsafe extern "C" fn game_explosion(agent: &mut L2CAgentBase) {
    if now!(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 70, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
}
pub unsafe extern "C" fn effect_explosion(agent: &mut L2CAgentBase) {
    if now!(agent) {
        fx(agent, "sys_attack_impact", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.2); tint(agent, 1.0, 0.3, 0.6);
        fx(agent, "sys_crown", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 90.0, 1.2); tint(agent, 1.0, 0.3, 0.6);
        fx(agent, "sys_hit_cut", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.4);
    }
}

// ======================================================================= UP B: GIGA IMPACT
unsafe extern "C" fn giga_game(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0); }
    at!(agent, 9);
    if now!(agent) {
        // launch: scoops them up with him (autolink, so they stay on him)
        hitm(agent, 0, "top", 4.0, 367, 100, 60, 0, 7.0, 4.0, 4.0, 20.0, 2.0, 0, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 11);
    if now!(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        AttackModule::clear_all(agent.module_accessor);
    }
    at!(agent, 13);
    if now!(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        // spinning ascent: carries them all the way to the top, where the dive takes over
        hitm(agent, 0, "top", 1.5, 367, 100, 40, 0, 6.5, 22.0, 2.0, 4.0, 1.0, 4, "collision_attr_rush", *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 21);
    if now!(agent) { notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); }
    at!(agent, 46);
    if now!(agent) { WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS); }
}
pub unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) { giga_game(agent) }
pub unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) { giga_game(agent) }
unsafe extern "C" fn giga_effect(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 9);
    if now!(agent) {
        fx(agent, "sys_smash_flash", "top", 0.0, 10.0, 2.0, 0.0, 0.0, 0.0, 1.2); tint(agent, 1.0, 0.5, 0.9);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    at!(agent, 12);
    if now!(agent) {
        fx_follow(agent, "sys_spin_wind", "top", 0.0, 10.0, 1.0, 0.0, 0.0, 0.0, 1.5); tint(agent, 1.0, 0.5, 0.9);
        fx_follow(agent, "sys_attack_speedline", "top", 0.0, 4.0, 1.0, -90.0, 0.0, 0.0, 1.4); tint(agent, 1.0, 0.5, 0.9);
    }
    at!(agent, 38);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "head", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.4); purple(agent); }
}
pub unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) { giga_effect(agent) }
pub unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) { giga_effect(agent) }
unsafe extern "C" fn giga_sound(agent: &mut L2CAgentBase) {
    at!(agent, 2);
    if now!(agent) { se(agent, "se_plizardon_squat"); }
    at!(agent, 9);
    if now!(agent) { se(agent, "se_plizardon_special_h01"); se(agent, "vc_plizardon_attack05"); }
    at!(agent, 38);
    if now!(agent) { se(agent, "vc_plizardon_attack07"); }
}
pub unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) { giga_sound(agent) }
pub unsafe extern "C" fn sound_specialairhi(agent: &mut L2CAgentBase) { giga_sound(agent) }
// the helpless fall after it is the belly-first DIVE: drags whoever he carried up back down into the crash
// (Bowser Bomb style), and catches anyone underneath on the way
pub unsafe extern "C" fn game_fallspecial(agent: &mut L2CAgentBase) {
    if now!(agent) {
        hitm(agent, 0, "top", 1.5, 367, 100, 40, 0, 7.5, 3.0, 2.0, 22.0, 2.0, 4, "collision_attr_normal", *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}
pub unsafe extern "C" fn effect_fallspecial(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { fx_follow(agent, "sys_attack_speedline", "top", 0.0, 4.0, 4.0, 90.0, 0.0, 0.0, 1.3); tint(agent, 1.0, 0.5, 0.9); }
}
// crash landing: crater shockwave
pub unsafe extern "C" fn game_landingfallspecial(agent: &mut L2CAgentBase) {
    if now!(agent) {
        // the slam: launches whatever he dragged down
        hitm(agent, 0, "top", 9.0, 65, 85, 0, 62, 9.0, 4.0, 9.0, 4.0, -7.0, 0, "collision_attr_normal", *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    at!(agent, 3);
    if now!(agent) { AttackModule::clear_all(agent.module_accessor); }
}
pub unsafe extern "C" fn effect_landingfallspecial(agent: &mut L2CAgentBase) {
    if now!(agent) {
        fx(agent, "sys_crown", "top", 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 1.3); tint(agent, 1.0, 0.5, 0.9);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        fx(agent, "sys_attack_impact", "top", 0.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.6);
    }
}
pub unsafe extern "C" fn sound_landingfallspecial(agent: &mut L2CAgentBase) {
    if now!(agent) { se(agent, "se_plizardon_landing02"); se(agent, "se_plizardon_smash_l01"); }
}
pub unsafe extern "C" fn expression_landingfallspecial(agent: &mut L2CAgentBase) {
    if now!(agent) { macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M); macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0); }
}
