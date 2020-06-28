use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Virus {
    pub distance: f32,
    pub recovery_time: usize, // days
    pub infection_rate: f32,  // between 0 and 1
    pub mortality_rate: f32,  // between 0 and 1
}

#[wasm_bindgen]
impl Virus {
    pub fn corona() -> Virus {
        Virus {
            distance: 10.0,
            recovery_time: 100,
            infection_rate: 0.7,
            mortality_rate: 0.05,
        }
    }
}
