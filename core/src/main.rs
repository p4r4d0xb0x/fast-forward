use {
  ndarray::{Array1, Array2, Axis},
  rand::prelude::*,
  rand_distr::Normal,
};

// ✅ 1. 신경망 구조 정의
#[derive(Debug)]
struct FFNetwork {
  weights: Array2<f64>, // 가중치 행렬 (Weight Matrix)
}

impl FFNetwork {
  // ✅ 신경망 초기화
  fn new(input_size: usize, output_size: usize) -> Self {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let weights =
      Array2::<f64>::from_shape_fn((output_size, input_size), |_| {
        normal.sample(&mut rand::rng())
      });

    FFNetwork { weights }
  }

  // ✅ 순전파 (Forward Pass)
  fn forward(&self, input: &Array1<f64>) -> f64 {
    let output = self.weights.dot(input);
    output.sum() // 활성화 값 (Sum of activations)
  }

  // ✅ FF 알고리즘을 사용한 학습
  fn train(
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

// ✅ 2. 데이터 생성
fn generate_data(samples: usize, input_size: usize) -> Vec<Array1<f64>> {
  let normal = Normal::new(0.0, 1.0).unwrap();
  (0..samples)
    .map(|_| {
      Array1::<f64>::from_shape_fn(input_size, |_| {
        normal.sample(&mut rand::thread_rng())
      })
    })
    .collect()
}

pub fn main() {
  let log4rs_config_path = std::path::Path::new("./log4rs.yml");
  log4rs::init_file(log4rs_config_path, Default::default()).unwrap();
  let input_size = 10000;
  let output_size = 100;
  let learning_rate: f64 = 0.000001;
  let epochs = 100000;

  // ✅ 3. 신경망 생성
  let mut network = FFNetwork::new(input_size, output_size);


  // ✅ 4. "좋은 데이터" 와 "나쁜 데이터" 생성
  let good_data = generate_data(100, input_size); // 정상 데이터
  let bad_data = generate_data(100, input_size).iter()
        .map(|x| x * 2.0) // 나쁜 데이터(변형된 데이터)
        .collect::<Vec<_>>();

  // ✅ 5. 학습 실행
  network.train(&good_data, &bad_data, learning_rate, epochs);
}
