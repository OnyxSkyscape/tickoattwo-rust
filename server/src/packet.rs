use base64;
use tungstenite::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Nickname(String),
}

// Simple text based packet coder
pub struct Packet {
    pub event: Event,
}

impl Packet {
    pub fn decode_message(message: &Message) -> Result<Self, &str> {
        let raw_message = match message.to_text() {
            Ok(msg) => msg,
            _ => return Err("invalid message"),
        };
        Self::decode_raw(raw_message)
    }

    pub fn decode_raw(raw: &str) -> Result<Self, &str> {
        let (raw_event, payload) = Self::decode_data(raw)?;

        // Define event decoding here
        let event = match raw_event.as_str() {
            "nickname" => Event::Nickname(payload),
            _ => return Err("unknown event type"),
        };

        Ok(Self { event })
    }

    fn decode_data(raw: &str) -> Result<(String, String), &str> {
        let parts = raw.split(':').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("invalid packet parts");
        }

        let event = String::from(parts[0]);
        if let Ok(bytes) = base64::decode(parts[1]) {
            if let Ok(payload) = String::from_utf8(bytes) {
                return Ok((event, payload));
            }
        }

        Err("failed to decode payload")
    }

    pub fn encode_message(&self) -> Message {
        Message::Text(self.encode_raw())
    }

    pub fn encode_raw(&self) -> String {
        // Define event encoding here
        match &self.event {
            Event::Nickname(nickname) => Self::encode_data("nickname", &nickname),
        }
    }

    fn encode_data(event: &str, payload: &str) -> String {
        format!("{}:{}", event, base64::encode(payload))
    }
}
