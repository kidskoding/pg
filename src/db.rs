extern crate hex;
use postgres::{Client, NoTls};

pub fn connect() -> Result<Client, postgres::Error> {
    let client = Client::connect("host=localhost user=anirudh", NoTls)?;
    Ok(client)
}
