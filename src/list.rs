use crate::card::Card;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct List<CardType>
where
    CardType: Card,
{
    cards: Vec<CardType>,
}

impl<CardType> List<CardType>
where
    CardType: Card,
{
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn cards(&self) -> Vec<CardType> {
        self.cards.clone()
    }

    pub fn cards_reverse(&self) -> Vec<CardType> {
        let mut result = self.cards();
        result.reverse();
        result
    }

    pub fn add_card(&mut self, data: impl Into<CardType>) {
        self.cards.push(data.into());
    }

    pub fn move_card_up(&mut self, uuid: &Uuid) {
        if let Some(position) = self.cards.iter().position(|card| card.uuid() == uuid) {
            let card = self.cards.remove(position);
            let new_position = if position == 0 { 0 } else { position - 1 };
            self.cards.insert(new_position, card);
        }
    }

    pub fn move_card_down(&mut self, uuid: &Uuid) {
        if let Some(position) = self.cards.iter().position(|card| card.uuid() == uuid) {
            let card = self.cards.remove(position);
            let new_position = position + 1;
            self.cards.insert(
                if new_position > self.cards.len() {
                    self.cards.len()
                } else {
                    new_position
                },
                card,
            );
        }
    }

    pub fn remove_card(&mut self, uuid: &Uuid) -> Option<CardType> {
        if let Some(position) = self.cards.iter().position(|card| card.uuid() == uuid) {
            return Some(self.cards.remove(position));
        }
        None
    }
}

impl<T: Card> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}
