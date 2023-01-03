use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Enter,
    Queued,
    Playing,
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| GameState::Enter);

    let onclick = {
        let state = state.clone();
        move |_| {
            state.set(GameState::Queued);
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
                        <input type="text" placeholder="username" class="text-white bg-black border border-2 px-4 py-1 text-lg rounded-lg" />
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
