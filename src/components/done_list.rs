use crate::card::{Card, DoneCard};
use crate::List;
use leptos::*;

#[component]
pub fn DoneCardList(cx: Scope, list: ReadSignal<List<DoneCard>>) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col items-center rounded-lg shadow-zinc-400 shadow-lg hover:shadow-xl hover:shadow-zinc-400 p-2 bg-zinc-200 transition ease-in-out w-1/3 gap-2">
            <h1 class="text-2xl font-bold">Done</h1>
            <hr class="h-1 w-5/6 bg-zinc-600 rounded-full my-2"></hr>
            <Show when={move || !list.get().is_empty()} fallback={move |_| view!{cx, <p class="text-zinc-400">"Nothing to see here!"</p>}}>
                <For each={move || list.get().cards_reverse()} key={move |card| *card.uuid()} view={move |_, card| view!{cx,
                    <div class="flex flex-col bg-zinc-100 rounded-md w-full hover:bg-zinc-50 transition ease-in-out justify-between h-fit items-center">
                        <p class="p-2 whitespace-pre-line">{card.card.text}</p>
                        <p class="mt-1 text-zinc-300 text-sm">Created at: {card.card.datetime.clone().to_rfc3339()}</p>
                        <p class="mt-1 text-zinc-300 text-sm">Done at: {card.done_datetime.clone().to_rfc3339()}</p>
                    </div>
                }} />
            </Show>
        </div>
    }
}
