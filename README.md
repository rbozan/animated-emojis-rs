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

1) Update the version in `flake.nix`
2) Run a proxy server at `http://127.0.0.1:3128` by using `docker-compose up`
3) Run `cargo test`. This will do some checks and fetch the emojis.
