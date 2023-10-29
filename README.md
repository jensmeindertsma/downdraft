# downdraft

Implementation of Fly.io's [Gossip Glomers](https://fly.io/dist-sys/) distributed systems challenges. These challenges were meant to be completed using Go but I'm using Rust instead 🦀.

## Setting up

Follow [these instructions](https://github.com/jepsen-io/maelstrom/blob/main/doc/01-getting-ready/index.md) to set up Maelstrom and its dependencies. You can place Maelstrom wherever you like but I just have a `maelstrom` folder with all the unzipped contents in the root of this repository. It is conveniently ignored in the Git ignore file. Once you are able to run the demo, you should be able to run Maelstrom on the binaries in this project.

## Reference material

The full protocol specification can be found [here](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md).

## Running the implementations against Maelstrom

1. First, compile the binaries: `cargo build --release`.
2. Secondly, change into the `maelstrom` directory: `cd maelstrom`
3. Execute one of the commands below to run any binary against Maelstrom

### Challenge 1: Echo

```
./maelstrom test -w echo --bin ../target/release/echo --node-count 1 --time-limit 10
```

### Challenge 2: Unique ID Generation

```
./maelstrom test -w unique-ids --bin ../target/release/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

### Challenge 3a: Single-Node Broadcast

```
./maelstrom test -w broadcast --bin ../target/release/broadcast --node-count 1 --time-limit 20 --rate 10
```

### Challenge 3b: Multi-Node Broadcast

```
./maelstrom test -w broadcast --bin ../target/release/broadcast --node-count 5 --time-limit 20 --rate 10
```

### Challenge 3c: Fault Tolerant Broadcast

```
./maelstrom test -w broadcast --bin ../target/release/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition
```

Other commands will follow as I complete my implementations...
