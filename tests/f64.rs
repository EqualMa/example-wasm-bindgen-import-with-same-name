use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

mod a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = parseFloat)]
        pub fn parse_float(text: &JsValue) -> f64;
    }
}

mod b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = parseFloat)]
        pub fn parse_float(text: &str) -> f64;
    }
}

#[wasm_bindgen_test]
pub fn pass_a() {
    let v = JsValue::from_str("1");
    assert_eq!(a::parse_float(&v), 1.);
}

#[wasm_bindgen_test]
pub fn pass_b() {
    assert_eq!(b::parse_float("2"), 2.);
}
