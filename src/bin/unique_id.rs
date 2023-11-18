use froth::{run, DummyState, Payload};
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

impl Payload<DummyState> for UniquePayload {
    fn gen_msg_payload(&self, _: &DummyState) -> Option<Self> {
        match self {
            Self::Init { .. } => Some(Self::InitOk),
            Self::InitOk => panic!("shouldn't receive init_ok"),
            Self::Generate => Some(Self::GenerateOk {
                id: Ulid::new().to_string(),
            }),
            _ => None,
        }
    }

    fn modify_state(&self, _: &mut DummyState) {}
}

fn main() -> anyhow::Result<()> {
    run::<DummyState, UniquePayload>(DummyState)
}
