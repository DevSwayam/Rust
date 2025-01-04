use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() {
    // Check if the correct number of arguments is provided
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <source_file> <target_file>");
        return;
    }

    // Extract the source and target file paths from arguments
    let source_file = &args[1];
    let target_file = &args[2];

    // Open the source file and wrap it in a buffered reader
    let source = match File::open(source_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open source file '{}': {}", source_file, err);
            return;
        }
    };
    let mut reader = BufReader::new(source);

    // Create the target file for the compressed output
    let target = match File::create(target_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create target file '{}': {}", target_file, err);
            return;
        }
    };

    /*
        Compression Levels in GzEncoder
    The compression level determines the trade-off between speed and compression ratio:

    Compression::fast(): Minimal compression but faster processing.
    Compression::default(): Balanced compression and speed (default level).
    Compression::best(): Maximum compression but slower processing.
    The choice depends on the use case:

    Use fast() for quick results when size reduction isn't critical.
    Use best() for maximum size reduction, especially for large files.
     */
    // Initialize the GzEncoder with default compression level
    let mut encoder = GzEncoder::new(target, Compression::best());

    // Record the start time of the compression process
    let start_time = Instant::now();

    // Copy data from the source file to the encoder, compressing it on the fly
    if let Err(err) = io::copy(&mut reader, &mut encoder) {
        eprintln!("Failed to compress file: {}", err);
        return;
    }

    // Finish the encoding process and get the compressed output file
    let compressed_file = match encoder.finish() {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to finalize compression: {}", err);
            return;
        }
    };

    // Record the end time of the compression process
    let duration = start_time.elapsed();

    // Get the sizes of the source and compressed files
    let source_size = match reader.get_ref().metadata() {
        Ok(metadata) => metadata.len(),
        Err(err) => {
            eprintln!("Failed to get metadata for source file: {}", err);
            return;
        }
    };
    let compressed_size = match compressed_file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(err) => {
            eprintln!("Failed to get metadata for compressed file: {}", err);
            return;
        }
    };

    // Output compression statistics
    println!("Compression completed in {:.2?}", duration);
    println!("Source file size: {} bytes", source_size);
    println!("Compressed file size: {} bytes", compressed_size);

    if compressed_size >= source_size {
        println!("Warning: The compressed file is larger than the source file.");
    } else {
        println!(
            "Compression ratio: {:.2}%",
            (compressed_size as f64 / source_size as f64) * 100.0
        );
    }
}
