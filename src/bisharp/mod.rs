//! Bisharp (Ivysaur's slot c01, on Captain Falcon's skeleton). Ivysaur's statuses still run underneath; these scripts
//! and the per-frame hook turn them into Bisharp's moveset (see BISHARP_FRAME_DATA.txt in the mod folder).
//!   normals.rs  - GENERATED from the animations (work/tools/bis_codegen.py): every hitbox follows the blade
//!   specials.rs - Focus Blast (Bullet Seed's hold -> a charged orb on Razor Leaf's projectile), Metal Claw, Pursuit
//!   grab.rs     - grabs (Falcon range, no vine), throws
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smashline::{Priority::*, *};
use smash_script::*;
use crate::util::*;

mod normals;
mod specials;
mod grab;

static mut FB_CHARGE: [i32; 8] = [0; 8];     // Focus Blast charge frames
static mut FB_SHOT: [f32; 8] = [0.0; 8];     // charge of the orb just thrown (read by the projectile)
static mut MOVED: [f32; 8] = [0.0; 8];       // root motion already applied this special
static mut PURSUIT_DIR: [(f32, f32); 8] = [(0.0, 0.0); 8];
static mut PURSUIT_SET: [bool; 8] = [false; 8];
pub(crate) const FB_FULL: i32 = 60;

/// Metal Claw travel per frame (from the animation; the anims carry no root motion, the code moves him)
const MC_Z: [f32; 32] = [0.0, 0.1, 0.2, 0.2, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 1.1,
    3.4, 7.4, 12.8, 15.0, 17.2, 19.4, 21.6, 23.8, 24.7, 25.2, 25.4, 25.4];
/// Focus Blast release: the step into the palm thrust
const FB_Z: [f32; 6] = [0.0, -0.2, -0.3, -0.3, 0.8, 4.3];
/// Pursuit: dash speed per frame along the aimed direction (frames 7-21)
const PURSUIT_SPEED: [f32; 15] = [4.0, 4.4, 4.4, 4.4, 4.2, 4.0, 3.8, 3.6, 3.4, 3.0, 1.6, 1.0, 0.6, 0.3, 0.1];
const PURSUIT_DEFAULT_DEG: f32 = 60.0;

const IVY_FX: [&str; 11] = ["pfushigisou_atk_hi4", "pfushigisou_atk_impact", "pfushigisou_attack_100", "pfushigisou_h_cutter",
    "pfushigisou_h_cutter_hold", "pfushigisou_leaf", "pfushigisou_leaf_atk", "pfushigisou_slip_arc", "pfushigisou_tanemg",
    "pfushigisou_throw_b_arc", "pfushigisou_vine"];

pub(crate) unsafe fn entry(boma: *mut BattleObjectModuleAccessor) -> usize {
    (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7)
}
pub(crate) unsafe fn fb_charge(e: usize) -> f32 { (FB_CHARGE[e] as f32 / FB_FULL as f32).min(1.0) }
pub(crate) unsafe fn fb_fire(e: usize) { FB_SHOT[e] = fb_charge(e); FB_CHARGE[e] = 0; }
pub(crate) unsafe fn fb_shot(e: usize) -> f32 { FB_SHOT[e] }

/// capsule hitbox relative to top (generated code uses this)
pub(crate) unsafe fn cap(agent: &mut L2CAgentBase, id: u64, dmg: f32, angle: u64, kbg: i32, bkb: i32, size: f32,
                         y: f32, z: f32, y2: f32, z2: f32, hitlag: f32, lr: i32, attr: &str, lvl: i32, snd: i32, region: i32) {
    hit_cap(agent, id, 0, "top", dmg, angle, kbg, 0, bkb, size, y, z, y2, z2, hitlag, lr, attr, lvl, snd, region);
}
pub(crate) unsafe fn steel(agent: &mut L2CAgentBase) { tint(agent, 0.85, 0.88, 0.95); }
pub(crate) unsafe fn dark(agent: &mut L2CAgentBase) { tint(agent, 0.45, 0.1, 0.6); }
pub(crate) unsafe fn blue(agent: &mut L2CAgentBase) { tint(agent, 0.6, 0.75, 1.0); }
pub(crate) unsafe fn red(agent: &mut L2CAgentBase) { tint(agent, 1.0, 0.18, 0.12); }

