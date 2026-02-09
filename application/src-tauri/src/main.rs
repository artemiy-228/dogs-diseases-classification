mod classifier;
use crate::classifier::DiseasesClassifier;
use anyhow::Result;
use ndarray::Array2;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[tauri::command]
fn predict_diseases(data: Vec<f32>) -> Result<HashMap<String, f32>, String> {
    let input = Array2::from_shape_vec((1, data.len()), data).map_err(|e| e.to_string())?;

    let mut model = MODEL.lock().map_err(|e| e.to_string())?;
    model.predict(input).map_err(|e| e.to_string())
}

static MODEL: Lazy<Mutex<DiseasesClassifier>> = Lazy::new(|| {
    let model =
        DiseasesClassifier::new("diseases_classification.onnx").expect("Failed to load model");
    Mutex::new(model)
});

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![predict_diseases])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
