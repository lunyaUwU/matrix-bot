use matrix_sdk::{Session, Client,
    ruma::{device_id,user_id}
};
use scanrs::scanln;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;
#[derive(Debug, Deserialize)]
struct Config {
    access_token: String,
    refresh_token: Option<String>,
    device_id: String,
    user_id: String,
}
pub fn get_session() -> anyhow::Result<Session, anyhow::Error> {
    let mut f = File::open("Session.toml")?;
    let mut data: String = "".to_string(); 
    f.read_to_string(&mut data)?;
    let decoded: Config = toml::from_str(&data)?;
    Ok(Session {
        access_token: decoded.access_token,
        refresh_token: decoded.refresh_token,
        device_id: device_id!(decoded.device_id.as_str()).to_owned(),
        user_id: <&user_id::UserId>::try_from(decoded.user_id.as_str())?.to_owned(),
    })
    
}
pub async fn onboarding() -> anyhow::Result<Client> {
    println!("it appears like you dont have a valid session");
    println!("please enter your userid");
    let user_id_str = scanln();
    let user = <&user_id::UserId>::try_from(user_id_str.as_str())?.to_owned();
    let client = Client::builder().server_name(user.server_name()).build().await.unwrap();
    println!("please enter your password");
    let password = scanln();
    client.login_username(&user,&password).send().await?;
    save_session(client.session().unwrap())?; 
    //client.login_username(user,"Economy-Reformed-Anew4").send().await?;
    //client.login_token(token).send().await?;
    //thread::sleep(time::Duration::from_secs(5));
    //let session = client.session().unwrap();
    Ok(client) 
}

fn save_session(s: Session) -> anyhow::Result<()> {
    let mut file = File::create("Session.toml")?;
    let content:String = match s.refresh_token {
        Some(t) => {
            format!("access_token = \"{}\"\nrefresh_token = \"{}\"\nuser_id = \"{}\"\ndevice_id = \"{}\""
                    , s.access_token, t, s.user_id, s.device_id)
        }
        None => {
            format!("access_token = \"{}\"\nuser_id = \"{}\"\ndevice_id = \"{}\""
                    , s.access_token, s.user_id, s.device_id)
        }};
    file.write_all(&content.as_bytes())?;
    
    Ok(())
}
