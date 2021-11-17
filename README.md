# is_available

A CLI tool to check a rust crate is available or not.

### For Unavailable Crate

```sh
cargo run rand

# Output: taken 👎
```

### For Available Crate

```sh
cargo run some_crate_which_is_unavailable

# Output: available 👍
```