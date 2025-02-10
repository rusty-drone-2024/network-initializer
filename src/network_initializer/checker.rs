use crate::factory::{DroneImpl, LeafImpl, NodeId};
use crate::NetworkInitializer;
use std::collections::{HashMap, HashSet};
use wg_2024::config::Config;

type MiniGraph = HashMap<NodeId, HashSet<NodeId>>;

impl NetworkInitializer {
    pub(super) fn check_config(config: &Config) -> Result<(), String> {
        let nodes = Self::get_nodes_from_config(config)?;

        // Check number of neighbour of client is 1 or 2
        // Also check for duplicated connection
        for client in &config.client {
            let nexts = client.connected_drone_ids.len();
            if nexts != 1 && nexts != 2 {
                return Err("Client can be connect to only 1 or 2 nodes".to_string());
            } else if nodes[&client.id].len() < nexts {
                return Err("Duplicated NodeId connection".to_string());
            }
        }

        // Check number of neighbour of server is 2 or more
        // Also check for duplicated connection
        for server in &config.server {
            let nexts = server.connected_drone_ids.len();
            if nexts < 2 {
                return Err("Server must be connected to at least 2 nodes".to_string());
            } else if nodes[&server.id].len() < nexts {
                return Err("Duplicated neighbour".to_string());
            }
        }

        // Check for duplicated connection in drones
        for drone in &config.drone {
            let nexts = drone.connected_node_ids.len();
            if nodes[&drone.id].len() < nexts {
                return Err("Duplicated neighbour".to_string());
            }
        }

        // Check that all nodes connection are simmetrical
        // And that connections exist between valid nodes
        for (id, nexts) in &nodes {
            for next in nexts {
                if id == next {
                    return Err("Node cannot be connected to itself".to_string());
                }
                let next_next = nodes.get(next).ok_or("Connected to not existing node")?;
                next_next.get(id).ok_or("Connection is not symmetrical")?;
            }
        }

        Ok(())
    }

    fn get_nodes_from_config(config: &Config) -> Result<MiniGraph, String> {
        let mut nodes: MiniGraph = HashMap::default();
        for server in &config.server {
            let old = nodes.insert(
                server.id,
                server.connected_drone_ids.iter().copied().collect(),
            );
            if old.is_some() {
                return Err("Duplicated node id".to_string());
            }
        }
        for client in &config.client {
            let old = nodes.insert(
                client.id,
                client.connected_drone_ids.iter().copied().collect(),
            );
            if old.is_some() {
                return Err("Duplicated node id".to_string());
            }
        }
        for drone in &config.drone {
            let old = nodes.insert(drone.id, drone.connected_node_ids.iter().copied().collect());
            if old.is_some() {
                return Err("Duplicated node id".to_string());
            }
        }

        Ok(nodes)
    }

    pub(super) fn check_factories(
        drones: &[DroneImpl],
        clients: &[LeafImpl],
        servers: &[LeafImpl],
    ) -> Result<(), String> {
        if drones.is_empty() || clients.is_empty() || servers.is_empty() {
            return Err("Implementation is empty".to_string());
        }
        Ok(())
    }
}
