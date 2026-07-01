include!("file_io.rs");
use std::path::PathBuf;

struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,   // worktree/.rgit
    objects: PathBuf,  // gitdir/objects
}

impl Repository {
    fn new(worktree: PathBuf) -> Self {
        let gitdir = worktree.join(".rgit");
        let objects = gitdir.join("objects");
        Self { worktree, gitdir, objects }
    }
}

struct Config {
    path: PathBuf,     // gitdir/config
}

fn main() {

    let repo = Repository::new(PathBuf::from("."));
    if !repo.gitdir.exists() { 
        std::fs::create_dir(&repo.gitdir).expect("Failed to create .rgit directory"); 
    }    
    if !repo.objects.exists() { 
        std::fs::create_dir(&repo.objects).expect("Failed to create .rgit/objects directory");
    }

    // read file and convert to blob
    let file_path = "./src/main.rs";
    let blob = convertToBlob(file_path);
    println!("Blob hash: {}", blob.hash());

    // write blob data to .rgit/objects/{hash}
    let blob_hash = blob.hash();
    let blob_path = format!("{}/{}", repo.objects.display(), blob_hash);
    writeFile(&blob_path, &blob.data);  
}
