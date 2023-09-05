# novision

Visual cryptography tool written in rust

---

### About

This tool takes a trasparent png image (can be grayscale colors are removed during encryption) and outputs two images that when layed on top of each other, the patter from original one can be seen.

Just take a look at example below

![Zrzut ekranu 2023-09-3 o 20 24 33](https://github.com/piotreknow02/novision/assets/65082017/1242d433-f946-495e-9e13-6d3b48cdfc73)

### Usage

```bash
cargo run -- testdata/x.png
```

### Building and installing

```bash
cargo build --release
```

```bash
sudo mv target/release/novision /usr/local/bin
```


