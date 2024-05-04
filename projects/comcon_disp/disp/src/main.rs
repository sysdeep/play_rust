use postgres::{Client, Error, NoTls};
use uuid::Uuid;

fn main() {
    println!("Hello, world!");

    db_test().unwrap();
}

fn db_test() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://comcon:docker@localhost:5454/test_dispatch",
        NoTls,
    )?;

    for row in client.query("SELECT * FROM ms3_drivers", &[])? {
        let id: Uuid = row.get("id");
        let name: String = row.get(1);
        println!("id: {}, name: {}", id.to_string(), name);
    }

    Ok(())
}
