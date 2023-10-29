use downdraft::{Body, Message, Node};
use serde::{Deserialize, Serialize};

fn main() {
    let mut node = Node::initialize();
    let mut id = 0;

    while let Some(message) = node.accept::<Payload>() {
        let Payload::Generate = message.body.payload else {
            panic!("Message payload should be right payload")
        };

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(node.next_message_id()),
                in_reply_to: message.body.message_id,
                payload: Payload::GenerateOk {
                    id: format!("{}-{id}", node.id.id()),
                },
            },
        };

        id += 1;

        node.send(reply)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk { id: String },
}
