use std::collections::{HashMap, HashSet};

use downdraft::{Body, Message, Node, NodeId};
use serde::{Deserialize, Serialize};

fn main() {
    let mut node = Node::initialize();
    let mut seen: HashSet<usize> = HashSet::new();
    let mut topology: Option<HashMap<NodeId, Vec<NodeId>>> = None;

    while let Some(message) = node.accept::<Payload>() {
        // TODO
        match message.body.payload {
            Payload::Broadcast { value } => {
                let Some(topology) = &topology else {
                    panic!("Topology should be available by first broadcast request")
                };

                eprintln!("Using known topology to broadcast: = {topology:?}");

                seen.insert(value);

                // for neighbor in topology
                //     .get(&node.id)
                //     .expect("Topology should contain own node ID")
                // {
                //     let message = Message {
                //         source: node.id.clone(),
                //         destination: neighbor.clone(),
                //         body: Body {
                //             message_id: Some(node.next_message_id()),
                //             in_reply_to: None,
                //             payload: Payload::Broadcast { value },
                //         },
                //     };

                //     node.send(message)
                // }

                let reply = Message {
                    source: node.id.clone(),
                    destination: message.source,
                    body: Body {
                        message_id: Some(node.next_message_id()),
                        in_reply_to: message.body.message_id,
                        payload: Payload::BroadcastOk,
                    },
                };

                node.send(reply)
            }
            Payload::Read => {
                let reply = Message {
                    source: node.id.clone(),
                    destination: message.source,
                    body: Body {
                        message_id: Some(node.next_message_id()),
                        in_reply_to: message.body.message_id,
                        payload: Payload::ReadOk {
                            messages: seen.iter().copied().collect(),
                        },
                    },
                };

                node.send(reply)
            }
            Payload::Topology {
                topology: new_topology,
            } => {
                // Store new topology
                topology = Some(new_topology);
                eprintln!("Received topology: {topology:?}");

                let reply = Message {
                    source: node.id.clone(),
                    destination: message.source,
                    body: Body {
                        message_id: Some(node.next_message_id()),
                        in_reply_to: message.body.message_id,
                        payload: Payload::TopologyOk,
                    },
                };

                node.send(reply)
            }
            _ => panic!("Unexpected incoming payload!"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        #[serde(rename = "message")]
        value: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<NodeId, Vec<NodeId>>,
    },
    TopologyOk,
}
