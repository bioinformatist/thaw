use leptos::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = vec![
        SelectOption {
            label: "main".into(),
            value: "https://thawui.vercel.app".into(),
        },
        SelectOption {
            label: "0.2.5".into(),
            value: "https://thaw-8og1kv8zs-thaw.vercel.app".into(),
        },
    ];

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "csr", feature = "hydrate"))] {
            let location = window().location();
            let origin = location.origin().ok();
            let version = RwSignal::new(origin);
            let _ = version.watch(move |origin| {
                if let Some(origin) = origin {
                    let pathname = location.pathname().unwrap_or_default();
                    let href = format!("{}{}", origin, pathname);
                    let _ = location.set_href(&href);
                }
            });
        } else {
            let version = RwSignal::new(None::<String>);
        }
    }

    view! {
        <Select value=version options/>
    }
}
