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
