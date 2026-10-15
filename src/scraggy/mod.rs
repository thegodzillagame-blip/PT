//! Scraggy (Squirtle's slot c01). Squirtle's statuses still run underneath; these scripts and the per-frame hook
//! turn them into Scraggy's moveset (see SCRAGGY_FRAME_DATA.txt in the mod folder).
use smash::app::{lua_bind::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::{Priority::*, *};
use smash_script::*;

mod normals;
mod specials;
mod throws;

static mut FP_CHARGE: [i32; 8] = [0; 8];      // Focus Punch charge frames (kept between uses, like DK's punch)
static mut HB_FRAMES: [i32; 8] = [0; 8];      // frames spent flying in the Headbutt
static mut FP_LAST: [f32; 8] = [0.0; 8];      // charge of the punch just thrown (for its effects)
static mut FP_GLOW: [bool; 8] = [false; 8];   // body tint is on (charging)
static mut FP_IDLE: [i32; 8] = [0; 8];        // timer for the "fully charged" glint while it's stored
static mut TRIP_DONE: [bool; 8] = [false; 8]; // down throw already marked its victim
static mut DAIR_BOUNCED: [bool; 8] = [false; 8]; // butt slam already bounced him this use
static mut TRIP_WAIT: [i32; 8] = [0; 8];      // per victim entry: frames left to trip them (set by the down throw)
const FP_FULL: i32 = 75;
const SQUIRTLE_WATER_FX: [&str; 24] = ["pzenigame_dash", "pzenigame_run", "pzenigame_brake", "pzenigame_dash_atk", "pzenigame_mouth_water",
    "pzenigame_water_smash", "pzenigame_water_smash_r", "pzenigame_water_ripple", "pzenigame_water_attack_spin", "pzenigame_water_attack",
    "pzenigame_water_cutter", "pzenigame_water_cutter_r", "pzenigame_atk_hi4", "pzenigame_throw_b", "pzenigame_mizuteppo_shoot",
    "pzenigame_mizuteppo_hold", "pzenigame_mizuteppo_hit", "pzenigame_mizuteppo_max", "pzenigame_takinobori", "pzenigame_takinobori_start",
    "pzenigame_takinobori_end", "pzenigame_takinobori_splash", "pzenigame_karanikomoru", "pzenigame_karanikomoru_line"];                      // frames of charging for a full-power punch
const HB_FLIGHT: i32 = 22;                    // how long the head-first flight lasts before a miss turns into the roll
const HB_SOUR: i32 = 13;                      // from here on the head is a weaker sourspot

pub(crate) unsafe fn focus_charge(e: usize) -> f32 { (FP_CHARGE[e] as f32 / FP_FULL as f32).min(1.0) }
pub(crate) unsafe fn focus_spend(e: usize) { FP_LAST[e] = focus_charge(e); FP_CHARGE[e] = 0; }
pub(crate) unsafe fn focus_last(e: usize) -> f32 { FP_LAST[e] }

unsafe extern "C" fn scraggy_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let e = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_DEAD || status == *FIGHTER_STATUS_KIND_REBIRTH {
        FP_CHARGE[e] = 0; HB_FRAMES[e] = 0;
        if FP_GLOW[e] { ColorBlendModule::cancel_main_color(boma, 0); FP_GLOW[e] = false; }
        return;
    }
    // Focus Punch: charge builds while he holds the wind-up, and it SHOWS: the body glows hotter and faster,
    // the fist flares bigger and more often, and a full charge flashes and keeps glinting while it's stored
    let charging = status == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE;
    if charging {
        if FP_CHARGE[e] < FP_FULL {
            FP_CHARGE[e] += 1;
            if FP_CHARGE[e] == FP_FULL { full_flash(boma); }
        }
        let c = focus_charge(e);
        let n = FP_CHARGE[e] as f32;
        let pulse = 0.5 + 0.5 * (n * (0.25 + 0.35 * c)).sin();
        let a = (0.12 + 0.5 * c) * (0.4 + 0.6 * pulse);
        ColorBlendModule::set_main_color(boma, &smash::phx::Vector4f { x: 1.0, y: 1.0, z: 1.0, w: 0.1 },
            &smash::phx::Vector4f { x: 1.0, y: 0.85 - 0.45 * c, z: 0.15, w: a }, 0.21, 2.2, 0, true);
        FP_GLOW[e] = true;
        let every = if FP_CHARGE[e] >= FP_FULL { 5 } else { 12 - (7.0 * c) as i32 };
        if FP_CHARGE[e] % every == 0 {
            let (r, g, b) = if FP_CHARGE[e] >= FP_FULL { (1.0, 0.35, 0.1) } else { (1.0, 0.9 - 0.4 * c, 0.3) };
            fist_fx(boma, "sys_smash_flash_s", 0.5 + 0.9 * c, r, g, b);
        }
        FP_IDLE[e] = 0;
    } else {
        if FP_GLOW[e] { ColorBlendModule::cancel_main_color(boma, 0); FP_GLOW[e] = false; }
        if FP_CHARGE[e] >= FP_FULL {
            FP_IDLE[e] += 1;
            if FP_IDLE[e] % 40 == 0 { fist_fx(boma, "sys_smash_flash_s", 0.6, 1.0, 0.45, 0.1); }
        }
    }
    // down throw: Tail Trip - on release, mark the victim; their own frame hook (trip_victim) trips them as soon as
    // they come out of the throw, so it doesn't matter how long the throw's hitlag keeps them in THROWN
    if status == *FIGHTER_STATUS_KIND_THROW && MotionModule::motion_kind(boma) == smash::hash40("throw_lw") {
        if !TRIP_DONE[e] && MotionModule::frame(boma) >= 17.0 {
            let id = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT) as u32;
            if id != 0 && id != 0x50000000 && smash::app::sv_battle_object::is_active(id) {
                let v = smash::app::sv_battle_object::module_accessor(id);
                if smash::app::utility::get_category(&mut *v) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    let ve = (WorkModule::get_int(v, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
                    TRIP_WAIT[ve] = 45;
                }
            }
            TRIP_DONE[e] = true;
        }
    } else {
        TRIP_DONE[e] = false;
    }
    // dair butt slam: on hit he pops back up a little (combo starter, not a spike)
    if status == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(boma) == smash::hash40("attack_air_lw") {
        if !DAIR_BOUNCED[e] && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.6);
            DAIR_BOUNCED[e] = true;
        }
    } else {
        DAIR_BOUNCED[e] = false;
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
        // late in the flight the headbutt is a sourspot: 5%, smaller, sends up and away instead of hard forward
        if HB_FRAMES[e] >= HB_SOUR && AttackModule::is_attack(boma, 0, false) {
            AttackModule::set_power(boma, 0, 5.0, false);
            AttackModule::set_size(boma, 0, 3.0);
            AttackModule::set_vector(boma, 0, 70, false);
        }
        if HB_FRAMES[e] == HB_FLIGHT {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, false);
        }
    } else {
        HB_FRAMES[e] = 0;
    }
}
unsafe fn fist_fx(boma: *mut BattleObjectModuleAccessor, name: &str, scale: f32, r: f32, g: f32, b: f32) {
    let z = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
    let h = EffectModule::req_follow(boma, smash::phx::Hash40::new(name), smash::phx::Hash40::new("handr"), &z, &z, scale, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(boma, h as u32, r, g, b);
}
/// runs on every fighter: a victim marked by Scraggy's down throw trips (SLIP -> prone) once the throw lets go of them
unsafe extern "C" fn trip_victim(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let e = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
    if TRIP_WAIT[e] <= 0 { return; }
    TRIP_WAIT[e] -= 1;
    let s = StatusModule::status_kind(boma);
    if s == *FIGHTER_STATUS_KIND_THROWN || s == *FIGHTER_STATUS_KIND_CAPTURE_PULLED || s == *FIGHTER_STATUS_KIND_CAPTURE_WAIT
        || s == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE { return; }
    if s == *FIGHTER_STATUS_KIND_SLIP || s == *FIGHTER_STATUS_KIND_DOWN || s == *FIGHTER_STATUS_KIND_DEAD { TRIP_WAIT[e] = 0; return; }
    if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
        KineticModule::clear_speed_all(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SLIP, false);
        TRIP_WAIT[e] = 0;
    }
}
/// full charge: a big flash on the fist, a ring and the smash-charge "ding"
unsafe fn full_flash(boma: *mut BattleObjectModuleAccessor) {
    fist_fx(boma, "sys_smash_flash", 1.6, 1.0, 0.5, 0.1);
    fist_fx(boma, "sys_crown", 0.5, 1.0, 0.8, 0.3);
    SoundModule::play_se(boma, smash::phx::Hash40::new("se_common_smash_start_02"), true, false, false, false, smash::app::enSEType(0));
}

pub fn install() {
    Agent::new("fighter").on_line(Main, trip_victim).install();
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
