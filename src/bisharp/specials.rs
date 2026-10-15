//! Bisharp specials + the hand-made bits of the jab:
//!   Neutral B  FOCUS BLAST - Bullet Seed's hold charges (mod.rs counts it), the release (fire on 5) throws an orb that is
//!                            Razor Leaf's projectile re-skinned: pale-blue energy ball, damage/size grow with charge.
//!   Side B     METAL CLAW  - glint (8), draw (8-18), rush through (20-28, 24 forward), rising pop-up cut (29-31).
//!   Up B       PURSUIT     - coil, aimed dash (stick angle, 7-16), finishing cut (17-21), then helpless.
//! Hitboxes for Metal Claw / Pursuit are generated with the normals (specials_gen / specialhi_gen).
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash_script::*;
use crate::util::*;
use super::{steel, dark, blue, red};

macro_rules! at { ($a:ident, $f:expr) => { frame($a.lua_state_agent, $f as f32) } }
macro_rules! now { ($a:ident) => { macros::is_excute($a) } }

pub unsafe extern "C" fn effect_none(_agent: &mut L2CAgentBase) {}

pub unsafe extern "C" fn effect_smashcharge(agent: &mut L2CAgentBase) {
    for f in [0.0f32, 10.0, 20.0, 30.0, 40.0, 50.0] {
        frame(agent.lua_state_agent, f);
        if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6); steel(agent); }
    }
}

// ===================================================================== JAB FLURRY (18f loop: blade jabs on 1, 7, 13)
unsafe fn flurry_hit(agent: &mut L2CAgentBase) {
    hit_cap(agent, 0, 0, "top", 1.0, 361, 30, 0, 8, 5.0, 11.0, 8.0, 11.0, 20.0, 0.5, *ATTACK_LR_CHECK_F, "collision_attr_cutup",
            *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
}
unsafe fn flurry_gap(agent: &mut L2CAgentBase) {
    AttackModule::clear_all(agent.module_accessor);
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
}
pub unsafe extern "C" fn game_attack100(agent: &mut L2CAgentBase) {
    at!(agent, 1);
    if now!(agent) { flurry_hit(agent); }
    at!(agent, 2);
    if now!(agent) { flurry_gap(agent); }
    at!(agent, 7);
    if now!(agent) { flurry_hit(agent); }
    at!(agent, 8);
    if now!(agent) { flurry_gap(agent); }
    at!(agent, 13);
    if now!(agent) { flurry_hit(agent); }
    at!(agent, 14);
    if now!(agent) { flurry_gap(agent); }
}
pub unsafe extern "C" fn effect_attack100(agent: &mut L2CAgentBase) {
    for (f, y) in [(1.0f32, 10.0f32), (7.0, 12.0), (13.0, 9.0)] {
        frame(agent.lua_state_agent, f);
        if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, y, 8, 0, 0, 0, 0.9, true, *EF_FLIP_YZ); steel(agent); }
    }
}
pub unsafe extern "C" fn sound_attack100(agent: &mut L2CAgentBase) {
    for f in [0.0f32, 6.0, 12.0] {
        frame(agent.lua_state_agent, f);
        if now!(agent) { se(agent, "se_common_swing_06"); }
    }
}

