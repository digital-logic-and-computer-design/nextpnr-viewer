use crate::{
    architecture::ECP5Arch, chipdb::ecp5::get_chipdb, decal::ECP5DecalID, renderer::Renderer,
};

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct ViewerECP5 {
    renderer: Renderer<'static, ECP5DecalID>,
}

#[wasm_bindgen]
impl ViewerECP5 {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, chipdata: &[u8]) -> Result<Self, JsError> {
        let db = match get_chipdb(chipdata) {
            Ok(db) => db,
            Err(e) => return Err(JsError::from(&*e)),
        };

        let arch = ECP5Arch::new(db);

        let renderer = match Renderer::new(canvas, arch) {
            Ok(r) => r,
            Err(e) => return Err(JsError::from(&*e)),
        };

        return Ok(Self { renderer });
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsError> {
        self.renderer.create_graphic_elements();
        self.renderer.update_webgl_elements();
        self.renderer.render();

        return Ok(());
    }

    #[wasm_bindgen]
    pub fn zoom(&mut self, amt: f32, x: f32, y: f32) {
        self.renderer.zoom(amt, x, y);
    }
}
