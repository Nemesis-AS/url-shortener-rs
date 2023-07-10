mod db;
mod server;

fn main() {
    let res = server::start();
    if let Err(error) = res {
        println!("Could not start Server!");
        println!("{:?}", error);
    }
}
