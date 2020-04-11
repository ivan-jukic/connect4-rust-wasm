# Connect4 > Rust + WebAssembly + Elm

Connect4 webapp game that has an AI and parts of its logic implemented in `Rust`
lang, exposed via `WebAssembly` or `WASM` to our `Typescript` code, which is
then further exposed via ports to the `Elm` code used to render the UI.


## Play in console! 

The first step was to implement working console application with Minimax algorithm! You may play against "AI" in console.

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

You should be able to play in console against AI opponent that uses a
[minimax](https://en.wikipedia.org/wiki/Minimax) algorithm to determine which
move should be its next. To change the difficulty modify the `run` method in the `connect4` module. There's a few levels of difficulty available listed in
`enums/difficulty` module.


## In progress...

The next step is to transform `Rust` code from an app into a lib, and expose
functionality through `WASM` for the UI.

To run the `Elm` UI, clone the repository first, and then:
```
yarn install
```

We're using parcel to run the app locally, but we have prepared a simple command
to run the UI:
```
yarn start
```

UI should be available at [http://localhost:3001](http://localhost:3001)!

Next steps are:
- adding WASM support and exposing functionality
- implement communication between the `Rust` and `Elm` code
- build basic UI for two players sitting next to each other, or player vs AI play

Future plans:
- implement `rust` backend
- use websockets to enable "over the internet" multiplayer
- player stats
- graphics (I'm thinking retro at the moment)
- \+ more...
