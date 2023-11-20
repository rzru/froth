use std::collections::HashMap;

use froth::{run, Body, GenericNode, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum BroadcastPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Broadcast {
        message: isize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<isize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

impl BroadcastPayload {
    fn gen_msg_payload(&self, state: &BroadcastState) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Broadcast { .. } => Some(Self::BroadcastOk),
            Self::Read => Some(Self::ReadOk {
                messages: state.messages.clone(),
            }),
            Self::Topology { .. } => Some(Self::TopologyOk),
            _ => None,
        }
    }
}

struct BroadcastState {
    node_id: String,
    messages: Vec<isize>,
    topology: HashMap<String, Vec<String>>,
}

impl BroadcastState {
    fn new() -> Self {
        Self {
            node_id: String::new(),
            messages: vec![],
            topology: HashMap::new(),
        }
    }

    fn update(&mut self, payload: &BroadcastPayload) {
        match payload {
            BroadcastPayload::Init { node_id, .. } => self.node_id = node_id.clone(),
            BroadcastPayload::Broadcast { message } => self.messages.push(*message),
            BroadcastPayload::Topology { topology } => self.topology = topology.clone(),
            _ => {}
        }
    }
}

struct BroadcastNode {
    msg_id: usize,
    state: BroadcastState,
}

impl BroadcastNode {
    fn new() -> Self {
        Self {
            msg_id: 0,
            state: BroadcastState::new(),
        }
    }

    pub fn inc_id(&mut self) {
        self.msg_id += 1;
    }
}

impl GenericNode<BroadcastPayload> for BroadcastNode {
    fn process(&mut self, src_msg: Message<BroadcastPayload>) -> Vec<Message<BroadcastPayload>> {
        let mut result = vec![];

        let payload = src_msg.body.payload;

        // Broadcast is acknowledged if the message is already present in the state
        let is_acknowledged = match &payload {
            BroadcastPayload::Broadcast { message } => self.state.messages.contains(message),
            _ => false,
        };

        // We don't need to process already acknowledged broadcast
        if is_acknowledged {
            return result;
        }

        self.state.update(&payload);

        if let Some(payload) = payload.gen_msg_payload(&self.state) {
            result.push(Message {
                src: src_msg.dest,
                dest: src_msg.src.clone(),
                body: Body {
                    msg_id: Some(self.msg_id),
                    in_reply_to: src_msg.body.msg_id,
                    payload,
                },
            });

            self.inc_id();
        }

        // If the message is broadcast we need to propagate it to the nearest nodes
        match payload {
            BroadcastPayload::Broadcast { .. } => {
                if let Some(neighbours) = self.state.topology.get(&self.state.node_id) {
                    for neghbour in neighbours {
                        result.push(Message {
                            src: self.state.node_id.clone(),
                            dest: neghbour.clone(),
                            body: Body {
                                msg_id: Some(self.msg_id),
                                in_reply_to: None,
                                payload: payload.clone(),
                            },
                        });
                    }

                    self.inc_id();
                }
            }
            _ => {}
        }

        result
    }
}

fn main() -> anyhow::Result<()> {
    run::<BroadcastNode, BroadcastPayload>(BroadcastNode::new())
}
