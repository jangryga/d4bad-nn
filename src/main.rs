use d4bad_nn::mnist::{MnistDataLoader, log_image};
use std::path::PathBuf;


#[allow(unused_variables)]
fn main() {
    let data_dir = PathBuf::from("./dataset/mnist-archive");
    let training_images_path = data_dir.join("train-images-idx3-ubyte/train-images-idx3-ubyte");
    let training_labels_path = data_dir.join("train-labels-idx1-ubyte/train-labels-idx1-ubyte");
    let testing_images_path = data_dir.join("t10k-images-idx3-ubyte/t10k-images-idx3-ubyte");
    let testing_labels_path = data_dir.join("t10k-labels-idx1-ubyte/t10k-labels-idx1-ubyte");

    let data_loader = MnistDataLoader::new(
        training_images_path,
        training_labels_path,
        testing_images_path,
        testing_labels_path,
    );
    let ((x_train, y_train), (x_rest, y_test)) = data_loader.load_data();
    log_image(&x_train, &y_train, 2);
}
