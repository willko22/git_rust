# TODO: Git in Rust Implementation

## Phase 1: Foundation & Init Command

- [x] Set up Rust project structure and modules
- [ ] Define core data structures (Repository, Config)
- [ ] Implement `git init` command
  - [ ] Create .git directory structure (currently `.rgit/objects` is created ad-hoc in `main.rs`, not via a real `init` command)
  - [ ] Initialize config file
  - [ ] Create HEAD reference
- [ ] Wire up CLI argument parsing with `clap` (dependency is added but unused — `main.rs` still hardcodes behavior instead of dispatching subcommands)

## Phase 2: Object Storage & Add/Commit

- [ ] Implement Git object model
  - [x] Blob objects (file contents) — `Blob` struct + `hash()` implemented in `objects.rs`
  - [~] Tree objects (directory structure) — `Tree` struct, `add`/`order`/`getData`/`hash()` implemented, but nothing builds a `Tree` from an actual directory listing yet
  - [ ] Commit objects (snapshots with metadata)
- [x] Implement hashing/serialization for objects — blob & tree hashing done using SHA-256 (git normally uses SHA-1/SHA-256 mode); object header format (`"<type> <size>\0" + data`) matches real git
- [ ] Implement `git add` command
  - [ ] Build staging area (index)
  - [ ] Hash and store blobs (writing to `.rgit/objects/<hash>` is demoed manually in `main.rs`, not as a reusable `add` operation)
- [ ] Implement `git commit` command
  - [ ] Create tree from index
  - [ ] Create commit object
  - [ ] Update HEAD reference

## Phase 3: Display & Log/Status

- [ ] Implement `git log` command
  - [ ] Read commit history from HEAD
  - [ ] Display commits with messages, hashes, timestamps
- [ ] Implement `git status` command
  - [ ] Compare working directory vs index
  - [ ] Compare index vs HEAD
  - [ ] Show modified, staged, untracked files

## Phase 4: Branching & Checkout

- [ ] Implement branch reference system
- [ ] Implement `git branch` command
  - [ ] List branches
  - [ ] Create new branches
- [ ] Implement `git checkout` command
  - [ ] Switch branches
  - [ ] Update working directory
  - [ ] Update HEAD reference

## Optional Enhancements

- [ ] Add error handling and validation
- [ ] Add unit tests
- [ ] Add support for .gitignore
- [ ] Add diff functionality
- [ ] Add merge functionality
