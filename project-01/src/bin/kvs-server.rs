use clap::{crate_authors, arg_enum, crate_version, App, Arg, value_t_or_exit};
use kvs::{Result};
use std::net::SocketAddr;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Debug)]
    pub enum KvsEngine {
        KVS,
        SLED,
    }
}

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .about("store key value")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .possible_values(&KvsEngine::variants())
                .case_insensitive(true)
                .default_value("kvs")
                .value_name("ENGINE-NAME")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let addr = value_t_or_exit!(matches, "addr", SocketAddr);
    let engine = value_t_or_exit!(matches, "engine", KvsEngine);

    Ok(())
}
