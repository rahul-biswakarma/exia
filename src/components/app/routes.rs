use crate::components::{home::Home, synapse::Synapse};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/synapse")]
    Synapse {},
}
