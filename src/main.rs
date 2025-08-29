mod mnist_loader;
use mnist_loader::MnistData;
use std::io;

const FILE_PATHS: [&str; 4] = [
    "data/train-images-idx3-ubyte",
    "data/train-labels-idx1-ubyte",
    "data/t10k-images-idx3-ubyte",
    "data/t10k-labels-idx1-ubyte",
];

fn main() -> io::Result<()> {
    loop {
        println!("Please select a file to load (or 'q' to quit):");
        println!("a) {}", FILE_PATHS[0]);
        println!("b) {}", FILE_PATHS[2]);

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        if choice == "q" {
            break;
        }

        let file_path = match choice {
            "a" => FILE_PATHS[0],
            "b" => FILE_PATHS[2],
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        let data = match MnistData::new(file_path, false) {
            Ok(d) => d,
            Err(e) => {
                println!("Error loading file: {}", e);
                continue;
            }
        };

        println!("Loaded data from: {}", file_path);
        println!("Number of items: {}", data.sizes[0]);

        loop {
            println!("
Enter the index of the item to display (or 'q' to go back):");
            let mut index_str = String::new();
            io::stdin().read_line(&mut index_str)?;
            let index_str = index_str.trim();

            if index_str == "q" {
                break;
            }

            let index: usize = match index_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number");
                    continue;
                }
            };

            if index >= data.sizes[0] as usize {
                println!("Index out of bounds");
                continue;
            }

            if data.sizes.len() == 3 { // Image data
                let image_size = (data.sizes[1] * data.sizes[2]) as usize;
                let start = index * image_size;
                let end = start + image_size;
                let image_data = &data.data[start..end];

                println!("Displaying image at index {}:", index);
                for y in 0..data.sizes[1] {
                    for x in 0..data.sizes[2] {
                        let pixel = image_data[(y * data.sizes[2] + x) as usize];
                        if pixel > 128 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
            } else { // Label data
                println!("Displaying label at index {}: {}", index, data.data[index]);
            }
        }
    }

    Ok(())
}
