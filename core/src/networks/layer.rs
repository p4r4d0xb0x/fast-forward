use crate::prelude::*;

// ✅ 1. 신경망 구조 정의
#[derive(Debug)]
pub struct FFNetwork {
  weights: Array2<f64>, // 가중치 행렬 (Weight Matrix)
}

impl FFNetwork {
  // ✅ 신경망 초기화
  pub fn new(input_size: usize, output_size: usize) -> Self {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let weights =
      Array2::<f64>::from_shape_fn((output_size, input_size), |_| {
        normal.sample(&mut rand::rng())
      });

    FFNetwork { weights }
  }

  // ✅ 순전파 (Forward Pass)
  pub fn forward(&self, input: &Array1<f64>) -> f64 {
    let output = self.weights.dot(input);
    output.sum() // 활성화 값 (Sum of activations)
  }

  // ✅ FF 알고리즘을 사용한 학습
  pub fn train(
    &mut self,
    good_data: &Vec<Array1<f64>>,
    bad_data: &Vec<Array1<f64>>,
    learning_rate: f64,
    epochs: usize,
  ) {
    for epoch in 0..epochs {
      let mut good_activation = 0.0;
      let mut bad_activation = 0.0;

      // 좋은 데이터 학습
      for input in good_data {
        good_activation += self.forward(input);
      }
      good_activation /= good_data.len() as f64;

      // 나쁜 데이터 학습
      for input in bad_data {
        bad_activation += self.forward(input);
      }
      bad_activation /= bad_data.len() as f64;

      // 손실 함수: 좋은 데이터는 활성화 높이고, 나쁜 데이터는 낮추기
      let loss = -(good_activation - bad_activation);

      // 가중치 업데이트 (FF 방식)
      let grad = good_activation - bad_activation;
      self.weights.mapv_inplace(|w| w - learning_rate * grad);

      if epoch % 10 == 0 {
        println!("Epoch {}: Loss = {:.4}", epoch, loss);
      }
    }
  }
}
