use downdraft::{Body, Message, Node};
use serde::{Deserialize, Serialize};

fn main() {
    let mut node = Node::initialize();

    while let Some(message) = node.accept::<Payload>() {
        let Payload::Echo { echo } = message.body.payload else {
            panic!("Message payload should be right payload")
        };

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(node.next_message_id()),
                in_reply_to: message.body.message_id,
                payload: Payload::EchoOk { echo },
            },
        };

        node.send(&reply)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}
