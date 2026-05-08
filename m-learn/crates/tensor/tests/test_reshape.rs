use tensor::Tensor;

#[test]
fn test_squeeze_single_dimension() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0], vec![1, 3]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![3]);
    assert_eq!(squeezed.data(), &vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_squeeze_multiple_dimensions() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0, 4.0], vec![1, 1, 4]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![4]);
    assert_eq!(squeezed.data(), &vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_squeeze_no_change() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![3]);
    assert_eq!(squeezed.data(), &vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_squeeze_2d_no_ones() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![2, 2]);
}

#[test]
fn test_squeeze_scalar() {
    let t = Tensor::build(vec![42.0], vec![1]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![1]);
    assert_eq!(squeezed.data(), &vec![42.0]);
}

#[test]
fn test_squeeze_middle_dimension() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3, 1]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![2, 3]);
}

#[test]
fn test_broadcast_simple() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
    let broadcasted = t.broadcast(&[1, 3]).unwrap();
    assert_eq!(broadcasted.data(), &vec![1.0, 2.0, 3.0]);
    assert_eq!(broadcasted.shape(), &vec![3]);
}

#[test]
fn test_broadcast_to_larger() {
    let t = Tensor::build(vec![1.0, 2.0], vec![2]).unwrap();
    let broadcasted = t.broadcast(&[3, 2]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![3, 2]);
    assert_eq!(broadcasted.data(), &vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0]);
}

#[test]
fn test_broadcast_scalar() {
    let t = Tensor::build(vec![5.0], vec![1]).unwrap();
    let broadcasted = t.broadcast(&[2, 3]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![2, 3]);
    assert_eq!(broadcasted.data(), &vec![5.0; 6]);
}

#[test]
fn test_broadcast_same_shape() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
    let broadcasted = t.broadcast(&[3]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![3]);
    assert_eq!(broadcasted.data(), &vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_broadcast_error_incompatible() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
    let result = t.broadcast(&[2]);
    assert!(result.is_err());
}

#[test]
fn test_broadcast_error_cannot_broadcast() {
    let t = Tensor::build(vec![1.0, 2.0], vec![2]).unwrap();
    let result = t.broadcast(&[3]);
    assert!(result.is_err());
}

#[test]
fn test_broadcast_2d_to_3d() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
    let broadcasted = t.broadcast(&[3, 2, 2]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![3, 2, 2]);
}

#[test]
fn test_broadcast_1d_to_2d() {
    let t = Tensor::build(vec![10.0, 20.0], vec![2]).unwrap();
    let broadcasted = t.broadcast(&[4, 2]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![4, 2]);
}

#[test]
fn test_squeeze_after_broadcast() {
    let t = Tensor::build(vec![1.0, 2.0], vec![2]).unwrap();
    let broadcasted = t.broadcast(&[1, 2]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![2]);
    let squeezed = broadcasted.squeeze();
    assert_eq!(squeezed.shape(), &vec![2]);
}

#[test]
fn test_broadcast_complex_shape() {
    let t = Tensor::build(vec![1.0, 2.0, 3.0, 4.0], vec![1, 4]).unwrap();
    let broadcasted = t.broadcast(&[5, 4]).unwrap();
    assert_eq!(broadcasted.shape(), &vec![5, 4]);
    assert_eq!(broadcasted.data().len(), 20);
}

#[test]
fn test_squeeze_all_ones() {
    let t = Tensor::build(vec![42.0], vec![1, 1, 1]).unwrap();
    let squeezed = t.squeeze();
    assert_eq!(squeezed.shape(), &vec![1]);
    assert_eq!(squeezed.data(), &vec![42.0]);
}
