use std::rc::Rc;
use supabase_rs::Provider;

use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaDiscord, FaGoogle};
use dioxus_free_icons::{Icon, IconShape};

use crate::hooks::use_supabase;

// region: Wrapper

struct IconShapeWrapper(Rc<dyn IconShape>);

impl IconShape for IconShapeWrapper {
    fn view_box(&self) -> &str {
        self.0.view_box()
    }

    fn xmlns(&self) -> &str {
        self.0.xmlns()
    }

    fn child_elements(&self) -> Element {
        self.0.child_elements()
    }

    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        self.0.fill_and_stroke(user_color)
    }

    fn stroke_linecap(&self) -> &str {
        self.0.stroke_linecap()
    }

    fn stroke_linejoin(&self) -> &str {
        self.0.stroke_linejoin()
    }
}

impl Clone for IconShapeWrapper {
    fn clone(&self) -> Self {
        IconShapeWrapper(Rc::clone(&self.0))
    }
}

impl PartialEq for IconShapeWrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// endregion

fn get_icon_shape(provider: &Provider) -> IconShapeWrapper {
    match provider {
        Provider::Discord => IconShapeWrapper(Rc::new(FaDiscord)),
        Provider::Google => IconShapeWrapper(Rc::new(FaGoogle)),
        _ => todo!(),
    }
}

fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[component]
pub fn OauthButton(provider: Provider) -> Element {
    let nav = use_navigator();
    let provider_name = capitalize_words(&provider.to_string());
    let shape = get_icon_shape(&provider);
    let on_click = move |_| {
        let oauth_response = use_supabase()
            .login_with_oauth(provider.clone(), None)
            .unwrap();
        nav.push(oauth_response.url.as_str());
    };

    rsx! {
        button {
            class: "cursor-pointer flex items-center justify-center space-x-2 w-full px-4 py-4 rounded-md text-sm font-medium transition-colors \
                duration-200 border border-gray-300 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 \
                disabled:opacity-50 disabled:cursor-not-allowed",
            onclick: on_click,
            Icon { icon: shape }
            span { "Sign in with {provider_name}" }
        }
    }
}
