use gloo::console;
use gloo::storage::{errors::StorageError, LocalStorage, Storage};
use std::rc::Rc;
use web_sys::{window, MediaQueryList, Window};
use yew::{use_reducer, Hook, Reducible, UseReducerHandle};

pub enum ThemeAction {
    Light,
    Dark,
}

#[derive(PartialEq, Debug)]
pub struct ThemeState {
    pub current: &'static str,
}

pub fn use_theme_context() -> impl Hook<Output = UseReducerHandle<ThemeState>> + 'static {
    use_reducer(ThemeState::default)
}

impl Default for ThemeState {
    fn default() -> Self {
        // Get the theme stored in local storage
        let ls_theme_option: Result<String, StorageError> = LocalStorage::get("theme");

        let ls_theme: &str = match &ls_theme_option {
            Ok(theme) => theme,
            _ => {
                let window: Window = window().expect("No Window Object!");
                let match_media_result = window.match_media("(prefers-color-scheme: dark)"); // : Result<Option<MediaQueryList>, JsValue>
                match match_media_result {
                    Ok(match_media_option) => {
                        let match_media: MediaQueryList =
                            match_media_option.expect("MEDIAQUERYLIST NOT SUPPORTED!");
                        if match_media.matches() {
                            "dark"
                        } else {
                            "light"
                        }
                    }
                    _ => "light",
                }
            }
        };

        match ls_theme {
            "dark" => Self { current: "dark" },
            "light" | _ => Self { current: "light" },
        }
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_theme = match action {
            ThemeAction::Light => "light",
            ThemeAction::Dark => "dark",
        };

        match LocalStorage::set("theme", next_theme) {
	    Ok(()) => console::log!(format!("Theme set to {}", next_theme)),
	    _ => console::error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	};

        Self {
            current: next_theme,
        }
        .into()
    }
}
