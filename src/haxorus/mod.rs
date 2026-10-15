//! Haxorus (Charizard's slot c01). Only the moves he does differently are replaced;
//! everything else keeps running Charizard's code on his retargeted animations.
use smash::app::{lua_bind::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smashline::{Priority::*, *};
use smash_script::*;

mod kept_fixed;
mod normals;
mod specials;
mod throws;

/// Nair roar: his body yaw (degrees from facing forward) on each frame of c05attackairn. Authored facing right,
/// where it turns him toward the camera; facing left the game spins the model 180 so he'd turn away instead.
const NAIR_YAW: [f32; 53] = [0.,-22.,-38.,-50.,-58.,-60.,-63.,-71.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-84.,-83.,-80.,-76.,-72.,-67.,-62.,-58.,-53.,-49.,-45.,-42.,-41.,-40.,-40.,-39.,-37.,-35.,-32.,-30.,-27.,-23.,-20.,-17.,-13.,-10.,-8.,-5.,-3.,-1.,0.,0.];

// Dragon Dance state, per player slot
static mut DD_DANCE: [i32; 8] = [0; 8];       // frames spent stomping in the current dance
static mut DD_GRANTED: [bool; 8] = [false; 8];
static mut DD_SPEED: [i32; 8] = [0; 8];       // frames of speed boost left
static mut DD_ARMED: [bool; 8] = [false; 8];  // next attack is powered up
static mut DD_USING: [bool; 8] = [false; 8];  // ...and that attack is happening now
static mut OUTRAGE_HITS: [i32; 8] = [0; 8];
static mut OUTRAGE_WAS_HIT: [bool; 8] = [false; 8];
static mut OUTRAGE_FRAMES: [i32; 8] = [0; 8];
static mut GIGA: [bool; 8] = [false; 8];
const DD_NEED: i32 = 60;               // frames of dancing (about 3 stomp cycles) before letting go grants the buff
const DD_SPEED_FRAMES: i32 = 300;      // 5 seconds of extra speed
const DD_SPEED_MUL: f32 = 0.3;         // +30% movement speed
const DD_POWER: f32 = 1.4;             // the next attack only: +40% damage
const GIGA_DIVE_SPEED: f32 = 4.5;      // units/frame straight down (Charizard's max fall is ~1.9)
const OUTRAGE_MULTI: i32 = 4;          // claw rakes before the finisher
const OUTRAGE_SLOW: f32 = 0.35;        // side B runs at 65% of Flare Blitz's speed
const OUTRAGE_RUSH_MAX: i32 = 13;      // frames of running before a whiffed rampage stops (shorter than Flare Blitz)

unsafe fn is_attack_status(st: i32) -> bool {
    [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
     *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4,
     *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4,
     *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4,
     *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_SPECIAL_S,
     *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END,
     *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&st)
}
unsafe fn is_move_status(st: i32) -> bool {
    [*FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_DASH,
     *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FALL,
     *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&st)
}
unsafe fn aura(boma: *mut BattleObjectModuleAccessor, bone: &str, scale: f32, r: f32, g: f32, b: f32) {
    let z = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
    let h = EffectModule::req_follow(boma, Hash40::new("sys_hit_aura_s"), Hash40::new(bone), &z, &z, scale, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(boma, h as u32, r, g, b);
}

/// Dragon Dance: dance for a while, then the roar gives a short burst of speed and powers up the NEXT attack only.
unsafe fn dragon_dance(boma: *mut BattleObjectModuleAccessor, e: usize) {
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_DEAD || status == *FIGHTER_STATUS_KIND_REBIRTH {
        DD_DANCE[e] = 0; DD_SPEED[e] = 0; DD_ARMED[e] = false; DD_USING[e] = false;
        AttackModule::set_power_up(boma, 1.0);
        return;
    }
    // no Flamethrower: the breath article never gets to exist (its scripts are blanked too, see install)
    ArticleModule::remove_exist(boma, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_BREATH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    EffectModule::kill_kind(boma, Hash40::new("plizardon_kaenhousya_start"), false, true);
    EffectModule::kill_kind(boma, Hash40::new("plizardon_kaenhousya_start_air"), false, true);
    let motion = MotionModule::motion_kind(boma);
    // air Dragon Dance: kills all momentum as he starts, like a stall
    if motion == smash::hash40("special_air_n_start") && MotionModule::frame(boma) < 1.0 {
        KineticModule::clear_speed_all(boma);
    }
    if motion == smash::hash40("special_n_start") || motion == smash::hash40("special_air_n_start") { DD_DANCE[e] = 0; }
    if motion == smash::hash40("special_n") || motion == smash::hash40("special_air_n") { DD_DANCE[e] += 1; }
    let ending = motion == smash::hash40("special_n_end") || motion == smash::hash40("special_air_n_end");
    if ending {
        if !DD_GRANTED[e] && MotionModule::frame(boma) >= 10.0 {
            DD_GRANTED[e] = true;
            if DD_DANCE[e] >= DD_NEED {
                DD_SPEED[e] = DD_SPEED_FRAMES;
                DD_ARMED[e] = true;
                DD_USING[e] = false;
            }
        }
    } else {
        DD_GRANTED[e] = false;
    }
    // speed: extra movement on top of whatever he's doing (ground and air movement only, never in hitstun)
    if DD_SPEED[e] > 0 {
        DD_SPEED[e] -= 1;
        if is_move_status(status) {
            let vx = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            PostureModule::add_pos(boma, &Vector3f { x: vx * DD_SPEED_MUL, y: 0.0, z: 0.0 });
        }
        if DD_SPEED[e] % 20 == 0 { aura(boma, "top", 0.7, 0.4, 0.75, 1.0); }
    }
    // power: the next attack he starts is boosted; it's spent when that attack is over
    if DD_ARMED[e] {
        let attacking = is_attack_status(status);
        if attacking && !DD_USING[e] { DD_USING[e] = true; }
        if DD_USING[e] && !attacking {
            DD_ARMED[e] = false; DD_USING[e] = false;
            AttackModule::set_power_up(boma, 1.0);
        } else {
            AttackModule::set_power_up(boma, if DD_USING[e] { DD_POWER } else { 1.0 });
            if MotionModule::frame(boma) as i32 % 30 == 0 { aura(boma, "bust", 1.2, 0.65, 0.2, 1.0); }
        }
    }
}

/// Outrage: Flare Blitz's rush, but it never explodes and never blows him back (no recoil, no self damage).
/// 4 claw rakes - or a short run if he whiffs - and it goes straight into the finisher swipe.
unsafe fn outrage(boma: *mut BattleObjectModuleAccessor, e: usize) {
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_SPECIAL_S { OUTRAGE_HITS[e] = 0; OUTRAGE_WAS_HIT[e] = false; OUTRAGE_FRAMES[e] = 0; return; }
    if status != *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH { return; }
    WorkModule::on_flag(boma, *FIGHTER_PLIZARDON_STATUS_SPECIAL_S_FLAG_IS_DISABLE_FIGHTER_HIT_EXPLOSION);
    WorkModule::off_flag(boma, *FIGHTER_PLIZARDON_STATUS_SPECIAL_S_FLAG_IS_STATUS_CHANGE_BLOWN);
    let hit = AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT);
    if hit && !OUTRAGE_WAS_HIT[e] { OUTRAGE_HITS[e] += 1; }
    OUTRAGE_WAS_HIT[e] = hit;
    OUTRAGE_FRAMES[e] += 1;
    // slower charge than Flare Blitz: hold back a share of his travel every frame
    let vx = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    PostureModule::add_pos(boma, &Vector3f { x: -vx * OUTRAGE_SLOW, y: 0.0, z: 0.0 });
    // while it's connecting he keeps raking; a whiff only runs a short way
    let limit = if OUTRAGE_HITS[e] > 0 { OUTRAGE_RUSH_MAX + 10 } else { OUTRAGE_RUSH_MAX };
    if OUTRAGE_HITS[e] >= OUTRAGE_MULTI || OUTRAGE_FRAMES[e] >= limit {
        OUTRAGE_FRAMES[e] = -1000;  // only request once
        StatusModule::change_status_request_from_script(boma, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END, false);
    }
}

/// Giga Impact: cut straight into the dive once he has flipped over at the top, and dive FAST from the first frame.
unsafe fn giga_dive(fighter: &mut L2CFighterCommon, e: usize) {
    let boma = fighter.module_accessor;
    let status = StatusModule::status_kind(boma);
    if status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        GIGA[e] = true;
        if MotionModule::frame(boma) >= 48.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
        return;
    }
    if status != *FIGHTER_STATUS_KIND_FALL_SPECIAL { GIGA[e] = false; return; }
    if GIGA[e] {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, GIGA_DIVE_SPEED);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, GIGA_DIVE_SPEED);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -GIGA_DIVE_SPEED);
    }
}

