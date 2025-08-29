use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::GzDecoder;

#[derive(Debug)]
pub struct MnistData {
    pub sizes: Vec<u32>,
    pub data: Vec<u8>,
}

impl MnistData {
    pub fn new(path: &str, emnist_rotate: bool) -> io::Result<Self> {
        let path = Path::new(path);
        let file = File::open(path)?;

        let mut reader: Box<dyn Read> = if path.extension().map_or(false, |e| e == "gz") {
            Box::new(GzDecoder::new(file))
        } else {
            Box::new(BufReader::new(file))
        };

        let magic_number = reader.read_u32::<BigEndian>()?;
        
        let mut sizes = Vec::new();
        let mut data = Vec::new();

        if magic_number == 2051 { // Images
            let num_images = reader.read_u32::<BigEndian>()?;
            let num_rows = reader.read_u32::<BigEndian>()?;
            let num_cols = reader.read_u32::<BigEndian>()?;
            sizes.push(num_images);
            sizes.push(num_rows);
            sizes.push(num_cols);
            reader.read_to_end(&mut data)?;
            if emnist_rotate {
                Self::rotate_and_mirror_images(&mut data, num_images as usize, num_rows as usize, num_cols as usize);
            }

        } else if magic_number == 2049 { // Labels
            let num_labels = reader.read_u32::<BigEndian>()?;
            sizes.push(num_labels);
            reader.read_to_end(&mut data)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid magic number {} for file {}", magic_number, path.display())));
        }

        Ok(MnistData { sizes, data })
    }

    fn rotate_and_mirror_images(data: &mut Vec<u8>, num_images: usize, rows: usize, cols: usize) {
        let image_size = rows * cols;
        for i in 0..num_images {
            let start = i * image_size;
            let end = start + image_size;
            let image_data = data[start..end].to_vec();

            let mut rotated_image = vec![0; image_size];
            for r in 0..rows {
                for c in 0..cols {
                    rotated_image[c * rows + r] = image_data[r * cols + c];
                }
            }
            // Reverse rows
            let mut final_image = vec![0; image_size];
            for r in 0..rows {
                for c in 0..cols {
                    final_image[r * cols + c] = rotated_image[(rows - 1 - r) * cols + c];
                }
            }

            data.splice(start..end, final_image);
        }
    }
}