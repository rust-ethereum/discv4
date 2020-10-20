use crate::message::*;
use enum_primitive_derive::Primitive;
use tokio::sync::oneshot::Sender as OneshotSender;

#[derive(Primitive)]
pub enum MessageId {
    Ping = 1,
    Pong = 2,
    FindNode = 3,
    Neighbours = 4,
}

pub enum EgressMessage {
    Ping(PingMessage),
    Pong(PongMessage),
    FindNode((FindNodeMessage, Option<OneshotSender<NeighboursMessage>>)),
    Neighbours(NeighboursMessage),
}
