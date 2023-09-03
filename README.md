# novision

Visual cryptography tool written in rust

---

### About

This tool takes a trasparent png image (can be grayscale colors are removed during encryption) and outputs two images that when layed on top of each other, the patter from original one can be seen.

Just take a look at example below



### Usage

```bash
cargo run -- testdata/x.png
```

### Building and installing

```bash
cargo build --release
```

```bash
sudomv target/release/novision /usr/local/bin
```


