use anyhow::Result;
use rusty_kv::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    nickname: String,
    level: u32,
}

fn main() -> Result<()> {
    let mut db = Database::load("storage.json")?;

    let new_user = User {
        nickname: "Rustacean".to_string(),
        level: 10,
    };

    db.insert("user_1".to_string(), new_user);
    db.save()?;
    println!("Database saved successfully.");

    if let Some(user) = db.get("user_1") {
        println!("Loaded User: {} (Level {})", user.nickname, user.level)
    }

    Ok(())
}
