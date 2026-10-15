//! Scraggy (Squirtle's slot c01). Squirtle's statuses still run underneath; these scripts and the per-frame hook
//! turn them into Scraggy's moveset (see SCRAGGY_FRAME_DATA.txt in the mod folder).
use smash::app::{lua_bind::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::{Priority::*, *};

mod normals;
mod specials;
mod throws;

static mut FP_CHARGE: [i32; 8] = [0; 8];      // Focus Punch charge frames (kept between uses, like DK's punch)
static mut HB_FRAMES: [i32; 8] = [0; 8];      // frames spent flying in the Headbutt
const FP_FULL: i32 = 75;
const SQUIRTLE_WATER_FX: [&str; 24] = ["pzenigame_dash", "pzenigame_run", "pzenigame_brake", "pzenigame_dash_atk", "pzenigame_mouth_water",
    "pzenigame_water_smash", "pzenigame_water_smash_r", "pzenigame_water_ripple", "pzenigame_water_attack_spin", "pzenigame_water_attack",
    "pzenigame_water_cutter", "pzenigame_water_cutter_r", "pzenigame_atk_hi4", "pzenigame_throw_b", "pzenigame_mizuteppo_shoot",
    "pzenigame_mizuteppo_hold", "pzenigame_mizuteppo_hit", "pzenigame_mizuteppo_max", "pzenigame_takinobori", "pzenigame_takinobori_start",
    "pzenigame_takinobori_end", "pzenigame_takinobori_splash", "pzenigame_karanikomoru", "pzenigame_karanikomoru_line"];                      // frames of charging for a full-power punch
const HB_FLIGHT: i32 = 12;                    // how long the head-first flight lasts before a miss turns into the roll

pub(crate) unsafe fn focus_charge(e: usize) -> f32 { (FP_CHARGE[e] as f32 / FP_FULL as f32).min(1.0) }
pub(crate) unsafe fn focus_spend(e: usize) { FP_CHARGE[e] = 0; }

unsafe extern "C" fn scraggy_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let e = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_DEAD || status == *FIGHTER_STATUS_KIND_REBIRTH { FP_CHARGE[e] = 0; HB_FRAMES[e] = 0; return; }
    // Focus Punch: charge builds while he holds the wind-up
    if status == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE && FP_CHARGE[e] < FP_FULL {
        FP_CHARGE[e] += 1;
        if FP_CHARGE[e] == FP_FULL { macros_flash(boma); }
    }
    // jab is 2 hits: Squirtle's status would still chain into jab 3, so never let jab 2 open the combo
    if status == *FIGHTER_STATUS_KIND_ATTACK && MotionModule::motion_kind(boma) == smash::hash40("attack_12") {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    if status == *FIGHTER_STATUS_KIND_ATTACK && MotionModule::motion_kind(boma) == smash::hash40("attack_13") {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    // Focus Punch: jump-cancelable once it connects
    if status == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        let cat = ControlModule::get_command_flag_cat(boma, 0);
        if cat & (*FIGHTER_PAD_CMD_CAT1_FLAG_JUMP | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            } else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            }
        }
    }
    // Brick Break rock: the Up B animations show it until the smash (f10); hide it everywhere else as a backstop
    let m = MotionModule::motion_kind(boma);
    let rock = (m == smash::hash40("special_hi") || m == smash::hash40("special_air_hi")) && MotionModule::frame(boma) <= 10.0;
    ModelModule::set_mesh_visibility(boma, smash::phx::Hash40::new("pzenigame_rock"), rock);
    // Scraggy has nothing to do with water: clear any of Squirtle's own water effects that a vanilla script still spawns
    for name in SQUIRTLE_WATER_FX { EffectModule::kill_kind(boma, smash::phx::Hash40::new(name), false, true); }
    // no Water Gun water, ever
    ArticleModule::remove_exist(boma, *FIGHTER_PZENIGAME_GENERATE_ARTICLE_WATER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    // Headbutt: a short head-first flight, then the miss roll (Withdraw would keep sliding)
    if status == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP {
        HB_FRAMES[e] += 1;
        if HB_FRAMES[e] == HB_FLIGHT {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, false);
        }
    } else {
        HB_FRAMES[e] = 0;
    }
}
/// full charge: a bright flash on the fist
unsafe fn macros_flash(boma: *mut BattleObjectModuleAccessor) {
    let z = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
    let h = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("handr"), &z, &z, 1.0, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(boma, h as u32, 1.0, 0.9, 0.3);
}

