# Froth - Distributed Systems challenge

## Summary

In this repository lies my attempt to implement distributed systems nodes for [this challenge](https://fly.io/dist-sys/). At the moment the first three nodes are implemented and fully cover usecases 1-3b. I got some of my inspiration from [this](https://www.youtube.com/watch?v=gboGyccRVXI) awesome video (kudos to [@jonhoo](https://github.com/jonhoo)), however only very, very partially, 90% of the code is self-written without any influence.

## How to test


### Prerequisites

- Make sure that you have [Rust](https://www.rust-lang.org/) installed.
- Make sure that you have [Maelstrom](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3) installed somewhere. Installation process is better described [here](https://fly.io/dist-sys/1/)

### Actual process

```bash
cargo b # build the nodes binaries

~/path/to/maelstrom test -w echo --bin ./target/debug/echo -node-count 1 --time-limit 10 # test 1

~/path/to/maelstrom test -w unique-ids --bin target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition # test 2

~/path/to/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10 # test 3a

~/path/to/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10 # test 3b
```

In case it works you should see `Everything looks good! ヽ(‘ー‘)ノ` in the end.
