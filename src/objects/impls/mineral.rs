use crate::{
    constants::{Density, ResourceType},
    objects::RoomObject,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Mineral`], which can be harvested for resources with an extractor.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Mineral;

    /// The density of the mineral on the next refill after it's depleted.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.density)
    #[wasm_bindgen(method, getter)]
    pub fn density(this: &Mineral) -> Density;

    /// Type of resource contained in this mineral.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.mineralType)
    #[wasm_bindgen(method, getter = mineralType)]
    pub fn mineral_type(this: &Mineral) -> ResourceType;

    /// Object ID of the mineral, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Mineral) -> JsString;

    /// The number of ticks until this mineral regenerates from depletion.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.ticksToRegeneration)
    #[wasm_bindgen(method, getter = ticksToRegeneration)]
    pub fn ticks_to_regeneration(this: &Mineral) -> u32;
}

// use crate::{
//     constants::{Density, ResourceType},
//     objects::Mineral,
// };

// simple_accessors! {
//     impl Mineral {
//         pub fn density() -> Density = density;
//         // id from HasId trait
//     }
// }

// impl Mineral {
//     pub fn mineral_type(&self) -> ResourceType {
//         js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
//     }

//     pub fn mineral_amount(&self) -> u32 {
//         // workaround for the fact that some private servers return floating point
//         // mineralAmount values
//         js_unwrap!(Math.floor(@{self.as_ref()}.mineralAmount))
//     }

//     pub fn ticks_to_regeneration(&self) -> u32 {
//         js_unwrap!(Math.max(0, @{self.as_ref()}.ticksToRegeneration || 0))
//     }
// }
