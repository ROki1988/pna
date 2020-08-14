use criterion::{criterion_group, criterion_main, BatchSize, Criterion, ParameterizedBenchmark};
use kvs::thread_pool::{NaiveThreadPool, ThreadPool};
use kvs::{KvStore, KvsServer};
use rand::prelude::*;
use slog::*;
use std::iter;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use tempfile::TempDir;

fn write_queued_kvstore(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "native",
        |b, _| {
            let addr: SocketAddr = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), 4000);
            let temp_dir = TempDir::new().unwrap();
            let pool = NaiveThreadPool::new(10).unwrap();
            let store = KvStore::open(temp_dir.path()).unwrap();
            let logger = slog::Logger::root(Discard, o!("pool" => "native"));
            let s = KvsServer::new(store, pool).run(addr, logger).unwrap();
            b.iter(|| {
                for i in 1..(1 << 12) {
                    let mut client = kvs::KvsClient::connect(addr).unwrap();
                    client
                        .set(format!("key{}", i), i.to_string())
                        .expect("receive error");
                    client.close().unwrap();
                }
            });
            s.do_shutdown().unwrap();
        },
        iter::once(()),
    )
    .with_function("queue", move |b, _| {
        b.iter(|| {
            let addr: SocketAddr = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), 4001);
            let logger = slog::Logger::root(Discard, o!("pool" => "queue"));
            let temp_dir = TempDir::new().unwrap();
            let pool = NaiveThreadPool::new(10).unwrap();
            let store = KvStore::open(temp_dir.path()).unwrap();
            let s = KvsServer::new(store, pool).run(addr, logger).unwrap();
            for i in 1..(1 << 12) {
                let mut client = kvs::KvsClient::connect(addr).unwrap();
                client.set(i.to_string(), "value".to_string()).unwrap();
                client.close().unwrap();
            }
            s.do_shutdown().unwrap();
        })
    });
    c.bench("write_queued_kvstore", bench);
}

criterion_group!(benches, write_queued_kvstore);
criterion_main!(benches);
