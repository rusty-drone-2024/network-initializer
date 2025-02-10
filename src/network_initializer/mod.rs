mod checker;
pub mod creator;
mod initializer;
mod parser;
mod public;

use crate::network::Network;
pub use parser::load_from_file;

pub struct NetworkInitializer {
    pub network: Network,
}
