use clap::{crate_authors, crate_version, value_t_or_exit, App, Arg, SubCommand};
use kvs::{KvsClient, Result};
use std::net::SocketAddr;
use std::process::exit;

fn main() -> Result<()> {
    let set = SubCommand::with_name("set")
        .about("set value with key")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        )
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
        );
    let get = SubCommand::with_name("get")
        .about("get value by key")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        );
    let rm = SubCommand::with_name("rm")
        .about("remove value by key")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("127.0.0.1:4000")
                .value_name("IP-PORT")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        );
    let matches = App::new("kvs-client")
        .about("communicate kvs-server")
        // use crate_version! to pull the version number
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
        .subcommands(vec![set, get, rm])
        .get_matches();

    match matches.subcommand() {
        ("set", Some(s)) => {
            let addr = value_t_or_exit!(s, "addr", SocketAddr);
            KvsClient::connect(addr)?.set(
                s.value_of("key").unwrap().to_string(),
                s.value_of("value").unwrap().to_string(),
            )?;
            Ok(())
        }
        ("get", Some(g)) => {
            let addr = value_t_or_exit!(g, "addr", SocketAddr);
            if let Some(v) =
                KvsClient::connect(addr)?.get(g.value_of("key").unwrap().to_string())?
            {
                println!("{}", v);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        ("rm", Some(r)) => {
            let addr = value_t_or_exit!(r, "addr", SocketAddr);
            KvsClient::connect(addr)?
                .remove(r.value_of("key").unwrap().to_string())
                .map_err(|e| {
                    if e.is_key_not_found() {
                        eprintln!("Key not found");
                        exit(1);
                    }
                    e
                })
        }
        _ => unreachable!(),
    }
}
