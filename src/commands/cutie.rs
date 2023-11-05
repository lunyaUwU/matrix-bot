use matrix_sdk::{
    ruma::{events::room::message::{
        OriginalSyncRoomMessageEvent,
        RoomMessageEventContent
    }, TransactionId}, 
    room::Joined, Client
};
pub async fn cute(event:OriginalSyncRoomMessageEvent,room: Joined, client:Client) {
    if event.content.body().contains("not cute") && event.sender != client.user_id().unwrap() {
        let message = RoomMessageEventContent::text_plain("you are cute mew").make_reply_to(&event.clone().into_full_event(room.room_id().into()));
        let txn_id = TransactionId::new();        
        room.send(message, Some(&txn_id)).await;
    }
}
