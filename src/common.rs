#[cfg(not(target_arch = "wasm32"))]
extern crate rand;

#[cfg(not(target_arch = "wasm32"))]
pub fn random() -> f64 {
    rand::random()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f64;

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> usize;
}
