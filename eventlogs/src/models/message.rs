use serde::{Deserialize, Serialize};
use tokio_util::{
    bytes::Buf,
    codec::{Decoder, Encoder},
};

use super::{agent::Agent, log::Log};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
        if src.is_empty() {
            return Ok(None);
        }

        if let Ok(data) = String::from_utf8(src.to_vec()) {
            src.advance(data.len());
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(None)
        }
    }
}
