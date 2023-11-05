use matrix_sdk::{
    ruma::{events::room::message::{
        OriginalSyncRoomMessageEvent,
        RoomMessageEventContent
    }, TransactionId}, 
    room::Joined
};
pub async fn ping(event:OriginalSyncRoomMessageEvent,room: Joined) {
    let message = RoomMessageEventContent::text_plain("pong").make_reply_to(&event.clone().into_full_event(room.room_id().into()));
    let txn_id = TransactionId::new();
    room.send(message, Some(&txn_id)).await.unwrap();
}
