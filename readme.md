# Connect4

---

#### **_Rust + WebAssembly + Elm_**

This project implements a **`Connect4`** game, using `Rust` for the `minimax` algorithm that acts as the app's backend, `Elm` for the "in browser" game UI, and connects these two parts using the `WebAssembly`.

To run the code, you will need to have `Nix` installed, as it depends on the `Nix package manager` to provide the required packages and binaries to compile and run the code.

## Play in console!

If you're interested in trying it out, clone the repository

```
git clone git@github.com:ivan-jukic/connect4-rust-wasm.git
```

And checkout to the following tag

```
git checkout pre-wasm
```

Now just run the code via (assuming you have `cargo` installed)

```
cargo run
```

You should be able to play in console against an "AI" opponent that uses a [minimax](https://en.wikipedia.org/wiki/Minimax) algorithm to determine which move should be its next. To change the difficulty modify the `run` method in the `connect4` module. There's a few levels of difficulty available listed in `enums/difficulty` module.
