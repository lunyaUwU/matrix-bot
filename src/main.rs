use matrix_sdk::{
    Client, room::Room, config::SyncSettings,
    ruma::events::room::message::OriginalSyncRoomMessageEvent,
};
async fn on_message_event(event:OriginalSyncRoomMessageEvent, r:Room,client:Client) {
        println!("Received a message {:?}", event.content.body());
        let room = client.get_joined_room(r.room_id()).unwrap();
        match event.content.body() {
            "ping" => ping::ping(event.clone(), room.clone()).await, 
            _ => {}
        } 
        cute(event, room,client).await
        
}
mod commands;
mod session;
use crate::{session::{get_session,onboarding}, commands::{ping, cutie::cute}};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session =  get_session();
    let client=  match session {
        Ok(session) => {
            let client = Client::builder().server_name(session.user_id.server_name()).build().await.unwrap();
            client.restore_login(session).await.expect("something went wrong");
            client
 
        },
        Err(_) => {
            onboarding().await.expect("dont be stupid bitch")
        }  
    };
   //println!("{}",client.access_token().unwrap());
    //println!("access_token ={}\n device_id = {}",
    //         session.access_token, session.device_id);
    client.add_event_handler(on_message_event);
    
    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    let _ = client.sync(SyncSettings::default()).await;
    Ok(())
}


