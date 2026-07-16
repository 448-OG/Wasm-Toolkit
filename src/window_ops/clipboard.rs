use wasm_bindgen_futures::JsFuture;

use crate::{WasmToolkitError, WasmToolkitResult, WasmWindow};

impl WasmWindow {
    pub async fn copy_to_clipboard(&self, content: &str) -> WasmToolkitResult<()> {
        let pending: JsFuture = self.navigator().clipboard().write_text(content).into();

        pending.await.map_err(|error| {
            WasmToolkitError::parse_js_error(error, "Unable to copy contents to clipboard")
        })?;

        Ok(())
    }

    pub async fn read_clipboard(&self) -> WasmToolkitResult<String> {
        let pending: JsFuture = self.navigator().clipboard().read_text().into();

        let outcome = pending.await.map_err(|error| {
            WasmToolkitError::parse_js_error(error, "Unable to read the contents in the clipboard")
        })?;

        outcome.as_string().ok_or(WasmToolkitError::JsError {
            name: "Not a String".to_string(),
            message: outcome.js_typeof().as_string().unwrap_or_default(),
        })
    }
}
