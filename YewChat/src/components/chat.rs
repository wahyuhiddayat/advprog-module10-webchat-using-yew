use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen">
                <div class="flex-none w-60 bg-gray-800 text-gray-300">
                    <div class="text-xl p-3 text-white">{"Users"}</div>
                    {
                        for self.users.iter().map(|u| {
                            html!{
                                <div class="flex m-2 bg-gray-700 rounded-lg p-2 hover:bg-gray-600 transition-colors">
                                    <img class="w-12 h-12 rounded-full mr-3" src={u.avatar.clone()} alt="User avatar"/>
                                    <div>
                                        <div class="font-semibold">{u.name.clone()}</div>
                                        <div class="text-sm text-gray-400">{"Hi there!"}</div>
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
                <div class="flex-grow flex flex-col bg-gray-900 text-white">
                    <div class="flex-none h-14 bg-gray-800 flex justify-between items-center px-4 border-b border-gray-700">
                        <h1 class="text-lg font-bold">{"💬 Chat"}</h1>
                    </div>
                    <div class="flex-grow overflow-auto">
                        {
                            for self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-start m-3 bg-gray-800 rounded-lg p-3 hover:bg-gray-700 transition-colors">
                                        <img class="w-8 h-8 rounded-full mr-3" src={user.avatar.clone()} alt="User avatar"/>
                                        <div>
                                            <div class="font-semibold">{m.from.clone()}</div>
                                            <div class="text-sm text-gray-400">{m.message.clone()}</div>
                                        </div>
                                    </div>
                                }
                            })
                        }
                    </div>
                    <div class="flex-none h-14 flex items-center px-4 bg-gray-800">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Type a message..." class="flex-grow py-2 px-4 bg-gray-700 rounded-full outline-none focus:bg-gray-600 text-white placeholder-gray-400"/>
                        <button onclick={submit} class="ml-4 bg-blue-600 hover:bg-blue-700 transition-colors p-2 rounded-full">
                            <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}