include!("objects.rs");

fn readFile(path: &str) -> Vec<u8> {
    return std::fs::read(path).expect("Failed to read file");
}

fn convertToBlob(path: &str) -> Blob {
    let data = readFile(path);
    return Blob::new(data);
}

fn writeFile(path: &str, data: &[u8]) {
    std::fs::write(path, data).expect("Failed to write file");
}