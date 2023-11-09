use std::io::{self, BufRead};

use downdraft::{handle_initialization, send_message, Body, Message};
use serde::{Deserialize, Serialize};

fn main() {
    let mut input = io::stdin().lock().lines();
    let mut output = io::stdout().lock();

    let (node_id, mut next_message_id) =
        handle_initialization(&mut input, &mut output).expect("initialization should succeed");

    let mut id = 0;

    while let Some(message) = input
        .next()
        .map(|item| item.expect("next item to be read successfully"))
        .map(|line| {
            serde_json::from_str::<Message<Payload>>(&line)
                .expect("message to be deserialized successfully")
        })
    {
        let Payload::Generate = message.body.payload else {
            panic!("Message payload should be right payload")
        };

        let reply = Message {
            source: node_id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(next_message_id.next_id()),
                in_reply_to: message.body.message_id,
                payload: Payload::GenerateOk {
                    id: format!("{node_id}-{id}",),
                },
            },
        };

        id += 1;

        send_message(&mut output, reply).expect("sending of reply to succeed")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk { id: String },
}
