pub mod utils;
pub mod engine; // Core Engine (Renderer, Components, Systems, Input, ECS)

#[cfg(target_arch = "wasm32")]
pub mod game; // Game-specific logic (Zombobs)

#[cfg(target_arch = "wasm32")]
mod wasm_exports {
    use super::game::GameEngine;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen]
    pub fn init_panic_hook() {
        crate::utils::set_panic_hook();
    }

    #[wasm_bindgen]
    pub async fn create_engine(canvas_id: &str) -> Result<GameEngine, JsValue> {
        log("Initializing ZOMBS-ENGINE...");

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("document should have #canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        let engine =
            GameEngine::new(canvas).await.map_err(|e| JsValue::from_str(&e.to_string()))?;

        log("ZOMBS-ENGINE initialized successfully!");
        Ok(engine)
    }
}
