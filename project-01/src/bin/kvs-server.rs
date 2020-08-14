use clap::{arg_enum, crate_authors, crate_version, value_t_or_exit, App, Arg};
use kvs::thread_pool::ThreadPool;
use kvs::{KvStore, KvsServer, Result, SledKvsEngine};
use slog::*;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Debug)]
    pub enum KvsEngineType {
        kvs,
        sled,
    }
}

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
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

    let pool = kvs::thread_pool::SharedQueueThreadPool::new(4).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
        .expect("Error setting Ctrl-C handler");
    let s = match engine_type {
        KvsEngineType::sled => {
            KvsServer::new(SledKvsEngine::open("./")?, pool).run(addr, server)?
        }
        KvsEngineType::kvs => KvsServer::new(KvStore::open("./")?, pool).run(addr, server)?,
    };
    while running.load(Ordering::Relaxed) {}
    info!(root, "stopping server...");
    s.do_shutdown().unwrap();
    info!(root, "stopped");
    std::thread::sleep(Duration::from_millis(1000));

    Ok(())
}