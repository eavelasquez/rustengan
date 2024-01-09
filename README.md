# Rustengan

The [fly.io](https://fly.io/dist-sys/) distributed systems challenges solved in Rust.

To run the tests, you need to have [maelstrom](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3) installed. Then, you can run the tests with the following command. Make sure you have the `rustengan` binary: 

```bash
cargo build
# echo
~/maelstrom/maelstrom test -w echo --bin target/debug/rustengan --node-count 1 --time-limit 10
# unique id generation
~/maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
# broadcast
~/maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10
```
