use froth::{run, Body, GenericNode, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

struct EchoNode {
    msg_id: usize,
}

impl GenericNode<EchoPayload> for EchoNode {
    fn process(&mut self, src_msg: Message<EchoPayload>) -> Vec<Message<EchoPayload>> {
        let mut result = vec![];

        if let Some(payload) = src_msg.body.payload.gen_msg_payload() {
            result.push(Message {
                src: src_msg.dest,
                dest: src_msg.src,
                body: Body {
                    msg_id: Some(self.msg_id),
                    in_reply_to: src_msg.body.msg_id,
                    payload,
                },
            });
        }

        self.inc_id();
        result
    }
}

impl EchoNode {
    fn new() -> Self {
        Self { msg_id: 0 }
    }

    pub fn inc_id(&mut self) {
        self.msg_id += 1;
    }
}

impl EchoPayload {
    fn gen_msg_payload(&self) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Echo { echo } => Some(Self::EchoOk {
                echo: echo.to_string(),
            }),
            _ => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    run::<EchoNode, EchoPayload>(EchoNode::new())
}
