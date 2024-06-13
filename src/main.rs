use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::PathBuf;

struct MnistDataLoader {
    training_images_path: PathBuf,
    training_labels_path: PathBuf,
    testing_images_path: PathBuf,
    testing_labels_path: PathBuf,
}

impl MnistDataLoader {
    fn new(
        training_images_path: PathBuf,
        training_labels_path: PathBuf,
        testing_images_path: PathBuf,
        testing_labels_path: PathBuf,
    ) -> Self {
        MnistDataLoader {
            training_images_path,
            training_labels_path,
            testing_images_path,
            testing_labels_path,
        }
    }

    fn read_image_data(
        &self,
        images_path: &PathBuf,
        labels_path: &PathBuf,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut buffer = [0u8; 16];
        let mut file = File::open(&images_path)?;
        file.read_exact(&mut buffer)?;
        let magic = u32::from_be_bytes((&buffer[0..4]).try_into().unwrap());
        if magic != 2051 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("incorrect header, found: {}", magic),
            ));
        }
        let mut image_buffer = Vec::new();
        file.read_to_end(&mut image_buffer)?;

        let mut buffer = [0u8; 8];
        let mut file = File::open(&labels_path)?;
        file.read_exact(&mut buffer)?;
        let magic = u32::from_be_bytes((&buffer[0..4]).try_into().unwrap());
        if magic != 2049 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("incorrect header, found: {}", magic),
            ));
        }
        let mut label_buffer = Vec::new();
        file.read_to_end(&mut label_buffer)?;
        Ok((image_buffer, label_buffer))
    }

    fn load_data(&self) -> ((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>)) {
        let (x_train, y_train) = self
            .read_image_data(&self.training_images_path, &self.training_labels_path)
            .unwrap();
        let (x_rest, y_test) = self
            .read_image_data(&self.testing_images_path, &self.testing_labels_path)
            .unwrap();
        ((x_train, y_train), (x_rest, y_test))
    }
}

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
}
