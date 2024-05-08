use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gray-900 flex w-screen h-screen items-center justify-center">
            <div class="flex flex-col items-center w-full max-w-md px-4 py-8 bg-gray-800 rounded-lg shadow-md">
                <div class="mb-4 text-lg font-medium text-white">{"Login to Chat"}</div>
                <form class="w-full">
                    <input {oninput} type="text" class="w-full p-3 mb-4 text-white bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" placeholder="Username" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len() < 1} class="w-full p-3 bg-blue-600 text-white font-bold rounded-lg hover:bg-blue-700 transition-colors">
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}