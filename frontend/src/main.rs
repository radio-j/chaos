use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}


#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());
    {
      let welcome = welcome.clone();
      use_effect_with_deps(
          move |name| {
              update_welcome_message(welcome, name.clone());
              || ()
          },
          (*name).clone(),
      );
    }

    let message = (*welcome).clone();

    html! {
        <>
        //<div style="background-color:grey;color:white;margin:20px;padding:20px;border-radius:25px;">
        <div>
            <h2 class={"example"}>{message}</h2>
        </div>
        </>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}
