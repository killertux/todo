use crate::{Card, List};
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};

pub fn load_and_create_list_signal<CardType>(
    cx: Scope,
    list_key: &'static str,
) -> (ReadSignal<List<CardType>>, WriteSignal<List<CardType>>)
where
    CardType: Card + Serialize + DeserializeOwned,
{
    let storage = window()
        .local_storage()
        .expect("Failed to get storage")
        .unwrap();
    let (list, set_list) = create_signal(
        cx,
        storage
            .get(list_key)
            .expect("Error getting item from storage")
            .map(|text| {
                serde_json::from_str::<List<CardType>>(&text).expect("Error desserializing item")
            })
            .unwrap_or_default(),
    );
    create_effect(cx, move |_| {
        let storage = window()
            .local_storage()
            .expect("Failed to get storage")
            .unwrap();
        storage.set(
            list_key,
            &serde_json::to_string(&list.get()).expect("Error serializing list"),
        )
    });
    (list, set_list)
}
