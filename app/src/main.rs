use std::{sync::Arc, time::Duration};

use futures::SinkExt;
use patternfly_yew::{use_toaster, Toast, ToastViewer, Type};
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
    .await?;

    let ev = ws.close().await?;
    if !ev.was_clean {
        return Err(WsErr::ConnectionFailed { event: ev });
    }

    Ok(())
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| GameState::Enter);
    let input_ref = use_node_ref();
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let toaster = Arc::new(use_toaster().expect("Must be nested under a ToastViewer component"));

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
        let toaster = toaster.clone();
        move |_| {
            state.set(GameState::Queued);

            let value = value.clone();
            let toaster = toaster.clone();

            spawn_local(async move {
                if let Err(err) = websocket(value).await {
                    toaster.toast(Toast {
                        timeout: Some(Duration::from_secs(3)),
                        title: err.to_string(),
                        actions: Vec::new(),
                        body: Default::default(),
                        r#type: Type::Danger,
                    });
                    web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
                }
            });
        }
    };

    let mut animate_logo: Option<&str> = None;

    if *state == GameState::Queued {
        animate_logo = Some("animate-bounce")
    }

    html! {
        <div class="flex flex-col justify-between items-center h-screen">
            <div></div>
            <div class="flex flex-col justify-center items-center">
                <div class={classes!(animate_logo)}>
                    <img src="/icon.svg" height=80 width=80 />
                </div>

                if *state == GameState::Enter {
                    <>
                        <div class="mt-4 mb-6">
                            <span class="text-4xl font-bold">{"Welcome to TickOatTwo!"}</span>
                        </div>
                        <div class="sep flex justify-center gap-2">
                            <span class="w-1 h-1 bg-gray-700 rounded-full"></span>
                            <span class="w-1 h-1 bg-gray-700 rounded-full"></span>
                            <span class="w-1 h-1 bg-gray-700 rounded-full"></span>
                        </div>
                        <div class="mt-6 flex flex-col gap-2">
                            <span class="text-gray-500 font-mono">{"Please enter a nickname"}</span>
                            <div class="flex gap-4">
                                <input ref={input_ref} {onchange} value={input_value} type="text" class="text-white font-mono bg-black border border-2 px-4 py-1 text-lg rounded-lg w-[300px]" />
                                <div {onclick} class="bg-white text-black rounded-lg px-8 py-2 text-lg cursor-pointer select-none">
                                    <span>{"Enter"}</span>
                                </div>
                            </div>
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
            <div class="footer flex border-t-2 border-gray-800 justify-between items-center px-3 py-2 pt-1 w-screen">
                <div class="flex flex-row gap-2 items-center">
                    <div class="flex justify-center items-center">
                        <div class="flex h-3 w-3 relative">
                            <div class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></div>
                            <div class="relative inline-flex rounded-full h-3 w-3 bg-green-500"></div>
                        </div>
                    </div>
                    <span>{"0 players online"}</span>
                </div>
                <div class="flex flex-row gap-2 items-center">
                    <a href="http://github.com/OnyxSkyscape/tickoattwo-rust" target="_blank">{"source"}</a>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn Frame() -> Html {
    html! {
        <>
            <ToastViewer>
                <App />
            </ToastViewer>
        </>
    }
}

fn main() {
    yew::Renderer::<Frame>::new().render();
}
