use serde::{Deserialize, Serialize};

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

    pub fn gen_msg(&self, src_msg: Message) -> Option<Message> {
        if let Some(payload) = src_msg.body.payload.gen_msg_payload() {
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
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Body {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
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

impl Payload {
    fn gen_msg_payload(&self) -> Option<Payload> {
        match self {
            Payload::Init { .. } => Some(Payload::InitOk),
            Payload::InitOk => panic!("shouldn't receive init_ok"),
            Payload::Echo { echo } => Some(Payload::EchoOk {
                echo: echo.to_string(),
            }),
            Payload::EchoOk { .. } => None,
        }
    }
}
