use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Card: Clone {
    fn uuid(&self) -> &Uuid;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToDoCard {
    pub uuid: Uuid,
    pub text: String,
    pub datetime: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DoneCard {
    pub card: ToDoCard,
    pub done_datetime: DateTime<Utc>,
}

impl From<String> for ToDoCard {
    fn from(text: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            text: text.to_string(),
            datetime: Utc::now(),
        }
    }
}

impl<'a> From<&'a str> for ToDoCard {
    fn from(text: &'a str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            text: text.to_string(),
            datetime: Utc::now(),
        }
    }
}

impl From<ToDoCard> for DoneCard {
    fn from(card: ToDoCard) -> Self {
        Self {
            card,
            done_datetime: Utc::now(),
        }
    }
}

impl Card for ToDoCard {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Card for DoneCard {
    fn uuid(&self) -> &Uuid {
        &self.card.uuid
    }
}
