# TODO: Git in Rust Implementation

## Phase 1: Foundation & Init Command

- [x] Set up Rust project structure and modules
- [ ] Define core data structures (Repository, Config)
- [ ] Implement `git init` command
  - [ ] Create .git directory structure
  - [ ] Initialize config file
  - [ ] Create HEAD reference

## Phase 2: Object Storage & Add/Commit

- [ ] Implement Git object model
  - [ ] Blob objects (file contents)
  - [ ] Tree objects (directory structure)
  - [ ] Commit objects (snapshots with metadata)
- [ ] Implement hashing/serialization for objects
- [ ] Implement `git add` command
  - [ ] Build staging area (index)
  - [ ] Hash and store blobs
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
