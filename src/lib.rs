#![feature(global_asm)]

use skyline::hooks::InlineCtx;
use skyline::install_hook;
use skyline_smash::lib::lua_const::*;
use skyline_smash::app::lua_bind::*;
use skyline_smash::app::*;

#[skyline::hook(offset = 0x3b1ee0)]
fn add_damage_hook(battle_object: &mut BattleObject, damage: f32, unk1: u32, unk2: bool) {
    let doubled_damage = damage * 2.0;
    skyline::log::ln!("[Plugin] Original damage: {}, Doubled: {}", damage, doubled_damage);
    original!()(battle_object, doubled_damage, unk1, unk2);
}

#[skyline::main(name = "double_damage")]
pub fn main() {
    skyline::log::ln!("[Plugin] Double damage plugin loaded!");
    install_hook!(add_damage_hook);
}