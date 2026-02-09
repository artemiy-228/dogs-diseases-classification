use anyhow::Result;
use ndarray::Array2;
use ort::{environment::init, inputs, memory::Allocator, session::Session, value::*};
use std::collections::HashMap;

// allocator works with allocation memory to tensor
// model is a prediction model
pub struct DiseasesClassifier {
    allocator: Allocator,
    model: Session,
}

impl DiseasesClassifier {
    pub fn new(model_path: &str) -> Result<Self, anyhow::Error> {
        let allocator = Allocator::default();
        let model = Session::builder()?.commit_from_file(model_path)?;
        Ok(Self { allocator, model })
    }

    pub fn predict(&mut self, input: Array2<f32>) -> Result<HashMap<String, f32>, ort::Error> {
        let tensor = Tensor::from_array(input)?;
        let prediction = &self.model.run(inputs! { "input" => tensor })?;

        let output = prediction.get("output_probability").unwrap();
        let label =
            &(output.try_extract_sequence::<MapValueType<String, f32>>(&self.allocator)?)[0];
        let map: HashMap<String, f32> = label.try_extract_map()?;

        Ok(map)
    }
}
