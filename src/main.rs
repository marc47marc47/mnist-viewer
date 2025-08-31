use std::env;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use byteorder::{BigEndian, ReadBytesExt};

mod mnist_loader;
use mnist_loader::MnistData;

fn get_magic_number(path: &Path) -> io::Result<u32> {
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    reader.read_u32::<BigEndian>()
}

fn find_data_files(dir_path: &str) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut images_paths = Vec::new();
    let mut labels_paths = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.to_str().map_or(false, |s| s.ends_with("ubyte")) {
            if let Ok(magic) = get_magic_number(&path) {
                match magic {
                    2051 => images_paths.push(path),
                    2049 => labels_paths.push(path),
                    _ => (),
                }
            }
        }
    }

    if images_paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No image files found."));
    }
    if labels_paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No label files found."));
    }

    images_paths.sort();
    labels_paths.sort();

    Ok((images_paths, labels_paths))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 { &args[1] } else { "./data" };

    loop {
        println!("Searching for data files in: {}", dir_path);

        let (images_paths, labels_paths) = match find_data_files(dir_path) {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Ok(())
            }
        };

        println!("Please select a dataset to load (or 'q' to quit):");
        for (i, path) in images_paths.iter().enumerate() {
            println!("{}) {}", i + 1, path.display());
        }

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str)?;
        let choice_str = choice_str.trim();

        if choice_str == "q" {
            break;
        }

        let choice: usize = match choice_str.parse::<usize>() {
            Ok(num) if num > 0 && num <= images_paths.len() => num - 1,
            _ => {
                eprintln!("Invalid choice.");
                continue;
            }
        };

        let images_path = &images_paths[choice];
        
        let image_stem = images_path.file_stem().unwrap().to_str().unwrap();
        let best_match_label = labels_paths.iter().max_by_key(|p| {
            let label_stem = p.file_stem().unwrap().to_str().unwrap();
            image_stem.chars().zip(label_stem.chars()).take_while(|(a, b)| a == b).count()
        }).unwrap();

        let labels_path = best_match_label;

        println!("Loading image file: {}", images_path.display());
        println!("Loading label file: {}", labels_path.display());

        let image_data = MnistData::new(images_path.to_str().unwrap(), false)?;
        let label_data = MnistData::new(labels_path.to_str().unwrap(), false)?;

        if image_data.sizes[0] != label_data.sizes[0] {
            eprintln!("Warning: Number of images ({}) does not match number of labels ({})", image_data.sizes[0], label_data.sizes[0]);
        }

        println!("Loaded {} images and {} labels.", image_data.sizes[0], label_data.sizes[0]);

        loop {
            println!("\nEnter the index of the item to display (or 'q' to select another file):");
            let mut index_str = String::new();
            io::stdin().read_line(&mut index_str)?;
            let index_str = index_str.trim();

            if index_str == "q" {
                break;
            }

            let index: usize = match index_str.parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number");
                    continue;
                }
            };

            if index >= image_data.sizes[0] as usize {
                println!("Index out of bounds");
                continue;
            }

            if index < label_data.data.len() {
                println!("Label at index {}: {}", index, label_data.data[index]);
            }

            let image_size = (image_data.sizes[1] * image_data.sizes[2]) as usize;
            let start = index * image_size;
            let end = start + image_size;

            if end <= image_data.data.len() {
                let single_image_data = &image_data.data[start..end];
                println!("Displaying image at index {}:", index);
                for y in 0..image_data.sizes[1] {
                    for x in 0..image_data.sizes[2] {
                        let pixel_index = (y * image_data.sizes[2] + x) as usize;
                        let pixel = single_image_data[pixel_index];
                        if pixel > 128 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
            } else {
                println!("Image data for index {} not found.", index);
            }
        }
    }

    Ok(())
}