/// Every frame: Charizard's code keeps re-spawning his tail flame - Haxorus has none.
/// Also mirrors the nair turn when he faces left, so the roar always faces the camera.
unsafe extern "C" fn haxorus_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("plizardon_tail_fire"), false, true);
    let e = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize).min(7);
    dragon_dance(boma, e);
    outrage(boma, e);
    giga_dive(fighter, e);
    if PostureModule::lr(boma) < 0.0 && MotionModule::motion_kind(boma) == smash::hash40("attack_air_n") {
        let f = (MotionModule::frame(boma) as usize).min(NAIR_YAW.len() - 1);
        let rot = Vector3f { x: 0.0, y: -2.0 * NAIR_YAW[f], z: 0.0 };
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rot,
            MotionNodeRotateCompose { _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8 },
            MotionNodeRotateOrder { _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8 });
    }
}

pub fn install() {
    use kept_fixed::*;
    use normals::*;
    use throws::*;
    use specials::*;
    Agent::new("plizardon")
        .set_costume(crate::SLOTS.to_vec())
        .on_line(Main, haxorus_frame)
        // jabs
        .game_acmd("game_attack11", game_attack11, Low)
        .effect_acmd("effect_attack11", effect_attack11, Low)
        .sound_acmd("sound_attack11", sound_attack11, Low)
        .game_acmd("game_attack12", game_attack12, Low)
        .effect_acmd("effect_attack12", effect_attack12, Low)
        .sound_acmd("sound_attack12", sound_attack12, Low)
        .game_acmd("game_attack13", game_attack13, Low)
        .effect_acmd("effect_attack13", effect_attack13, Low)
        .sound_acmd("sound_attack13", sound_attack13, Low)
        // dash attack
        .game_acmd("game_attackdash", game_attackdash, Low)
        .effect_acmd("effect_attackdash", effect_attackdash, Low)
        .sound_acmd("sound_attackdash", sound_attackdash, Low)
        // tilts
        .game_acmd("game_attacks3", game_attacks3, Low)
        .effect_acmd("effect_attacks3", effect_attacks3, Low)
        .sound_acmd("sound_attacks3", sound_attacks3, Low)
        .expression_acmd("expression_attacks3", expression_attacks3, Low)
        .game_acmd("game_attacks3hi", game_attacks3hi, Low)
        .game_acmd("game_attacks3lw", game_attacks3lw, Low)
        .game_acmd("game_attackhi3", game_attackhi3, Low)
        .effect_acmd("effect_attackhi3", effect_attackhi3, Low)
        .sound_acmd("sound_attackhi3", sound_attackhi3, Low)
        // smashes
        .game_acmd("game_attackhi4", game_attackhi4, Low)
        .effect_acmd("effect_attackhi4", effect_attackhi4, Low)
        .sound_acmd("sound_attackhi4", sound_attackhi4, Low)
        .game_acmd("game_attacklw4", game_attacklw4, Low)
        .effect_acmd("effect_attacklw4", effect_attacklw4, Low)
        .sound_acmd("sound_attacklw4", sound_attacklw4, Low)
        .expression_acmd("expression_attacklw4", expression_attacklw4, Low)
        // aerials
        .game_acmd("game_attackairn", game_attackairn, Low)
        .effect_acmd("effect_attackairn", effect_attackairn, Low)
        .sound_acmd("sound_attackairn", sound_attackairn, Low)
        .game_acmd("game_attackairb", game_attackairb, Low)
        .effect_acmd("effect_attackairb", effect_attackairb, Low)
        .sound_acmd("sound_attackairb", sound_attackairb, Low)
        .expression_acmd("expression_attackairb", expression_attackairb, Low)
        .game_acmd("game_attackairf", game_attackairf, Low)
        .game_acmd("game_attackairhi", game_attackairhi, Low)
        .effect_acmd("effect_attackairhi", effect_attackairhi, Low)
        .sound_acmd("sound_attackairhi", sound_attackairhi, Low)
        // get-up attacks (tail tip off the parked fire bone)
        .game_acmd("game_downattackd", game_downattackd, Low)
        .game_acmd("game_downattacku", game_downattacku, Low)
        // throws
        .game_acmd("game_throwhi", game_throwhi, Low)
        .effect_acmd("effect_throwhi", effect_throwhi, Low)
        .sound_acmd("sound_throwhi", sound_throwhi, Low)
        .expression_acmd("expression_throwhi", expression_throwhi, Low)
        .game_acmd("game_throwlw", game_throwlw, Low)
        .effect_acmd("effect_throwlw", effect_throwlw, Low)
        .sound_acmd("sound_throwlw", sound_throwlw, Low)
        .expression_acmd("expression_throwlw", expression_throwlw, Low)
        // ---- specials
        .effect_acmd("effect_specialnstart", effect_specialnstart, Low)
        .sound_acmd("sound_specialnstart", sound_specialnstart, Low)
        .effect_acmd("effect_specialn", effect_specialn, Low)
        .sound_acmd("sound_specialn", sound_specialn, Low)
        .effect_acmd("effect_specialnend", effect_specialnend, Low)
        .sound_acmd("sound_specialnend", sound_specialnend, Low)
        .effect_acmd("effect_specialairnstart", effect_specialnstart, Low)
        .sound_acmd("sound_specialairnstart", sound_specialnstart, Low)
        .effect_acmd("effect_specialairn", effect_specialn, Low)
        .sound_acmd("sound_specialairn", sound_specialn, Low)
        .effect_acmd("effect_specialairnend", effect_specialnend, Low)
        .sound_acmd("sound_specialairnend", sound_specialnend, Low)
        .effect_acmd("effect_specialsstart", effect_specialsstart, Low)
        .sound_acmd("sound_specialsstart", sound_specialsstart, Low)
        .game_acmd("game_specials", game_specials, Low)
        .effect_acmd("effect_specials", effect_specials, Low)
        .sound_acmd("sound_specials", sound_specials, Low)
        .game_acmd("game_specialsend", game_specialsend, Low)
        .effect_acmd("effect_specialsend", effect_specialsend, Low)
        .sound_acmd("sound_specialsend", sound_specialsend, Low)
        .effect_acmd("effect_specialsblown", effect_specialsblown, Low)
        .effect_acmd("effect_specialairsstart", effect_specialsstart, Low)
        .sound_acmd("sound_specialairsstart", sound_specialsstart, Low)
        .game_acmd("game_specialairs", game_specialairs, Low)
        .effect_acmd("effect_specialairs", effect_specialairs, Low)
        .sound_acmd("sound_specialairs", sound_specialairs, Low)
        .game_acmd("game_specialairsend", game_specialairsend, Low)
        .effect_acmd("effect_specialairsend", effect_specialairsend, Low)
        .sound_acmd("sound_specialairsend", sound_specialairsend, Low)
        .effect_acmd("effect_specialairsblown", effect_specialsblown, Low)
        .game_acmd("game_specialhi", game_specialhi, Low)
        .effect_acmd("effect_specialhi", effect_specialhi, Low)
        .sound_acmd("sound_specialhi", sound_specialhi, Low)
        .game_acmd("game_specialairhi", game_specialairhi, Low)
        .effect_acmd("effect_specialairhi", effect_specialairhi, Low)
        .sound_acmd("sound_specialairhi", sound_specialairhi, Low)
        .game_acmd("game_fallspecial", game_fallspecial, Low)
        .effect_acmd("effect_fallspecial", effect_fallspecial, Low)
        .game_acmd("game_fallspecialf", game_fallspecial, Low)
        .effect_acmd("effect_fallspecialf", effect_fallspecial, Low)
        .game_acmd("game_fallspecialb", game_fallspecial, Low)
        .effect_acmd("effect_fallspecialb", effect_fallspecial, Low)
        .game_acmd("game_landingfallspecial", game_landingfallspecial, Low)
        .effect_acmd("effect_landingfallspecial", effect_landingfallspecial, Low)
        .sound_acmd("sound_landingfallspecial", sound_landingfallspecial, Low)
        .expression_acmd("expression_landingfallspecial", expression_landingfallspecial, Low)
        .install();
    // no Flamethrower: the breath projectile does nothing and shows nothing
    Agent::new("plizardon_breath")
        .set_costume(crate::SLOTS.to_vec())
        .game_acmd("game_move", specials::effect_none, Low)
        .effect_acmd("effect_move", specials::effect_none, Low)
        .sound_acmd("sound_move", specials::effect_none, Low)
        .install();
    // Outrage's rage burst replaces Flare Blitz's fire explosion
    Agent::new("plizardon_explosion")
        .set_costume(crate::SLOTS.to_vec())
        .game_acmd("game_start", specials::game_explosion, Low)
        .effect_acmd("effect_start", specials::effect_explosion, Low)
        .install();
}
