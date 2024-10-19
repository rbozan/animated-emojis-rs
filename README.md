# Noto Animated emojis

These are all the animated emojis from https://googlefonts.github.io/noto-emoji-animation/ which are licensed under CC BY 4.0, directly useable for your project.


## How to use?

### Rust

```
cargo add animated-emojis-rs
```

### Sparse checkout

Alternatively you can choose to use a sparse checkout to fetch all the Lottie files in this repository.


## Development

### Updating Noto Emoji metadata version

Update the version in `flake.nix`, run a proxy server at `http://127.0.0.1:3128` and run `cargo test`
