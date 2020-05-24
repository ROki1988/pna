use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use kvs::{KvStore, Result};
use std::process::exit;

fn main() -> Result<()> {
    let set = SubCommand::with_name("set")
        .about("set value with key")
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("value")
                .value_name("VALUE")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        );
    let get = SubCommand::with_name("get")
        .about("get value by key")
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        );
    let rm = SubCommand::with_name("rm")
        .about("remove value by key")
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        );
    let matches = App::new("kvs-client")
        .about("communicate kvs-server")
        // use crate_version! to pull the version number
        .version(crate_version!())
        .author(crate_authors!())
        .subcommands(vec![set, get, rm])
        .get_matches();

    let mut store = KvStore::open("./")?;
    match matches.subcommand() {
        ("set", Some(s)) => store.set(
            s.value_of("key").unwrap().to_string(),
            s.value_of("value").unwrap().to_string(),
        ),
        ("get", Some(g)) => {
            if let Some(v) = store.get(g.value_of("key").unwrap().to_string())? {
                println!("{}", v);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        ("rm", Some(r)) => store
            .remove(r.value_of("key").unwrap().to_string())
            .map_err(|e| {
                if e.is_invalid_argument() {
                    println!("Key not found");
                    exit(1);
                }
                e
            }),
        _ => unreachable!(),
    }
}
