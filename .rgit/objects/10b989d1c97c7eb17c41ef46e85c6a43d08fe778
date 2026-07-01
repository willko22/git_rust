include!("file_io.rs");

fn main() {
    println!("Hello, world!");

    let file_path = "./src/main.rs";
    let blob = convertToBlob(file_path);
    println!("Blob hash: {}", blob.hash());

    // check if folder .rgit exists, if not create it
    let rgit_path = "./.rgit";
    if !std::path::Path::new(rgit_path).exists() {
        std::fs::create_dir(rgit_path).expect("Failed to create .rgit directory");
    }

    // check if folder .rgit/objects exists, if not create it
    let objects_path = "./.rgit/objects";
    if !std::path::Path::new(objects_path).exists() {
        std::fs::create_dir(objects_path).expect("Failed to create .rgit/objects directory");
    }

    // write blob data to .rgit/objects/{hash}
    let blob_hash = blob.hash();
    let blob_path = format!("{}/{}", objects_path, blob_hash);
    writeFile(&blob_path, &blob.data);  
}