unsafe fn glow(boma: *mut BattleObjectModuleAccessor, name: &str, bone: &str, scale: f32, r: f32, g: f32, b: f32) {
    let z = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
    let h = EffectModule::req_follow(boma, Hash40::new(name), Hash40::new(bone), &z, &z, scale, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(boma, h as u32, r, g, b);
}
/// move him along the animation's own travel table (the special anims carry no root motion)
unsafe fn follow_table(boma: *mut BattleObjectModuleAccessor, e: usize, table: &[f32]) {
    let f = MotionModule::frame(boma);
    if f < 1.0 { MOVED[e] = 0.0; }
    let i = (f as usize).min(table.len() - 1);
    let d = table[i] - MOVED[e];
    if d.abs() > 0.001 {
        let lr = PostureModule::lr(boma);
        PostureModule::add_pos(boma, &Vector3f { x: d * lr, y: 0.0, z: 0.0 });
        MOVED[e] = table[i];
    }
}

unsafe extern "C" fn bisharp_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let e = entry(boma);
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_DEAD || status == *FIGHTER_STATUS_KIND_REBIRTH { FB_CHARGE[e] = 0; PURSUIT_SET[e] = false; return; }
    // no Ivysaur leftovers: seeds, vines and leaf effects (the Razor Leaf projectile is reused for Focus Blast)
    for name in IVY_FX { EffectModule::kill_kind(boma, Hash40::new(name), false, true); }
    ArticleModule::remove_exist(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(boma, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_VINE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    // ---- Focus Blast: charge while B is held (Bullet Seed's loop), the orb grows with it
    if status == *FIGHTER_STATUS_KIND_SPECIAL_N && MotionModule::frame(boma) < 1.0 { FB_CHARGE[e] = 0; }
    if status == *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_LOOP {
        if FB_CHARGE[e] < FB_FULL {
            FB_CHARGE[e] += 1;
            if FB_CHARGE[e] == FB_FULL { glow(boma, "sys_smash_flash", "handl", 1.4, 0.6, 0.75, 1.0); }
        }
        let c = fb_charge(e);
        let every = if FB_CHARGE[e] >= FB_FULL { 5 } else { 10 - (5.0 * c) as i32 };
        if FB_CHARGE[e] % every == 0 { glow(boma, "sys_smash_flash_s", "handl", 0.5 + 0.8 * c, 0.6, 0.75, 1.0); }
    }
    if status == *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_END { follow_table(boma, e, &FB_Z); }

    // ---- Metal Claw: the rush travel comes from the animation table
    if status == *FIGHTER_STATUS_KIND_SPECIAL_S { follow_table(boma, e, &MC_Z); }

    // ---- Pursuit: aimed dash (stick angle, default up-forward), then the cut, then helpless
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        let f = MotionModule::frame(boma);
        if f < 1.0 { PURSUIT_SET[e] = false; }
        if f >= 7.0 && !PURSUIT_SET[e] {
            PURSUIT_SET[e] = true;
            let lr = PostureModule::lr(boma);
            let sx = ControlModule::get_stick_x(boma) * lr;
            let sy = ControlModule::get_stick_y(boma);
            let mut deg = PURSUIT_DEFAULT_DEG;
            if sx * sx + sy * sy > 0.25 { deg = sy.atan2(sx).to_degrees(); }
            let deg = deg.clamp(15.0, 110.0).to_radians();
            PURSUIT_DIR[e] = (deg.cos(), deg.sin());
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
        if PURSUIT_SET[e] {
            let i = f as usize;
            if i >= 7 && i <= 21 {
                KineticModule::clear_speed_all(boma);
                let v = PURSUIT_SPEED[i - 7];
                let lr = PostureModule::lr(boma);
                let (dx, dy) = PURSUIT_DIR[e];
                PostureModule::add_pos(boma, &Vector3f { x: dx * v * lr, y: dy * v, z: 0.0 });
            }
            if f >= 22.0 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
            }
        }
    } else if status != *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        PURSUIT_SET[e] = false;
    }
}

