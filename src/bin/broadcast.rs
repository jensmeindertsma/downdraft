use std::{
    collections::{HashMap, HashSet},
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use downdraft::{Body, Message, Node, NodeId};
use serde::{Deserialize, Serialize};

fn main() {
    let mut node = Node::initialize();
    let mut seen: HashSet<usize> = HashSet::new();
    let mut topology: Option<HashMap<NodeId, Vec<NodeId>>> = None;

    let backlog = Arc::new(Mutex::new(HashMap::new()));
    static STOP: AtomicBool = AtomicBool::new(false);

    let thread_backlog = backlog.clone();
    let handle = thread::spawn(move || {
        // Every second, go through the backlog and re-send all the messages.
        while !STOP.load(Relaxed) {
            thread::sleep(Duration::from_secs(1));

            for message in thread_backlog
                .lock()
                .expect("Locking should succeed")
                .values()
            {
                let mut output = io::stdout();
                writeln!(
                    output,
                    "{}",
                    serde_json::to_string(message).expect("Serializing message should succeed")
                )
                .expect("Writing to standard output should succeed");
            }
        }
    });

    while let Some(message) = node.accept::<Payload>() {
        // TODO
        match message.body.payload {
            Payload::Broadcast { value } => {
                let Some(topology) = &topology else {
                    panic!("Topology should be available by first broadcast request")
                };

                eprintln!("Using known topology to broadcast: = {topology:?}");

                let known = !seen.insert(value);

                for neighbor in topology
                    .get(&node.id)
                    .expect("Topology should contain own node ID")
                {
                    // Only broadcast to neighbors if we see the value for the first time.
                    // Otherwise they might send it back to us even though we already knew
                    // the value, and we might send again, in a loop.
                    if !known {
                        let message_id = node.next_message_id();

                        let message = Message {
                            source: node.id.clone(),
                            destination: neighbor.clone(),
                            body: Body {
                                message_id: Some(message_id),
                                in_reply_to: None,
                                payload: Payload::Broadcast { value },
                            },
                        };

                        node.send(&message);

                        // add reply to list of messages to re-send every second.
                        // When they acknowledge we remove from the list by message ID
                        let broadcast_backlog = backlog.clone();
                        broadcast_backlog
                            .lock()
                            .expect("Locking should succeed")
                            .insert(message_id, message);
                    }
                }

                let reply = Message {
                    source: node.id.clone(),
                    destination: message.source,
                    body: Body {
                        message_id: Some(node.next_message_id()),
                        in_reply_to: message.body.message_id,
                        payload: Payload::BroadcastOk,
                    },
                };

                node.send(&reply)
            }
            Payload::BroadcastOk => {
                // We can use here the `in_reply_to` to know the message arrived and we can stop
                // re-sending.
                if let Some(message_id) = message.body.in_reply_to {
                    let broadcast_backlog = backlog.clone();
                    broadcast_backlog
                        .lock()
                        .expect("Locking should succeed")
                        .remove(&message_id);
                } else {
                    panic!("Received broadcastOk without in_reply_to")
                }
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

                node.send(&reply)
            }
            Payload::ReadOk { .. } => {
                panic!("We never asked for a read so why are we getting an okay back?")
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

                node.send(&reply)
            }
            Payload::TopologyOk => {
                panic!("We never asked for a topology so why are we getting an okay back?")
            }
        }
    }

    // kill thread.
    STOP.store(true, Relaxed);
    handle.join().unwrap();
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
