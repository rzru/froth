use std::{
    io::{StdoutLock, Write},
    marker::PhantomData,
};

use anyhow::Context;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct DummyState;

pub trait Payload<S>
where
    Self: Sized,
{
    fn gen_msg_payload(&self, state: &S) -> Option<Self>;
    fn modify_state(&self, state: &mut S);
}

pub struct Node<S, P>
where
    P: Payload<S>,
{
    msg_id: usize,
    state: S,
    phantom: PhantomData<P>,
}

impl<S, P> Node<S, P>
where
    P: Payload<S>,
{
    pub fn new(state: S) -> Self {
        Self {
            msg_id: 0,
            state,
            phantom: PhantomData,
        }
    }

    pub fn process(&mut self, src_msg: Message<P>) -> Option<Message<P>>
    where
        P: Payload<S>,
    {
        src_msg.body.payload.modify_state(&mut self.state);

        let reply = src_msg.body.payload.gen_msg_payload(&self.state);

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

pub fn run<S, P>(state: S) -> anyhow::Result<()>
where
    P: DeserializeOwned + Payload<S> + Serialize,
{
    let mut node = Node::<S, P>::new(state);

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
