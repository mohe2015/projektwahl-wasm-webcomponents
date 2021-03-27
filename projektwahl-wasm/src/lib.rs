

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    let window = web_sys::window().expect("no window exists");
    let document = window.document().expect("window should have a document");
    let body = document.body().expect("document should have a body");                
    let val = document.create_element("p")?;
    val.set_inner_html("Hello World from webassemblyMan!");                
    body.append_child(&val)?;                
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}