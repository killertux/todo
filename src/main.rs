use chrono::{DateTime, Utc};
use leptos::*;
use uuid::Uuid;

fn main() {
    mount_to_body(|cx| {
        move || {
            let (todo_list, set_todo_list) = create_signal(cx, List::new());
            set_todo_list.update(|list| {
                list.add_card("Something very important to be done".to_string());
                list.add_card("Another important thing to be done".to_string());
                list.add_card(
                    "Another very big important task that we need to se how it works".to_string(),
                );
            });
            let (done_list, set_done_list) = create_signal(cx, List::new());

            view! { cx,
                <main class="flex flex-col justify-center items-center m-4 gap-6">
                    <ToDoCardList list={todo_list} set_list={set_todo_list} done_card_set_list={set_done_list} />
                    <DoneCardList list={done_list} />
                </main>
            }
        }
    })
}

#[component]
fn ToDoCardList(
    cx: Scope,
    list: ReadSignal<List<ToDoCard>>,
    set_list: WriteSignal<List<ToDoCard>>,
    done_card_set_list: WriteSignal<List<DoneCard>>,
) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col items-center rounded-lg shadow-zinc-400 shadow-lg hover:shadow-xl hover:shadow-zinc-400 p-2 bg-zinc-200 transition ease-in-out w-1/3 gap-2">
            <h1 class="text-2xl font-bold">ToDo</h1>
            <hr class="h-1 w-5/6 bg-zinc-600 rounded-full my-2"></hr>
            <Show when={move || !list.get().is_empty()} fallback={move |_| view!{cx, <p class="text-zinc-400">"Nothing to see here!"</p>}}>
                <For each={move || list.get().cards()} key={move |card| *card.uuid()} view=move |_, card| {
                    let card_uuid = *card.uuid();
                    view!{cx,
                        <div class="flex flex-row bg-zinc-100 rounded-md w-full hover:bg-zinc-50 transition ease-in-out justify-between h-fit">
                            <div class="">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-1/2 hover:bg-zinc-300 rounded-tl-md transition ease-in-out cursor-pointer" on:click=move |_| set_list.update(|list| list.move_card_up(&card_uuid))>
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5" />
                                </svg>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-1/2 hover:bg-zinc-300 rounded-bl-md transition ease-in-out cursor-pointer" on:click=move |_| set_list.update(|list| list.move_card_down(&card_uuid))>
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                                </svg>
                            </div>
                            <div class="flex flex-col p-2 justify-center items-center">
                                <p class="">{card.text}</p>
                                <p class="mt-1 text-zinc-300 text-sm">{card.datetime.clone().to_rfc3339()}</p>
                            </div>
                            <div class="bg-green-400 hover:bg-green-200 rounded-r-md p-2 transition ease-in-out cursor-pointer" on:click=move |_| set_list.update(|list| {let card = list.remove_card(&card_uuid); done_card_set_list.update(|done_list| done_list.add_card(card.unwrap()))})>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-full">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                                </svg>
                            </div>
                        </div>
                }} />
            </Show>
        </div>
    }
}

#[component]
fn DoneCardList(cx: Scope, list: ReadSignal<List<DoneCard>>) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col items-center rounded-lg shadow-zinc-400 shadow-lg hover:shadow-xl hover:shadow-zinc-400 p-2 bg-zinc-200 transition ease-in-out w-1/3 gap-2">
            <h1 class="text-2xl font-bold">Done</h1>
            <hr class="h-1 w-5/6 bg-zinc-600 rounded-full my-2"></hr>
            <Show when={move || !list.get().is_empty()} fallback={move |_| view!{cx, <p class="text-zinc-400">"Nothing to see here!"</p>}}>
                <For each={move || list.get().cards()} key={move |card| *card.uuid()} view={move |_, card| view!{cx,
                    <div class="flex flex-col bg-zinc-100 rounded-md w-full hover:bg-zinc-50 transition ease-in-out justify-between h-fit items-center">
                        <p class="p-2">{card.card.text}</p>
                        <p class="mt-1 text-zinc-300 text-sm">Created at: {card.card.datetime.clone().to_rfc3339()}</p>
                        <p class="mt-1 text-zinc-300 text-sm">Done at: {card.done_datetime.clone().to_rfc3339()}</p>
                    </div>
                }} />
            </Show>
        </div>
    }
}

#[derive(Clone)]
struct List<CardType>
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

trait Card: Clone {
    fn uuid(&self) -> &Uuid;
}

#[derive(Clone)]
struct ToDoCard {
    pub uuid: Uuid,
    pub text: String,
    pub datetime: DateTime<Utc>,
}

#[derive(Clone)]
struct DoneCard {
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