#![allow(dead_code)]
use kvs::Store;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "KVS")]
enum KVS {
    Get { key: String },
    Set { key: String, value: String },
    RM { key: String },
}

fn main() {
    let mut store: Store = Default::default();
    match KVS::from_args() {
        KVS::Get { key } => {
            println!("{}", store.get(&key).expect("no matching value found"));
        }
        KVS::Set { key, value } => {
            store.set(&key, &value);
        }
        KVS::RM { key } => {
            store.remove(&key);
        }
    }
}

fn test() {
    let mut accounts: Store = Default::default();
    let key = "test";
    accounts.set(key, "test");
    println!(
        "{} : {}",
        key,
        accounts.get(key).expect("Key does not exist in database")
    );
    accounts.remove(key);
}
