use serde::{Deserialize, Serialize};
use tokio_util::codec::{Decoder, Encoder, Framed};

use super::{agent::Agent, log::Log};

#[derive(Clone, Serialize, Deserialize)]
pub enum Message {
    Agent(Agent),
    Auth(String, String),
    Log(Log),
}

pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = std::io::Error;
    fn encode(
        &mut self,
        item: Message,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let data = serde_json::to_string(&item)?;
        dst.extend_from_slice(data.as_bytes());
        dst.extend_from_slice(b"\n");
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Error = std::io::Error;
    type Item = Message;
    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if let Ok(data) = String::from_utf8(src.to_vec()) {
            Ok(Some(serde_json::from_str(&data)?))
        } else {
            Ok(None)
        }
    }
}
