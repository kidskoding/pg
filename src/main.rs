extern crate hex;
use postgres::{Client, NoTls};
use sha2::{Digest, Sha256};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("host=localhost user=anirudh", NoTls)?;

    let username = "Ferris";
    let email = "ferristhecrab@gmail.com";
    let password = "rustacean";

    let mut hasher: Sha256 = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    client.execute(
        "INSERT INTO users (username, email, password, password_hash) VALUES ($1, $2, $3, $4)",
        &[&username, &email, &password, &password_hash],
    )?;

    for row in client.query(
        "SELECT id, username, email, password, password_hash FROM users",
        &[],
    )? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let email: &str = row.get(2);
        let password: &str = row.get(3);
        let password_hash: &str = row.get(4);

        println!(
            "found person: {} {} {} {} {:?}",
            id, username, email, password, password_hash
        );
    }

    Ok(())
}
