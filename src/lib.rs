use std::io::{StdoutLock, Write};

use anyhow::Context;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Payload
where
    Self: Sized,
{
    fn gen_msg_payload(&self) -> Option<Self>;
}

pub struct Node {
    msg_id: usize,
    node_id: String,
}

impl Node {
    pub fn new() -> Self {
        Self {
            msg_id: 0,
            node_id: String::new(),
        }
    }

    pub fn process<P>(&mut self, src_msg: Message<P>) -> Option<Message<P>>
    where
        P: Payload,
    {
        let reply = src_msg.body.payload.gen_msg_payload();

        if let Some(payload) = reply {
            return Some(Message {
                src: src_msg.dest,
                dest: src_msg.src,
                body: Body {
                    msg_id: Some(self.msg_id),
                    in_reply_to: src_msg.body.msg_id,
                    payload,
                },
            });
        }

        None
    }

    pub fn inc_id(&mut self) {
        self.msg_id += 1;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message<P> {
    src: String,
    dest: String,
    body: Body<P>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Body<P> {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: P,
}

pub fn run<P>() -> anyhow::Result<()>
where
    P: DeserializeOwned + Payload + Serialize,
{
    let mut node = Node::new();

    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<P>>();

    let mut stdout: StdoutLock<'_> = std::io::stdout().lock();

    for input in inputs {
        let input = input.context("Maelstrom input could not be deserialised")?;

        if let Some(reply) = node.process(input) {
            serde_json::to_writer(&mut stdout, &reply).context("serialize response to init")?;
            stdout.write_all(b"\n").context("write trailing newline")?;

            node.inc_id();
        }
    }

    Ok(())
}
