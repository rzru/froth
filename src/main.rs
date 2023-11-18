use std::io::{StdoutLock, Write};

use anyhow::Context;

use froth::{Message, Node};

fn main() -> anyhow::Result<()> {
    let mut node = Node::new();

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout: StdoutLock<'_> = std::io::stdout().lock();

    for input in inputs {
        let input = input.context("Maelstrom input could not be deserialised")?;

        if let Some(reply) = node.gen_msg(input) {
            serde_json::to_writer(&mut stdout, &reply).context("serialize response to init")?;
            stdout.write_all(b"\n").context("write trailing newline")?;

            node.inc_id();
        }
    }

    Ok(())
}
