pub mod mnist {
    use std::path::PathBuf;
    use std::io::{Error, ErrorKind, Read, Result};
    use std::fs::File;

    pub struct MnistDataLoader {
        pub training_images_path: PathBuf,
        pub training_labels_path: PathBuf,
        pub testing_images_path: PathBuf,
        pub testing_labels_path: PathBuf,
    }

    impl MnistDataLoader {
        pub fn new(
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

        pub fn read_image_data(
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

        pub fn load_data(&self) -> ((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>)) {
            let (x_train, y_train) = self
                .read_image_data(&self.training_images_path, &self.training_labels_path)
                .unwrap();
            let (x_rest, y_test) = self
                .read_image_data(&self.testing_images_path, &self.testing_labels_path)
                .unwrap();
            ((x_train, y_train), (x_rest, y_test))
        }
    }

    pub fn log_image(data:  &Vec<u8>, labels: &Vec<u8>, offset: usize) {
        const IMG_SIZE: usize = 28*28;
        let mut buffer = [4u8; IMG_SIZE];
        for i in offset*IMG_SIZE..(offset+1)*IMG_SIZE {
            buffer[i % 784] = *data.get(i).unwrap()
        }
        let img: String = buffer.iter().enumerate().map(|(idx, x)| {
            let val = if *x == 0 {"·"} else { "■"};
            if (idx + 1) % 28 == 0 {
                return format!(" {}\n", val)
            }
            format!(" {val} ")
        }).collect();
        println!("{img}");
        println!("Label: {}", labels.get(offset).unwrap());
    }
}