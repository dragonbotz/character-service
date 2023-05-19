# Dragon Bot Z Library
Hello and welcome to the Rust implementation of Dragon Bot Z library.

The main purpose of this library is to ease the backend development of Dragon Bot Z.

It's intended to be easy to use, easy to understand and well documented.

# Usage
In order to properly use this library, you need to set the path to it in your `Cargo.toml` file, like this:
```toml
# In Cargo.toml
# Add the following:
[dependency.dbzlib-rs]
path = "path/to/dbzlib-rs"
```

Then, add the following lines to your code to import what you want. In the following example, we import the Character model:
```rust
use dbzlib_rs::model::character::Character;
```
