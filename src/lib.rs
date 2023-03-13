/*A library that returns back random fruit */
use onnxruntime::{
    environment::Environment, tensor::OrtOwnedTensor, GraphOptimizationLevel,
    LoggingLevel,
};
use image::{DynamicImage, GenericImageView, Pixel};
use ndarray::Array3;


//create a function that returns a random fruit
pub fn run(img: &DynamicImage) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let environment = Environment::builder()
        .with_name("test")
        .with_log_level(LoggingLevel::Info)
        .build()?;

    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file("./model.onnx")?;

    let input0_shape: Vec<usize> = session.inputs[0]
        .dimensions()
        .map(|d| d.unwrap())
        .collect();
    let output0_shape: Vec<usize> = session.outputs[0]
        .dimensions()
        .map(|d| d.unwrap())
        .collect();

    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);

    let mut tensor = Array3::<f32>::zeros((3, 224, 224));
    for (x, y, pixel) in resized.pixels() {
        let rgba = pixel.to_rgba();
        tensor[[0, y as usize, x as usize]] = rgba[0] as f32 / 255.0;
        tensor[[1, y as usize, x as usize]] = rgba[1] as f32 / 255.0;
        tensor[[2, y as usize, x as usize]] = rgba[2] as f32 / 255.0;
    }

    let array = tensor.into_shape(input0_shape).unwrap();
    let input_tensor_values = vec![array];

    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor_values.clone())?;

    assert_eq!(outputs[0].shape(), output0_shape.as_slice());
    let output_vec = outputs[0].as_slice().unwrap().to_vec();

    Ok(output_vec)
}
