use rust_file_unpacker::{Result, extract_deb_with_depth, cli::Args};
use clap::Parser;

fn main() -> Result<()> {
    // Parse command line arguments
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // Handle clap errors (help, version, invalid args, etc.)
            match e.kind() {
                clap::error::ErrorKind::DisplayHelp | 
                clap::error::ErrorKind::DisplayVersion => {
                    // For help and version, print the message and exit successfully
                    print!("{}", e);
                    std::process::exit(0);
                }
                _ => {
                    // For other errors, print the message and exit with error code
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    };
    
    if args.verbose {
        println!("Rust File Unpacker - .deb extraction tool");
        println!("Input file: {:?}", args.input);
        println!("Output directory: {:?}", args.output);
        println!("Max recursion depth: {}", args.max_depth);
    }
    
    // Verify input file exists
    if !args.input.exists() {
        eprintln!("Error: Input file does not exist: {:?}", args.input);
        eprintln!("Please provide a valid .deb file path.");
        std::process::exit(1);
    }
    
    // Verify input file has .deb extension (basic validation)
    if let Some(extension) = args.input.extension() {
        if extension != "deb"
            && args.verbose {
                println!("Warning: Input file does not have .deb extension, proceeding anyway...");
            }
    } else if args.verbose {
        println!("Warning: Input file has no extension, proceeding anyway...");
    }
    
    if args.verbose {
        println!("Starting extraction...");
    }
    
    // Extract the .deb file
    match extract_deb_with_depth(&args.input, &args.output, args.max_depth) {
        Ok(()) => {
            if args.verbose {
                println!("Extraction completed successfully!");
                println!("Files extracted to: {:?}", args.output);
            } else {
                println!("Extraction completed successfully.");
            }
        }
        Err(e) => {
            eprintln!("Extraction failed: {}", e);
            
            // Provide more helpful error messages based on error type
            match e {
                rust_file_unpacker::ExtractionError::Io(io_err) => {
                    match io_err.kind() {
                        std::io::ErrorKind::NotFound => {
                            eprintln!("Hint: Check that the input file path is correct.");
                        }
                        std::io::ErrorKind::PermissionDenied => {
                            eprintln!("Hint: Check file permissions for input file and output directory.");
                        }
                        _ => {
                            eprintln!("Hint: Check file permissions and available disk space.");
                        }
                    }
                }
                rust_file_unpacker::ExtractionError::PathTraversal { path } => {
                    eprintln!("Security warning: The archive contains a malicious path: {}", path);
                    eprintln!("Extraction was stopped to prevent directory traversal attack.");
                }
                rust_file_unpacker::ExtractionError::Format(msg) => {
                    eprintln!("Archive format error: {}", msg);
                    eprintln!("Hint: Ensure the input file is a valid .deb archive.");
                }
                rust_file_unpacker::ExtractionError::Path(path_err) => {
                    eprintln!("Path validation error: {}", path_err);
                }
                rust_file_unpacker::ExtractionError::RecursionLimitExceeded { depth, limit } => {
                    eprintln!("Recursion limit exceeded: reached depth {} (limit: {})", depth, limit);
                    eprintln!("Hint: Use --max-depth to increase the limit if needed.");
                }
            }
            
            std::process::exit(1);
        }
    }
    
    Ok(())
}
