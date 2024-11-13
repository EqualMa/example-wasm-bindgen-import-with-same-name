use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

mod a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = "String")]
        pub fn string(v: &JsValue) -> String;
    }
}

mod b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = "String")]
        pub fn string(v: &JsValue) -> JsValue;
    }
}

#[wasm_bindgen_test]
pub fn pass_a() {
    assert_eq!(a::string(&JsValue::TRUE), "true");
}

#[wasm_bindgen_test]
pub fn pass_b() {
    assert_eq!(b::string(&JsValue::TRUE), "true");
}
