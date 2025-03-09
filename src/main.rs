pub mod users;

extern crate hex;
use pg::db;
use users::{user::User, user_mgmt::{add_user, get_user}};

fn main() -> Result<(), postgres::Error> {
    let mut db = db::connect()?;

    let demo_user = User::new(
        "Ferris".to_string(),
        "ferristhecrab@gmail.com".to_string(),
        "rustacean".to_string(),
    );

    add_user(&mut db, &demo_user)?;

    let user = get_user(&mut db, "Ferris")?.unwrap();
    println!("User {} found: {}", user.username, user);

    Ok(())
}
