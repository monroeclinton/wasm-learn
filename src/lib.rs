use wasm_bindgen::prelude::*;
use web_sys::{console, window, MouseEvent};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn mouse_listener() {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let callback = Box::new(move |e: MouseEvent| {
        console::log_1(&"Got mouse event".into());
        console::log_3(&"Pos:".into(), &e.x().into(), &e.y().into());
    });

    let closure = Closure::wrap(callback as Box<dyn FnMut(MouseEvent)>);
    document.set_onmousemove(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
