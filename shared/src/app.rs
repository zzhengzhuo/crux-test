use crate::capabilities::signer::Signer;
use crux_core::render::Render;
use crux_macros::Effect;
use ethers::utils;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default)]
pub struct Model {
    signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewModel {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    // events from the shell
    SignMessage(String),
    SendUuid(Uuid),

    // events local to the core
    #[serde(skip)]
    ShowSignature(Vec<u8>),
}

#[cfg_attr(feature = "typegen", derive(crux_macros::Export))]
#[derive(Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub signer: Signer<Event>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Counter {
    value: isize,
    updated_at: i64,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match msg {
            Event::SignMessage(message) => {
                caps.signer.sign_message(&message, Event::ShowSignature);
            }
            Event::ShowSignature(signature) => {
                model.signature = signature;
                caps.render.render();
            }
            Event::SendUuid(_uuid) => {
                // No-op
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        let Model { signature } = model;

        Self::ViewModel {
            text: format!("Signed Message: 0x{}", utils::hex::encode(signature)),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::{App, Event, Model};
    // use crate::capabilities::sse::SseRequest;
    // use crate::{Counter, Effect};
    // use assert_let_bind::assert_let;
    // use crux_core::{assert_effect, testing::AppTester};
    // use crux_http::{
    //     protocol::{HttpRequest, HttpResponse},
    //     testing::ResponseBuilder,
    // };

    // #[test]
    // fn get_counter() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let mut update = app.update(Event::Get, &mut model);

    //     assert_let!(Effect::Http(request), update.effects_mut().next().unwrap());
    //     let actual = &request.operation;
    //     let expected = &HttpRequest {
    //         method: "GET".to_string(),
    //         url: "https://crux-counter.fly.dev/".to_string(),
    //         headers: vec![],
    //         body: vec![],
    //     };

    //     assert_eq!(actual, expected);

    //     let response = HttpResponse {
    //         status: 200,
    //         body: serde_json::to_vec(&Counter {
    //             value: 1,
    //             updated_at: 1,
    //         })
    //         .unwrap(),
    //     };
    //     let update = app.resolve(request, response).expect("an update");

    //     let actual = update.events;
    //     let expected = vec![Event::new_set(1, 1)];
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn set_counter() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let update = app.update(Event::new_set(1, 1), &mut model);

    //     assert_effect!(update, Effect::Render(_));

    //     let actual = model.count.value;
    //     let expected = 1;
    //     assert_eq!(actual, expected);

    //     let actual = model.confirmed;
    //     let expected = Some(true);
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn increment_counter() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let mut update = app.update(Event::Increment, &mut model);

    //     assert_effect!(update, Effect::Render(_));

    //     let actual = model.count.value;
    //     let expected = 1;
    //     assert_eq!(actual, expected);

    //     let actual = model.confirmed;
    //     let expected = Some(false);
    //     assert_eq!(actual, expected);

    //     assert_let!(Effect::Http(request), update.effects_mut().nth(1).unwrap());
    //     let expected = &HttpRequest {
    //         method: "POST".to_string(),
    //         url: "https://crux-counter.fly.dev/inc".to_string(),
    //         headers: vec![],
    //         body: vec![],
    //     };
    //     let actual = &request.operation;

    //     assert_eq!(actual, expected);

    //     let response = HttpResponse {
    //         status: 200,
    //         body: serde_json::to_vec(&Counter {
    //             value: 1,
    //             updated_at: 1,
    //         })
    //         .unwrap(),
    //     };

    //     let update = app.resolve(request, response).expect("Update to succeed");

    //     let actual = update.events;
    //     let expected = vec![Event::new_set(1, 1)];
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn decrement_counter() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let mut update = app.update(Event::Decrement, &mut model);

    //     assert_effect!(update, Effect::Render(_));

    //     let actual = model.count.value;
    //     let expected = -1;
    //     assert_eq!(actual, expected);

    //     let actual = model.confirmed;
    //     let expected = Some(false);
    //     assert_eq!(actual, expected);

    //     assert_let!(Effect::Http(request), update.effects_mut().nth(1).unwrap());
    //     let actual = request.operation.clone();
    //     let expected = HttpRequest {
    //         method: "POST".to_string(),
    //         url: "https://crux-counter.fly.dev/dec".to_string(),
    //         headers: vec![],
    //         body: vec![],
    //     };
    //     assert_eq!(actual, expected);

    //     let response = HttpResponse {
    //         status: 200,
    //         body: serde_json::to_vec(&Counter {
    //             value: -1,
    //             updated_at: 1,
    //         })
    //         .unwrap(),
    //     };

    //     let update = app.resolve(request, response).expect("a successful update");

    //     let actual = update.events;
    //     let expected = vec![Event::new_set(-1, 1)];
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn get_sse() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let update = app.update(Event::StartWatch, &mut model);

    //     assert_let!(
    //         Effect::ServerSentEvents(request),
    //         update.effects().next().unwrap()
    //     );
    //     let actual = &request.operation;
    //     let expected = &SseRequest {
    //         url: "https://crux-counter.fly.dev/sse".to_string(),
    //     };

    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn set_sse() {
    //     let app = AppTester::<App, _>::default();
    //     let mut model = Model::default();

    //     let count = Counter {
    //         value: 1,
    //         updated_at: 1,
    //     };
    //     let event = Event::WatchUpdate(count);

    //     let update = app.update(event, &mut model);

    //     assert_let!(Effect::Render(_), update.effects().next().unwrap());

    //     let actual = model.count.value;
    //     let expected = 1;
    //     assert_eq!(actual, expected);

    //     let actual = model.confirmed;
    //     let expected = Some(true);
    //     assert_eq!(actual, expected);
    // }

    // impl Event {
    //     fn new_set(value: isize, updated_at: i64) -> Event {
    //         let response = ResponseBuilder::ok()
    //             .body(Counter { value, updated_at })
    //             .build();

    //         Event::Set(Ok(response))
    //     }
    // }
}
