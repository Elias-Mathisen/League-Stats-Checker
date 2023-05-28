use std::io;
use reqwest::header;
use dotenv::dotenv;
use std::collections::HashSet;
use colored::*;

struct Summoner {
    name: String,
    id: String,
    level: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = dotenv::var("API_KEY").unwrap();
    let mut summoner = String::new();
    println!("Please input {}:", "summoner".blue());
    io::stdin().read_line(&mut summoner).expect("Failed to read summoner");
    println!();

    let mut server = String::new();
    println!("Please choose server:
    
    Brazil (BR1)
    Europe Nordic & East (EUN1)
    Europe West (EUW1)
    Latin America North (LA1)
    Latin America South (LA2)
    North America (NA1)
    Oceania (OCE/OC1)
    Russia (RU1)
    Turkey (TR1)
    Japan (JP1)
    Republic of Korea (KR)
    The Philippines (PH2)
    Singapore, Malaysia, & Indonesia (SG2)
    Taiwan, Hong Kong, and Macao (TW2)
    Thailand (TH2)
    Vietnam (VN2

Example: euw1"
);

    io::stdin().read_line(&mut server).expect("Failed to read server");
    println!();

    let valid_servers: HashSet<String> = [
        "br1", "eun1", "euw1", "la1", "la2", "na1", "oc1", "ru1", "tr1", "jp1", "kr", "ph2",
        "sg2", "tw2", "th2", "vn2",
    ]
    .iter()
    .map(|s| s.to_lowercase())
    .collect();

    let server_lowercase = server.trim().to_lowercase();
    if !valid_servers.contains(&server_lowercase) {
        println!("Server '{}' Not found", server.trim().red());
        return Ok(());
    }

    println!("Loading stats of summoner: {}", summoner.blue());

    let url = format!(
        "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}",
        server_lowercase,
        summoner.trim(),
        api_key
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .send()
        .await?;

    let body = response.bytes().await?;

    let summoner_info: serde_json::Value = serde_json::from_slice(&body)?;

    if let Some(name) = summoner_info.get("name") {
        let user = Summoner {
            name: name.as_str().unwrap().to_string(),
            id: summoner_info["id"].as_str().unwrap().to_string(),
            level: summoner_info["summonerLevel"].as_i64().unwrap() as i32,
        };

        println!("{}'s Information:", user.name.blue());
        println!("Name: {}", user.name);
        println!("ID: {}", user.id);
        println!("Level: {}", user.level);
    } else {
        println!("User '{}' Not found", summoner.trim());
    }

    Ok(())
}
