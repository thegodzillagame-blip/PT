//! Haxorus (Charizard's slot c01). Only the moves he does differently are replaced;
//! everything else keeps running Charizard's code on his retargeted animations.
use smash::app::{lua_bind::*, *};
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smashline::{Priority::*, *};

mod kept_fixed;
mod normals;
mod throws;

/// Nair roar: his body yaw (degrees from facing forward) on each frame of c05attackairn. Authored facing right,
/// where it turns him toward the camera; facing left the game spins the model 180 so he'd turn away instead.
const NAIR_YAW: [f32; 53] = [0.,-22.,-38.,-50.,-58.,-60.,-63.,-71.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-85.,-84.,-83.,-80.,-76.,-72.,-67.,-62.,-58.,-53.,-49.,-45.,-42.,-41.,-40.,-40.,-39.,-37.,-35.,-32.,-30.,-27.,-23.,-20.,-17.,-13.,-10.,-8.,-5.,-3.,-1.,0.,0.];

/// Every frame: Charizard's code keeps re-spawning his tail flame - Haxorus has none.
/// Also mirrors the nair turn when he faces left, so the roar always faces the camera.
unsafe extern "C" fn haxorus_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("plizardon_tail_fire"), false, true);
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
        .install();
}
