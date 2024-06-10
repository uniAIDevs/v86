use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

/// Optimized I/O operations to enhance performance
pub fn optimized_io_operations(input_path: &str, output_path: &str) -> io::Result<()> {
    // Open the input file in read-only mode
    let input_file = File::open(input_path)?;
    // Create a buffered reader for the input file
    let mut reader = BufReader::new(input_file);

    // Open the output file in write-only mode, create it if it doesn't exist
    let output_file = File::create(output_path)?;
    // Create a buffered writer for the output file
    let mut writer = BufWriter::new(output_file);

    // Buffer to hold data read from the input file
    let mut buffer = vec![0; 8 * 1024]; // 8 KB buffer

    loop {
        // Read a chunk of data into the buffer
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            // End of file reached
            break;
        }
        // Write the chunk of data to the output file
        writer.write_all(&buffer[..bytes_read])?;
    }

    // Ensure all data is flushed to the output file
    writer.flush()?;

    Ok(())
}