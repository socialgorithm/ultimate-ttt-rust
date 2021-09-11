# Ultimate TTT: Rust
> Ultimate Tick Tack Toe implementation for algorithmic battles & hackathons :)

[![Travis](https://img.shields.io/travis/socialgorithm/ultimate-ttt-rust.svg)](https://travis-ci.org/socialgorithm/ultimate-ttt-rust)
[![Coverage Status](https://coveralls.io/repos/github/socialgorithm/ultimate-ttt-rust/badge.svg?branch=master)](https://coveralls.io/github/socialgorithm/ultimate-ttt-rust?branch=master)
[![Crate](https://img.shields.io/crates/v/ultimate-ttt.svg)](https://crates.io/crates/ultimate-ttt)

This is a Rust implementation of the Ultimate Tic Tac Toe game.

What this package provides is all the required logic for the game, by exposing a simple API.


## Getting Started
1. Download Rust from https://www.rust-lang.org/
2. Install nodejs from https://nodejs.org/en/

## Project Structure
**game_engine** - This module contains the logic necessary for the ultimate tic-tac-toe to operate smoothly
**agents** - This is where you will code your agent. Take a look at the RandomAgent provided for a good starting point. Once you have created your agent in the agents folder, make sure to switch it over in the [main.rs](./src/main.rs) file.

## Running Your Agent
1. First build the project making sure to use release mode:
```bash
cargo build --release
```
2. Then to connect to the game server run:
```
npx @socialgorithm/uabc --host "SERVER_NAME" --lobby "LOBY_NAME" --token "TEAM_NAME" -f "target\release\ultimate_ttt.exe"
```

## [Code documentation](https://socialgorithm.org/ultimate-ttt-rust/ultimate_ttt/index.html)
