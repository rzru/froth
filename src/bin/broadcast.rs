use froth::{run, Payload};
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
    Topology,
    TopologyOk,
}

impl Payload<BroadcastState> for BroadcastPayload {
    fn gen_msg_payload(&self, state: &BroadcastState) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Broadcast { .. } => Some(Self::BroadcastOk),
            Self::Read => Some(Self::ReadOk {
                messages: state.messages.clone(),
            }),
            Self::Topology => Some(Self::TopologyOk),
            _ => None,
        }
    }

    fn modify_state(&self, state: &mut BroadcastState) {
        match self {
            Self::Broadcast { message } => state.messages.push(*message),
            _ => {}
        }
    }
}

struct BroadcastState {
    messages: Vec<isize>,
}

fn main() -> anyhow::Result<()> {
    run::<BroadcastState, BroadcastPayload>(BroadcastState { messages: vec![] })
}
