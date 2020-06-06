use clap::{arg_enum, crate_authors, crate_version, value_t_or_exit, App, Arg};
use kvs::{KvStore, KvsServer, Result, SledKvsEngine};
use slog::*;
use std::net::SocketAddr;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Debug)]
    pub enum KvsEngineType {
        kvs,
        sled,
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
                .possible_values(&KvsEngineType::variants())
                .case_insensitive(true)
                .default_value("kvs")
                .value_name("ENGINE-NAME")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let addr = value_t_or_exit!(matches, "addr", SocketAddr);
    let engine_type = value_t_or_exit!(matches, "engine", KvsEngineType);

    let json = slog_json::Json::default(std::io::stderr()).fuse();
    let drain = slog_async::Async::new(json).build().fuse();

    let root = slog::Logger::root(drain, o!("version" => crate_version!()));

    info!(root, "config" ; "addr" => addr, "engine" => engine_type.to_string());

    info!(root, "starting");

    let server = root.clone();

    match engine_type {
        KvsEngineType::sled => KvsServer::new(SledKvsEngine::open("./")?).run(addr, server)?,
        KvsEngineType::kvs => KvsServer::new(KvStore::open("./")?).run(addr, server)?,
    }

    info!(root, "stopping");
    Ok(())
}
