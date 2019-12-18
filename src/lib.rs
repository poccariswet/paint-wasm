mod utils;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement, MouseEvent,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    // it can use console.log on web console
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);

    fn alert(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_body_dimensions(body: &HtmlElement) -> (u32, u32) {
    let width = body.client_width() as u32;
    let height = body.client_height() as u32;

    (width, height)
}

#[wasm_bindgen]
pub fn start_app() -> Result<(), JsValue> {
    let document = window()
        .unwrap()
        .document()
        .expect("Could not find `document`");

    let body = document.body().expect("Could not find `body` element");

    let canvas = document.get_element_by_id("draw").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let (w, h) = get_body_dimensions(&body);
    // offset solid space
    canvas.set_width(w - 10);
    canvas.set_height(h - 10);
    canvas.style().set_property("border", "5px solid")?;

    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let drawing_ok = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let drawing_ok = drawing_ok.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            drawing_ok.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let drawing_ok = drawing_ok.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if drawing_ok.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let drawing_ok = drawing_ok.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            drawing_ok.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
