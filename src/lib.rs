use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console; // usefull for debugging

// When attached to a pub function this attribute will configure the start 
// section of the wasm executable to be emitted, executing the tagged function 
// as soon as the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")? //  get_context returns a Result<Option<Object>>
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    //context.set_fill_style(&"#0000FF".into());     
    context.set_fill_style(&"rgb(150,50,0)".into());
    context.move_to(300.0, 0.0); // top of triangle

    context.begin_path();
    context.line_to(0.0, 600.0); // bottom left of triangle
    context.line_to(600.0, 600.0); // bottom right of triangle
    context.line_to(300.0, 0.0); // back to top of triangle
    context.close_path();
    
    context.stroke(); // outline
    context.fill(); // inside

    // log_1 expects one argument 
    // [more here](https://www.webassemblyman.com/rustwasm/web_sys_console_log2.html)
    console::log_2(&"Color : %s ".into(), &context.fill_style().into());

    Ok(())
}
