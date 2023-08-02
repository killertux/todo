use super::get_element_by_id;
use crate::card::{Card, DoneCard, ToDoCard};
use crate::list::List;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::*;
use web_sys::HtmlParagraphElement;

#[component]
pub fn ToDoCardList(
    cx: Scope,
    list: ReadSignal<List<ToDoCard>>,
    set_list: WriteSignal<List<ToDoCard>>,
    done_card_set_list: WriteSignal<List<DoneCard>>,
) -> impl IntoView {
    let (show_insert_card, set_show_insert_card) = create_signal(cx, false);
    setup_keyboard_events(show_insert_card, set_show_insert_card, set_list);
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
                                <p class="whitespace-pre-line">{card.text}</p>
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
            <p type="text" contenteditable="true" id="new_card" class={move || format!("w-full bg-zinc-50 rounded-md p-2 whitespace-pre-line {}", if show_insert_card.get() {""} else {"hidden"})}></p>
        </div>
    }
}

fn setup_keyboard_events(
    show_insert_card: ReadSignal<bool>,
    set_show_insert_card: WriteSignal<bool>,
    set_list: WriteSignal<List<ToDoCard>>,
) {
    window_event_listener(ev::keypress, move |ev| {
        // ev is typed as KeyboardEvent automatically,
        // so .code() can be called
        let code = ev.code();
        let input = get_element_by_id::<HtmlParagraphElement>("new_card");
        match code.as_str() {
            "KeyI" if !show_insert_card.get() => {
                set_show_insert_card.set(true);
                input.set_text_content(None);
                input.focus().expect("Error focusing new card input");
            }
            "Enter" if ev.shift_key() && show_insert_card.get() => {
                set_show_insert_card.set(false);
                let value = input.inner_html();
                set_list.update(|list| list.add_card(value.replace("<br>", "\n").trim()));
            }
            _ => {}
        }
    });
}
