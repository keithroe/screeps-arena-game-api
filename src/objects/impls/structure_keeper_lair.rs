use crate::objects::{OwnedStructure, RoomObject, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureKeeperLair`], which regularly spawns
    /// creeps to defend nearby resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureKeeperLair;

    /// The number of ticks until the [`StructureKeeperLair`] will spawn a new
    /// creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair.ticksToSpawn)
    #[wasm_bindgen(method, getter = ticksToSpawn)]
    pub fn ticks_to_spawn(this: &StructureKeeperLair) -> u32;
}

// use crate::objects::StructureKeeperLair;

// simple_accessors! {
//     impl StructureKeeperLair {
//         pub fn ticks_to_spawn() -> u32 = ticksToSpawn;
//     }
// }
