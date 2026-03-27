use std::future::Future;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use crate::error::InputError;
use crate::input::bidi::keyboard::{KeyPressOptions, KeyboardTypeOptions};

pub trait Keyboard {
    fn down(
        &self,
        key: &str,
        context: &BrowsingContext,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn up(
        &self,
        key: &str,
        context: &BrowsingContext,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn press(
        &self,
        key: &str,
        context: &BrowsingContext,
        options: Option<KeyPressOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn type_text(
        &self,
        text: &str,
        context: &BrowsingContext,
        options: Option<KeyboardTypeOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;
}
