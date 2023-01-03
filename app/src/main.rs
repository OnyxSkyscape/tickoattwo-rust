use futures::SinkExt;
use tickoattwo::packet::{Event, Packet};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use ws_stream_wasm::{WsErr, WsMessage, WsMeta};
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Enter,
    Queued,
    Playing,
}

async fn websocket(username: String) -> Result<(), WsErr> {
    let (ws, mut io) = WsMeta::connect("ws://127.0.0.1:8080/api/ws", None).await?;

    io.send(WsMessage::Text(
        Packet::new(Event::Nickname(username)).encode_raw(),
    ))
    .await;

    Ok(())
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| GameState::Enter);
    let input_ref = use_node_ref();
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let onchange = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let onclick = {
        let state = state.clone();
        let value = input_value.clone();
        move |_| {
            state.set(GameState::Queued);

            let value = value.clone();

            spawn_local(async move {
                if let Err(err) = websocket(value).await {
                    web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
                }
            });
        }
    };

    html! {
        <div class="flex flex-col justify-center items-center p-12 h-screen">
            <div>
                <img src="/icon.svg" height=80 width=80 />
            </div>
            <div class="mt-4">
                <span class="text-4xl font-bold">{"TickOatTwo"}</span>
            </div>
            if *state == GameState::Enter {
                <>
                    <div class="mt-6">
                        <input ref={input_ref} {onchange} value={input_value} type="text" placeholder="username" class="text-white bg-black border border-2 px-4 py-1 text-lg rounded-lg" />
                    </div>
                    <div {onclick} class="mt-5 bg-white text-black rounded-lg px-8 py-2 text-lg cursor-pointer select-none">
                        <span>{"Enter"}</span>
                    </div>
                </>
            }
            if *state == GameState::Queued {
                <div class="mt-5 text-lg text-gray-400 flex gap-4 justify-center items-center">
                    <svg class="spinner">
                        <circle cx="20" cy="20" r="18"></circle>
                    </svg>
                    <span>{"Waiting for opponent..."}</span>
                </div>
            }
            if *state == GameState::Playing {
                <div class="flex flex-col mt-12 justify-center items-center">
                    <div class="flex">
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                    </div>
                    <div class="flex">
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                    </div>
                    <div class="flex">
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                        <div class="h-[100px] w-[100px] border border-4"></div>
                    </div>
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