pub fn install() {
    use normals::*;
    use specials::*;
    use throws::*;
    Agent::new("pzenigame")
        .set_costume(crate::SLOTS.to_vec())
        .on_line(Main, scraggy_frame)
        // movement: dust, not water
        .effect_acmd("effect_dash", effect_dash, Low)
        .effect_acmd("effect_turndash", effect_turndash, Low)
        .effect_acmd("effect_run", effect_run, Low)
        .effect_acmd("effect_runbrake", effect_runbrake, Low)
        .effect_acmd("effect_runbraker", effect_runbrake, Low)
        .effect_acmd("effect_runbrakel", effect_runbrake, Low)
        .effect_acmd("effect_turnrun", effect_turnrun, Low)
        .effect_acmd("effect_turnrunbrake", effect_runbrake, Low)
        // jab (2 hits)
        .game_acmd("game_attack11", game_attack11, Low)
        .effect_acmd("effect_attack11", effect_attack11, Low)
        .sound_acmd("sound_attack11", sound_attack11, Low)
        .game_acmd("game_attack12", game_attack12, Low)
        .effect_acmd("effect_attack12", effect_attack12, Low)
        .sound_acmd("sound_attack12", sound_attack12, Low)
        .game_acmd("game_attack13", game_attack13, Low)
        .game_acmd("game_attackdash", game_attackdash, Low)
        .effect_acmd("effect_attackdash", effect_attackdash, Low)
        .sound_acmd("sound_attackdash", sound_attackdash, Low)
        // tilts (the angled f-tilt/f-smash have no Scraggy animation yet, so they stay Squirtle's)
        .game_acmd("game_attacks3", game_attacks3, Low)
        .effect_acmd("effect_attacks3", effect_attacks3, Low)
        .sound_acmd("sound_attacks3", sound_attacks3, Low)
        .game_acmd("game_attackhi3", game_attackhi3, Low)
        .effect_acmd("effect_attackhi3", effect_attackhi3, Low)
        .sound_acmd("sound_attackhi3", sound_attackhi3, Low)
        .game_acmd("game_attacklw3", game_attacklw3, Low)
        .effect_acmd("effect_attacklw3", effect_attacklw3, Low)
        .sound_acmd("sound_attacklw3", sound_attacklw3, Low)
        // smashes
        .game_acmd("game_attacks4", game_attacks4, Low)
        .effect_acmd("effect_attacks4", effect_attacks4, Low)
        .sound_acmd("sound_attacks4", sound_attacks4, Low)
        .game_acmd("game_attackhi4", game_attackhi4, Low)
        .effect_acmd("effect_attackhi4", effect_attackhi4, Low)
        .sound_acmd("sound_attackhi4", sound_attackhi4, Low)
        .game_acmd("game_attacklw4", game_attacklw4, Low)
        .effect_acmd("effect_attacklw4", effect_attacklw4, Low)
        .sound_acmd("sound_attacklw4", sound_attacklw4, Low)
        // aerials
        .game_acmd("game_attackairn", game_attackairn, Low)
        .effect_acmd("effect_attackairn", effect_attackairn, Low)
        .sound_acmd("sound_attackairn", sound_attackairn, Low)
        .game_acmd("game_attackairf", game_attackairf, Low)
        .effect_acmd("effect_attackairf", effect_attackairf, Low)
        .sound_acmd("sound_attackairf", sound_attackairf, Low)
        .game_acmd("game_attackairb", game_attackairb, Low)
        .effect_acmd("effect_attackairb", effect_attackairb, Low)
        .sound_acmd("sound_attackairb", sound_attackairb, Low)
        .game_acmd("game_attackairhi", game_attackairhi, Low)
        .effect_acmd("effect_attackairhi", effect_attackairhi, Low)
        .sound_acmd("sound_attackairhi", sound_attackairhi, Low)
        .game_acmd("game_attackairlw", game_attackairlw, Low)
        .effect_acmd("effect_attackairlw", effect_attackairlw, Low)
        .sound_acmd("sound_attackairlw", sound_attackairlw, Low)
        // throws (up throw stays Squirtle's)
        .game_acmd("game_throwf", game_throwf, Low)
        .effect_acmd("effect_throwf", effect_throwf, Low)
        .sound_acmd("sound_throwf", sound_throwf, Low)
        .game_acmd("game_throwlw", game_throwlw, Low)
        .effect_acmd("effect_throwlw", effect_throwlw, Low)
        .sound_acmd("sound_throwlw", sound_throwlw, Low)
        .game_acmd("game_throwb", game_throwb, Low)
        .effect_acmd("effect_throwb", effect_throwb, Low)
        .sound_acmd("sound_throwb", sound_throwb, Low)
        // neutral B: Focus Punch (ground + air)
        .effect_acmd("effect_specialnstart", effect_specialnstart, Low)
        .sound_acmd("sound_specialnstart", sound_specialnstart, Low)
        .effect_acmd("effect_specialairnstart", effect_specialnstart, Low)
        .sound_acmd("sound_specialairnstart", sound_specialnstart, Low)
        .effect_acmd("effect_specialnhold", effect_specialnhold, Low)
        .sound_acmd("sound_specialnhold", sound_specialnhold, Low)
        .effect_acmd("effect_specialairnhold", effect_specialnhold, Low)
        .sound_acmd("sound_specialairnhold", sound_specialnhold, Low)
        .effect_acmd("effect_specialnchargemax", effect_specialnhold, Low)
        .game_acmd("game_specialnshot", game_specialnshot, Low)
        .effect_acmd("effect_specialnshot", effect_specialnshot, Low)
        .sound_acmd("sound_specialnshot", sound_specialnshot, Low)
        .game_acmd("game_specialairnshot", game_specialnshot, Low)
        .effect_acmd("effect_specialairnshot", effect_specialnshot, Low)
        .sound_acmd("sound_specialairnshot", sound_specialnshot, Low)
        // side B: Headbutt
        .game_acmd("game_specialsstart", game_specialsstart, Low)
        .effect_acmd("effect_specialsstart", effect_specialsstart, Low)
        .sound_acmd("sound_specialsstart", sound_specialsstart, Low)
        .game_acmd("game_specialairsstart", game_specialsstart, Low)
        .effect_acmd("effect_specialairsstart", effect_specialsstart, Low)
        .sound_acmd("sound_specialairsstart", sound_specialsstart, Low)
        .game_acmd("game_specials", game_specials, Low)
        .effect_acmd("effect_specials", effect_specials, Low)
        .sound_acmd("sound_specials", sound_specials, Low)
        .game_acmd("game_specialairs", game_specials, Low)
        .effect_acmd("effect_specialairs", effect_specials, Low)
        .sound_acmd("sound_specialairs", sound_specials, Low)
        .effect_acmd("effect_specialairshit", effect_specialshit, Low)
        .sound_acmd("sound_specialairshit", sound_specialshit, Low)
        .game_acmd("game_specialsend", game_specialsend, Low)
        .effect_acmd("effect_specialsend", effect_specialsend, Low)
        .sound_acmd("sound_specialsend", sound_specialsend, Low)
        .game_acmd("game_specialairsend", game_specialsend, Low)
        .effect_acmd("effect_specialairsend", effect_specialsend, Low)
        .sound_acmd("sound_specialairsend", sound_specialsend, Low)
        // up B: Brick Break
        .game_acmd("game_specialhi", game_specialhi, Low)
        .effect_acmd("effect_specialhi", effect_specialhi, Low)
        .sound_acmd("sound_specialhi", sound_specialhi, Low)
        .game_acmd("game_specialairhi", game_specialairhi, Low)
        .effect_acmd("effect_specialairhi", effect_specialairhi, Low)
        .sound_acmd("sound_specialairhi", sound_specialairhi, Low)
        .install();
}
