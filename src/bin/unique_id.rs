use froth::{run, Body, GenericNode, Message};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UniquePayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Generate,
    GenerateOk {
        id: String,
    },
}

struct UniqueNode {
    msg_id: usize,
}

impl GenericNode<UniquePayload> for UniqueNode {
    fn process(&mut self, src_msg: Message<UniquePayload>) -> Vec<Message<UniquePayload>> {
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

impl UniqueNode {
    fn new() -> Self {
        Self { msg_id: 0 }
    }

    pub fn inc_id(&mut self) {
        self.msg_id += 1;
    }
}

impl UniquePayload {
    fn gen_msg_payload(&self) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Generate => Some(Self::GenerateOk {
                id: Ulid::new().to_string(),
            }),
            _ => None,
        }
    }
}

fn main() -> anyhow::Result<()> {
    run::<UniqueNode, UniquePayload>(UniqueNode::new())
}
