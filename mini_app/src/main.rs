use contexts::theme::{use_theme_context, ThemeState};
use route::{switch_main, MainRoute};
use yew::prelude::*;
use yew_router::prelude::*;
mod app;
mod components;
mod contexts;
mod route;

#[derive(Clone, PartialEq)]
pub struct AppContext {
    theme: UseReducerHandle<ThemeState>,
    theme_cycle: Vec<&'static str>,
}

#[function_component]
fn App() -> Html {
    let theme: UseReducerHandle<ThemeState> = use_theme_context();
    let theme_cycle: Vec<&str> = vec!["light", "dark"];

    html! {
        <ContextProvider<AppContext> context={AppContext {
            theme: theme.clone(),
            theme_cycle: theme_cycle,
        }}>
        <main class ={theme.current}>
        <div class="dark:bg-gray-900 min-h-screen">
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
    </BrowserRouter>
    </div>
    </main>

     </ContextProvider<AppContext>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
