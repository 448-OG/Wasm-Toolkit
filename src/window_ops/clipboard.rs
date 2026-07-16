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
}
