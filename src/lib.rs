extern crate wasm_bindgen;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	console::log_1(&format!("Hello, {}!", name).into());
}

fn window() -> web_sys::Window {
	web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("should register `requestAnimationFrame` OK");
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	// Your code goes here!
	console::log_1(&JsValue::from_str("Hello world!"));

	let f = Rc::new(RefCell::new(None));
	let g = f.clone();

	let mut i:f64 = 0.0;
	*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
			i+=1.0;
			request_animation_frame(f.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));
	// request_animation_frame(g.borrow().as_ref().unwrap());
	// start();
	Ok(())
}

#[wasm_bindgen]
pub fn clear_canvas(id: &str){
	console_error_panic_hook::set_once();
	let document = web_sys::window().unwrap().document().unwrap();
	let canvas = document.get_element_by_id(id).unwrap();
	let canvas: web_sys::HtmlCanvasElement = canvas
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.map_err(|_| ())
		.unwrap();

	let context = canvas
		.get_context("2d")
		.unwrap()
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap();
	context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
}

#[wasm_bindgen]
pub fn create_wave(id: &str, wave: &[f64], p: u32, margin: u32, color: &str) {
	console_error_panic_hook::set_once();
	let document = web_sys::window().unwrap().document().unwrap();
	let canvas = document.get_element_by_id(id).unwrap();
	let canvas: web_sys::HtmlCanvasElement = canvas
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.map_err(|_| ())
		.unwrap();

	let context = canvas
		.get_context("2d")
		.unwrap()
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap();

	let width:f64 = canvas.width().into();
	let height:f64 = canvas.height().into();
	let half_h:f64 = height/2.0;
	let bar_margin:f64 = margin as f64;
	let num:f64 = wave.len() as f64;
	let bar_width:f64 = width / (num-bar_margin);

	for l in 0..wave.len() {
		let sample:f64 = wave[l];
		let bar_height:f64 = sample * half_h / 2.0;
		let bh:f64 = if p == 0 {
			-bar_height
		}else{
			0.0
		};
		create_fill_rect(&context, l as f64 * (bar_width + bar_margin), half_h + bh, bar_width + bar_margin, bar_height, color);
		// create_fill_rect(&context, 0.0, 0.0, 100.0, 100.0, "red");
	}
}

fn create_fill_rect(con: &web_sys::CanvasRenderingContext2d, x :f64, y:f64, width:f64, height:f64, color:&str){
	con.set_fill_style(&JsValue::from(color));
	con.fill_rect(x, y, width, height);
}