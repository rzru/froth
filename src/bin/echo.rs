use froth::{run, DummyState, Payload};
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

impl Payload<DummyState> for EchoPayload {
    fn gen_msg_payload(&self, _: &DummyState) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Echo { echo } => Some(Self::EchoOk {
                echo: echo.to_string(),
            }),
            _ => None,
        }
    }

    fn modify_state(&self, _: &mut DummyState) {}
}

fn main() -> anyhow::Result<()> {
    run::<DummyState, EchoPayload>(DummyState)
}
