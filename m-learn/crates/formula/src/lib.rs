//! Mathematical functions for data analysis and machine learning
//!
//! This crate provides pure mathematical functions designed for seamless integration
//! with tensor operations via the `tensor::map()` callback pattern.

pub mod normal;

pub use normal::{box_muller, normal_distribution};
