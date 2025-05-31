use crate::components::auth::AuthGuard;
use crate::contexts::theme::ThemeProvider;
use crate::supabase::{auth::AuthContext, SupabaseClient, SupabaseConfig};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let supabase_config = SupabaseConfig::from_env().unwrap_or_else(|_| {
        SupabaseConfig::new(
            "https://your-project.supabase.co".to_string(),
            "your-anon-key".to_string(),
        )
    });

    let supabase_client = SupabaseClient::new(supabase_config);
    let auth_context = use_signal(|| AuthContext::new(supabase_client));
    use_context_provider(|| auth_context);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ThemeProvider { AuthGuard {} }
    }
}
