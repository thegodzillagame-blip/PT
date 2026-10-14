//! PT team (c01) - custom move code for costume slot 2 of Pokemon Trainer's team:
//! Scraggy (Squirtle), Bisharp (Ivysaur), Haxorus (Charizard).
//! Every script is installed for costume 1 (= c01) only, so all other slots stay vanilla.
#![allow(non_snake_case, unused_macros, unused_imports, unused_unsafe, clippy::all)]

mod util;
mod haxorus;

/// costume slots this mod owns (c01)
pub const SLOTS: [usize; 1] = [1];

#[skyline::main(name = "pt_team_c01")]
pub fn main() {
    haxorus::install();
}