// ===================================================================== NEUTRAL B: FOCUS BLAST
pub unsafe extern "C" fn effect_fbstart(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handl", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6); blue(agent); }
}
pub unsafe extern "C" fn sound_fbstart(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { se(agent, "se_common_smash_start_02"); }
}
pub unsafe extern "C" fn sound_fbhold(_agent: &mut L2CAgentBase) {}
pub unsafe extern "C" fn game_fbend(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) {
        let e = super::entry(agent.module_accessor);
        super::fb_fire(e);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_LEAFCUTTER, false, -1);
    }
}
pub unsafe extern "C" fn effect_fbend(agent: &mut L2CAgentBase) {
    at!(agent, 5);
    if now!(agent) {
        fx(agent, "sys_smash_flash", "top", 0.0, 12.6, 12.0, 0.0, 0.0, 0.0, 1.2); blue(agent);
        fx(agent, "sys_crown", "top", 0.0, 12.6, 12.0, 0.0, 90.0, 90.0, 0.3); blue(agent);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}
pub unsafe extern "C" fn sound_fbend(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { se(agent, "se_common_smashswing_03"); }
}
/// the orb (Razor Leaf's projectile): placed between his palms, damage and size grow with the charge
unsafe fn owner_charge(agent: &mut L2CAgentBase) -> (f32, *mut BattleObjectModuleAccessor) {
    let id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = smash::app::sv_battle_object::module_accessor(id);
    (super::fb_shot(super::entry(owner)), owner)
}
pub unsafe extern "C" fn game_orb(agent: &mut L2CAgentBase) {
    if now!(agent) {
        let (c, owner) = owner_charge(agent);
        let p = *PostureModule::pos(owner);
        let lr = PostureModule::lr(owner);
        PostureModule::set_pos(agent.module_accessor, &Vector3f { x: p.x + 13.0 * lr, y: p.y + 12.6, z: p.z });
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 + 12.0 * c, 45, 75, 0, (35.0 + 25.0 * c) as i32, 3.5 + 2.5 * c, 0.0, 0.0, 0.0,
            None, None, None, 1.0 + 0.4 * c, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false,
            *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
    }
}
pub unsafe extern "C" fn effect_orb(agent: &mut L2CAgentBase) {
    let (c, _) = owner_charge(agent);
    let mut f = 0.0f32;
    while f < 90.0 {
        frame(agent.lua_state_agent, f);
        if now!(agent) {
            fx_follow(agent, "sys_hit_aura_s", "top", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.25 + 0.2 * c); blue(agent);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
        f += 6.0;
    }
}

// ===================================================================== SIDE B: METAL CLAW
pub unsafe extern "C" fn effect_metalclaw(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { fx_follow(agent, "sys_smash_flash", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8); blue(agent); }
    at!(agent, 19);
    if now!(agent) { macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 20);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 9, 4, 0, 0, 0, 1.3, true, *EF_FLIP_YZ); dark(agent); }
    at!(agent, 24);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 6.0, 10.0, 0.0, 0.0, 0.0, 1.0); red(agent); }
    at!(agent, 28);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 18, 12, -70, 0, 0, 1.2, true, *EF_FLIP_YZ); steel(agent); }
}
pub unsafe extern "C" fn sound_metalclaw(agent: &mut L2CAgentBase) {
    at!(agent, 8);
    if now!(agent) { se(agent, "se_common_smash_start_02"); }
    at!(agent, 19);
    if now!(agent) { se(agent, "se_common_smashswing_03"); }
    at!(agent, 28);
    if now!(agent) { se(agent, "se_common_swing_06"); }
}

// ===================================================================== UP B: PURSUIT
pub unsafe extern "C" fn effect_pursuit(agent: &mut L2CAgentBase) {
    at!(agent, 4);
    if now!(agent) { fx_follow(agent, "sys_smash_flash_s", "handr", 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7); dark(agent); }
    at!(agent, 6);
    if now!(agent) { macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false); }
    at!(agent, 7);
    if now!(agent) { fx_follow(agent, "sys_attack_speedline", "top", 0.0, 10.0, 0.0, -60.0, 0.0, 0.0, 1.0); dark(agent); }
    at!(agent, 17);
    if now!(agent) { macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 20, 8, 30, 0, 0, 1.2, true, *EF_FLIP_YZ); steel(agent); }
    at!(agent, 18);
    if now!(agent) { fx(agent, "sys_attack_impact", "top", 0.0, 20.0, 6.0, 0.0, 0.0, 0.0, 0.9); red(agent); }
}
pub unsafe extern "C" fn sound_pursuit(agent: &mut L2CAgentBase) {
    at!(agent, 6);
    if now!(agent) { se(agent, "se_common_step_jump"); se(agent, "se_common_smashswing_03"); }
    at!(agent, 17);
    if now!(agent) { se(agent, "se_common_swing_06"); }
}
