use std::io::{StdoutLock, Write};

use anyhow::Context;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message<P> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Body<P> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: P,
}

pub trait GenericNode<P> {
    fn process(&mut self, input: Message<P>) -> Vec<Message<P>>;
}

pub fn run<N, P>(mut node: N) -> anyhow::Result<()>
where
    N: GenericNode<P>,
    P: DeserializeOwned + Serialize,
{
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<P>>();

    let mut stdout: StdoutLock<'_> = std::io::stdout().lock();

    for input in inputs {
        let input = input.context("Maelstrom input could not be deserialised")?;

        for message in node.process(input) {
            serde_json::to_writer(&mut stdout, &message).context("serialize response to init")?;
            stdout.write_all(b"\n").context("write trailing newline")?;
        }
    }

    Ok(())
}