pub fn install() {
    use normals::*;
    use specials::*;
    use grab::*;
    Agent::new("pfushigisou")
        .set_costume(crate::SLOTS.to_vec())
        .on_line(Main, bisharp_frame)
        // jab: 1, 2, blade flurry, finisher
        .game_acmd("game_attack11", game_attack11, Low).effect_acmd("effect_attack11", effect_attack11, Low).sound_acmd("sound_attack11", sound_attack11, Low)
        .game_acmd("game_attack12", game_attack12, Low).effect_acmd("effect_attack12", effect_attack12, Low).sound_acmd("sound_attack12", sound_attack12, Low)
        .game_acmd("game_attack100", game_attack100, Low).effect_acmd("effect_attack100", effect_attack100, Low).sound_acmd("sound_attack100", sound_attack100, Low)
        .effect_acmd("effect_attack100start", effect_none, Low).sound_acmd("sound_attack100start", effect_none, Low)
        .game_acmd("game_attack100end", game_attackend, Low).effect_acmd("effect_attack100end", effect_attackend, Low).sound_acmd("sound_attack100end", sound_attackend, Low)
        // tilts / dash
        .game_acmd("game_attacks3", game_attacks3, Low).effect_acmd("effect_attacks3", effect_attacks3, Low).sound_acmd("sound_attacks3", sound_attacks3, Low)
        .game_acmd("game_attackhi3", game_attackhi3, Low).effect_acmd("effect_attackhi3", effect_attackhi3, Low).sound_acmd("sound_attackhi3", sound_attackhi3, Low)
        .game_acmd("game_attacklw3", game_attacklw3, Low).effect_acmd("effect_attacklw3", effect_attacklw3, Low).sound_acmd("sound_attacklw3", sound_attacklw3, Low)
        .game_acmd("game_attackdash", game_attackdash, Low).effect_acmd("effect_attackdash", effect_attackdash, Low).sound_acmd("sound_attackdash", sound_attackdash, Low)
        // smashes (angled f-smashes use the same animation)
        .game_acmd("game_attacks4", game_attacks4, Low).effect_acmd("effect_attacks4", effect_attacks4, Low).sound_acmd("sound_attacks4", sound_attacks4, Low)
        .game_acmd("game_attacks4hi", game_attacks4, Low).effect_acmd("effect_attacks4hi", effect_attacks4, Low).sound_acmd("sound_attacks4hi", sound_attacks4, Low)
        .game_acmd("game_attacks4lw", game_attacks4, Low).effect_acmd("effect_attacks4lw", effect_attacks4, Low).sound_acmd("sound_attacks4lw", sound_attacks4, Low)
        .game_acmd("game_attackhi4", game_attackhi4, Low).effect_acmd("effect_attackhi4", effect_attackhi4, Low).sound_acmd("sound_attackhi4", sound_attackhi4, Low)
        .game_acmd("game_attacklw4", game_attacklw4, Low).effect_acmd("effect_attacklw4", effect_attacklw4, Low).sound_acmd("sound_attacklw4", sound_attacklw4, Low)
        .effect_acmd("effect_attacks4charge", effect_smashcharge, Low).effect_acmd("effect_attackhi4charge", effect_smashcharge, Low)
        .effect_acmd("effect_attacklw4charge", effect_smashcharge, Low)
        // aerials
        .game_acmd("game_attackairn", game_attackairn, Low).effect_acmd("effect_attackairn", effect_attackairn, Low).sound_acmd("sound_attackairn", sound_attackairn, Low)
        .game_acmd("game_attackairf", game_attackairf, Low).effect_acmd("effect_attackairf", effect_attackairf, Low).sound_acmd("sound_attackairf", sound_attackairf, Low)
        .game_acmd("game_attackairb", game_attackairb, Low).effect_acmd("effect_attackairb", effect_attackairb, Low).sound_acmd("sound_attackairb", sound_attackairb, Low)
        .game_acmd("game_attackairhi", game_attackairhi, Low).effect_acmd("effect_attackairhi", effect_attackairhi, Low).sound_acmd("sound_attackairhi", sound_attackairhi, Low)
        .game_acmd("game_attackairlw", game_attackairlw, Low).effect_acmd("effect_attackairlw", effect_attackairlw, Low).sound_acmd("sound_attackairlw", sound_attackairlw, Low)
        // grabs + throws
        .game_acmd("game_catch", game_catch, Low).effect_acmd("effect_catch", effect_none, Low).sound_acmd("sound_catch", sound_catch, Low)
        .game_acmd("game_catchdash", game_catchdash, Low).effect_acmd("effect_catchdash", effect_none, Low).sound_acmd("sound_catchdash", sound_catch, Low)
        .game_acmd("game_catchturn", game_catchturn, Low).effect_acmd("effect_catchturn", effect_none, Low).sound_acmd("sound_catchturn", sound_catch, Low)
        .game_acmd("game_catchattack", game_catchattack, Low).effect_acmd("effect_catchattack", effect_catchattack, Low).sound_acmd("sound_catchattack", sound_catchattack, Low)
        .game_acmd("game_throwf", game_throwf, Low).effect_acmd("effect_throwf", effect_throwf, Low).sound_acmd("sound_throwf", sound_throwf, Low)
        .game_acmd("game_throwb", game_throwb, Low).effect_acmd("effect_throwb", effect_throwb, Low).sound_acmd("sound_throwb", sound_throwb, Low)
        .game_acmd("game_throwhi", game_throwhi, Low).effect_acmd("effect_throwhi", effect_throwhi, Low).sound_acmd("sound_throwhi", sound_throwhi, Low)
        .game_acmd("game_throwlw", game_throwlw, Low).effect_acmd("effect_throwlw", effect_throwlw, Low).sound_acmd("sound_throwlw", sound_throwlw, Low)
        // neutral B: Focus Blast (ground + air)
        .game_acmd("game_specialnstart", effect_none, Low).effect_acmd("effect_specialnstart", effect_fbstart, Low).sound_acmd("sound_specialnstart", sound_fbstart, Low)
        .game_acmd("game_specialairnstart", effect_none, Low).effect_acmd("effect_specialairnstart", effect_fbstart, Low).sound_acmd("sound_specialairnstart", sound_fbstart, Low)
        .game_acmd("game_specialn", effect_none, Low).effect_acmd("effect_specialn", effect_none, Low).sound_acmd("sound_specialn", sound_fbhold, Low)
        .game_acmd("game_specialairn", effect_none, Low).effect_acmd("effect_specialairn", effect_none, Low).sound_acmd("sound_specialairn", sound_fbhold, Low)
        .game_acmd("game_specialnend", game_fbend, Low).effect_acmd("effect_specialnend", effect_fbend, Low).sound_acmd("sound_specialnend", sound_fbend, Low)
        .game_acmd("game_specialairnend", game_fbend, Low).effect_acmd("effect_specialairnend", effect_fbend, Low).sound_acmd("sound_specialairnend", sound_fbend, Low)
        // side B: Metal Claw
        .game_acmd("game_specials", game_specials_gen, Low).effect_acmd("effect_specials", effect_metalclaw, Low).sound_acmd("sound_specials", sound_metalclaw, Low)
        .game_acmd("game_specialairs", game_specials_gen, Low).effect_acmd("effect_specialairs", effect_metalclaw, Low).sound_acmd("sound_specialairs", sound_metalclaw, Low)
        // up B: Pursuit
        .game_acmd("game_specialhi", game_specialhi_gen, Low).effect_acmd("effect_specialhi", effect_pursuit, Low).sound_acmd("sound_specialhi", sound_pursuit, Low)
        .game_acmd("game_specialairhi", game_specialhi_gen, Low).effect_acmd("effect_specialairhi", effect_pursuit, Low).sound_acmd("sound_specialairhi", sound_pursuit, Low)
        .install();
    // Razor Leaf's projectile becomes the Focus Blast orb
    Agent::new("pfushigisou_leafcutter")
        .set_costume(crate::SLOTS.to_vec())
        .game_acmd("game_move", game_orb, Low)
        .effect_acmd("effect_move", effect_orb, Low)
        .sound_acmd("sound_move", effect_none, Low)
        .install();
    // seeds / vine never spawn for Bisharp, but blank them in case a vanilla path still makes one
    Agent::new("pfushigisou_seed").set_costume(crate::SLOTS.to_vec())
        .game_acmd("game_move", effect_none, Low).effect_acmd("effect_move", effect_none, Low).install();
}
