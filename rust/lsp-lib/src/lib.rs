#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate chrono;
extern crate fern;

extern crate jsonrpc_lite;
extern crate languageserver_types as lsp_types;
extern crate serde;

extern crate url;
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;
extern crate xi_rpc;

use xi_plugin_lib::mainloop;
use xi_plugin_lib::Plugin;

pub mod conversion_utils;
pub mod language_server_client;
pub mod lsp_plugin;
pub mod parse_helper;
mod result_queue;
pub mod types;
mod utils;
pub use crate::lsp_plugin::LspPlugin;
pub use crate::types::Config;

pub fn start_mainloop<P: Plugin>(plugin: &mut P) {
    mainloop(plugin).unwrap();
}
