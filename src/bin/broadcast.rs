use std::{
    collections::{HashMap, HashSet},
    io::{StdoutLock, Write},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use rustengan::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    node: String,
    id: usize,
    messages: HashSet<usize>,
    known: HashMap<String, HashSet<usize>>,
    neighborhood: Vec<String>,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            node: init.node_id,
            id: 1,
            messages: HashSet::new(),
            known: init
                .node_ids
                .into_iter()
                .map(|nid| (nid, HashSet::new()))
                .collect(),
            neighborhood: Vec::new(),
        })
    }

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let mut reply = input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Broadcast { message } => {
                self.messages.insert(message);
                reply.body.payload = Payload::BroadcastOk;
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to broadcast")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }
            Payload::BroadcastOk { .. } => {}
            Payload::Read => {
                reply.body.payload = Payload::ReadOk {
                    messages: self.messages.clone(),
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to read")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }
            Payload::ReadOk { .. } => {}
            Payload::Topology { mut topology } => {
                self.neighborhood = topology
                    .remove(&self.node)
                    .unwrap_or_else(|| panic!("no topology given for node {}", self.node));
                reply.body.payload = Payload::TopologyOk;
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize response to topology")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }
            Payload::TopologyOk => {}
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
