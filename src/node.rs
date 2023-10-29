use crate::message::{Body, Message, MessageId};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{self, BufRead, Write};

pub struct Node<'a> {
    pub id: NodeId,

    input: io::Lines<io::StdinLock<'a>>,
    output: io::Stdout,

    next_message_id: MessageId,
}

impl<'a> Node<'a> {
    pub fn initialize() -> Self {
        let mut input = io::stdin().lock().lines();
        let mut output = io::stdout();

        let message: Message<InitializationPayload> = serde_json::from_str(
            &input
                .next()
                .expect("Standard input should hold an initialization message")
                .expect("Reading from standard input should succeed"),
        )
        .expect("Initialization message should be deserializable");

        let InitializationPayload::Receive { node_id, .. } = message.body.payload else {
            panic!("First message should hold initialization payload")
        };

        let reply = Message {
            source: message.destination,
            destination: message.source,
            body: Body {
                message_id: Some(MessageId::new(0)),
                in_reply_to: message.body.message_id,
                payload: InitializationPayload::Acknowledge,
            },
        };

        writeln!(
            output,
            "{}",
            serde_json::to_string(&reply).expect("Serializing message should succeed")
        )
        .expect("Writing to standard output should succeed");

        Self {
            input,
            output,
            id: node_id,
            next_message_id: MessageId::new(1),
        }
    }

    pub fn accept<Payload>(&mut self) -> Option<Message<Payload>>
    where
        Payload: DeserializeOwned,
    {
        self.input.next().map(|message| {
            serde_json::from_str(&message.expect("Reading from standard input should succeed"))
                .expect("Deserializing message should succeed")
        })
    }

    pub fn send<Payload>(&mut self, message: &Message<Payload>)
    where
        Payload: Serialize,
    {
        writeln!(
            self.output,
            "{}",
            serde_json::to_string(message).expect("Serializing message should succeed")
        )
        .expect("Writing to standard output should succeed")
    }

    pub fn next_message_id(&mut self) -> MessageId {
        let id = self.next_message_id;
        self.next_message_id = MessageId::new(id.id() + 1);
        id
    }
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}
