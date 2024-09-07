use crate::components::theme::{Dark, Light};
use crate::contexts::theme::ThemeAction;
use crate::AppContext;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");
    let cycle_theme = {
        let app_context = app_context.clone();
        let current_theme: &str = app_context.theme.current;
        let current_theme_index: usize = match app_context
            .theme_cycle
            .iter()
            .position(|x: &&str| x == &current_theme)
        {
            Some(i) => i,
            None => 0,
        };
        let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
            Some(nt) => nt,
            None => "light",
        };
        Callback::from(move |_| match next_theme {
            "dark" => app_context.theme.dispatch(ThemeAction::Dark),
            "light" | _ => app_context.theme.dispatch(ThemeAction::Light),
        })
    };

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "dark" => {
                html! {<Dark class={Some("cursor-pointer h-[1.5rem] w-[1.5rem] fill-slate-300")} />}
            }
            "light" | _ => {
                html! {<Light class={Some("cursor-pointer h-[1.5rem] w-[1.5rem] fill-orange-400")} />}
            }
        }
    }
    html! {
        <div>
        <nav class="fixed top-0 z-40 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
      <div class="px-3 py-4 lg:px-5 lg:pl-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center justify-start ">
            <a href="https://flowbite.com" class="flex ms-2 md:me-24">
              <span class="self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white">{"CRM"}</span>
            </a>
          </div>
          <div class="flex items-center">

              <div class="flex items-center ms-3">
                    <a onclick={ cycle_theme }>
                        { handle_theme_icon(app_context.clone()) }
                    </a>
                <div>
                </div>
            </div>
           </div>
        </div>
      </div>
    </nav>
    </div>
    }
}
