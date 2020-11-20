use crate::objects::{RoomObject, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePowerBank`], which can be destroyed
    /// for power resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructurePowerBank;

    /// The amount of power contained within the [`StructurePowerBank`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank.power)
    #[wasm_bindgen(method, getter)]
    pub fn power(this: &StructurePowerBank) -> u32;

    /// The number of ticks until the [`StructurePowerBank`] will decay.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructurePowerBank) -> u32;
}

// use crate::objects::StructurePowerBank;

// simple_accessors! {
//     impl StructurePowerBank {
//         pub fn power() -> u32 = power;
//     }
// }
