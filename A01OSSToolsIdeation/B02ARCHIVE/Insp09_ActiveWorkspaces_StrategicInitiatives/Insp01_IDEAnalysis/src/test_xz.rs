use std::fs::File;
use std::io::Read;
use xz2::read::XzDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/tmp/control.tar.xz")?;
    let mut decoder = XzDecoder::new(file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    
    println!("Decompressed {} bytes", decompressed.len());
    
    // Try to parse as tar
    let mut archive = tar::Archive::new(&decompressed[..]);
    let entries = archive.entries()?;
    
    for entry in entries {
        let entry = entry?;
        println!("Entry: {:?}", entry.path()?);
    }
    
    Ok(())
}