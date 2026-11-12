Some stuff for solving competitive programming problems in Rust.

Includes pinned Rust toolchains, platform-specific `Cargo.toml` files and code templates.

## Useful bash commands to operate this crap

```sh
cp ../../template_main.rs ./main.rs
```
```sh
cat input.txt | cargo run > output.txt && diff output.txt expected_output.txt
```

Speeding up the copypaste of testcases. I'm on wayland, hence wl-paste.
You can also use neovim/emacs script, or basically anything you fancy.
There's also `online-judge-tools` thingy, and I'm probably gonna be using it soon,
but this is good enough for now.
```sh
wl-paste > input.txt
wl-paste > expected_output.txt
```

You can probably directly invoke rustc, but I'm too lazy to do that.

## Leetcode

Leetcode has its own template file inside `src/`.

It also has different toolchain version, hence its own `devenv` files.
