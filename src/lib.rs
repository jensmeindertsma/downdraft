mod message;
mod node;

pub use message::{Body, Message, MessageId};
pub use node::{Node, NodeId};
use serde::{Deserialize, Serialize};
use std::io;

pub fn handle_initialization(
    input: &mut io::Lines<impl io::BufRead>,
    output: impl io::Write,
) -> Result<(NodeId, MessageId), &'static str> {
    let message: Message<InitializationPayload> = serde_json::from_str(
        &input
            .next()
            .ok_or("missing first line of input")?
            .map_err(|_| "failed to read first line of input")?,
    )
    .map_err(|_| "initialization message to be deserialized successfully")?;

    let InitializationPayload::Receive { node_id, .. } = message.body.payload else {
        return Err("unexpected initialization message payload");
    };

    let mut next_message_id = MessageId::new(0);

    let reply = Message {
        source: message.destination,
        destination: message.source,
        body: Body {
            message_id: Some(next_message_id.next_id()),
            in_reply_to: message.body.message_id,
            payload: InitializationPayload::Acknowledge,
        },
    };

    send_message(output, reply)?;

    Ok((node_id, next_message_id))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum InitializationPayload {
    #[serde(rename = "init")]
    Receive {
        node_id: NodeId,
        node_ids: Vec<NodeId>,
    },
    #[serde(rename = "init_ok")]
    Acknowledge,
}

pub fn send_message<P: Serialize>(
    mut output: impl io::Write,
    message: Message<P>,
) -> Result<(), &'static str> {
    writeln!(
        output,
        "{}",
        serde_json::to_string(&message).map_err(|_| "failed to serialize message")?
    )
    .map_err(|_| "failed to write message to output buffer")
}
