pub mod networks;

pub mod prelude {
  pub use {
    ndarray::{Array1, Array2, Axis},
    rand::prelude::*,
    rand_distr::Normal,
  };
}
