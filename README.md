# Git in Rust

A basic implementation of Git version control system in Rust. This project is designed to learn both how Git works internally and how to write Rust applications.

## Goal

Implement the core Git workflow commands from scratch:
- `init` - Initialize a new repository
- `add` - Stage files for commit
- `commit` - Create snapshots of staged changes
- `log` - View commit history
- `status` - Show repository status
- `branch` - Manage branches
- `checkout` - Switch between branches/commits

## Implementation Order

We'll implement these features in the specified order to build up knowledge progressively:

1. **init** - Create the basic repository structure (.git directory, config files)
2. **add** - Stage files and build the index/staging area
3. **commit** - Create commits with snapshots of staged content
4. **log** - Read and display commit history
5. **status** - Show differences between working directory, index, and HEAD
6. **branch** - Create and manage branch references
7. **checkout** - Switch branches and update the working directory

## Learning Path

- **Phase 1 (init, add, commit)**: Learn Git's storage model, objects (blobs, trees, commits), and the staging area
- **Phase 2 (log, status)**: Learn how to read the repository structure and display state
- **Phase 3 (branch, checkout)**: Learn Git's reference system and branch management

## Project Structure

```
src/
├── main.rs           # CLI entry point and command routing
├── lib.rs            # Core library (if needed)
├── commands/         # Command implementations
├── objects/          # Git object types (blob, tree, commit)
└── repository.rs     # Repository management
```

## Running

```bash
cargo run -- init              # Initialize new repo
cargo run -- add <file>        # Stage a file
cargo run -- commit -m "msg"   # Create a commit
cargo run -- log               # Show commit history
cargo run -- status            # Show repo status
cargo run -- branch            # List/create branches
cargo run -- checkout <ref>    # Switch to branch/commit
```

## Notes

This is an educational project focused on understanding Git internals and learning Rust idioms.
We store everything in .rgit/