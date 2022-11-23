mod cli;
use cli::{CliOptions, NearCliBackend};
mod config;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    CliOptions::handle::<NearCliBackend>()
}

// fn main() {
//     let options = Options::parse();
//     dotenv().ok();

//     if let Some(command) = options.command {
//         match command {
//             // cargo run --bin seda register-node --socket-address
// 127.0.0.1:9000             Commands::RegisterNode { socket_address } => {
//                 register_node(socket_address).unwrap();
//             }
//             // cargo run --bin seda get-nodes --limit 2
//             Commands::GetNodes { limit, offset } => {
//                 get_nodes(limit, offset).unwrap();
//             }
//             // cargo run --bin seda get-node-socket-address --node-id 9
//             Commands::GetNodeSocketAddress { node_id } => {
//                 get_node_socket_address(node_id).unwrap();
//             }
//             // cargo run --bin seda run
//             Commands::Run => seda_node::run(),
//             // cargo run --bin seda remove-node --node-id 9
//             Commands::RemoveNode { node_id } =>
// remove_node(node_id).unwrap(),             // cargo run --bin seda
// set-node-socket-address --node-id 9 --socket-address 127.0.0.1:9000
//             Commands::SetNodeSocketAddress {
//                 node_id,
//                 socket_address,
//             } => set_node_socket_address(node_id, socket_address).unwrap(),
//             // cargo run --bin seda get-node-owner --node-id 9
//             Commands::GetNodeOwner { node_id } =>
// get_node_owner(node_id).unwrap(),         }
//     } else {
//         todo!()
//     }
// }
