use serde::{Deserialize, Serialize};
#[doc = "Event for when an animation has been cancelled.\n[animationCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCanceled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCanceledParams {
    #[doc = "Id of the animation that was cancelled."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationCanceledMethod {
    #[serde(rename = "Animation.animationCanceled")]
    AnimationCanceled,
}
#[doc = "Event for when an animation has been cancelled.\n[animationCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCanceled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCanceled {
    pub method: AnimationCanceledMethod,
    pub params: AnimationCanceledParams,
}
impl AnimationCanceled {
    pub const IDENTIFIER: &'static str = "Animation.animationCanceled";
}
#[doc = "Event for each animation that has been created.\n[animationCreated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCreatedParams {
    #[doc = "Id of the animation that was created."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationCreatedMethod {
    #[serde(rename = "Animation.animationCreated")]
    AnimationCreated,
}
#[doc = "Event for each animation that has been created.\n[animationCreated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCreated {
    pub method: AnimationCreatedMethod,
    pub params: AnimationCreatedParams,
}
impl AnimationCreated {
    pub const IDENTIFIER: &'static str = "Animation.animationCreated";
}
#[doc = "Event for animation that has been started.\n[animationStarted](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationStarted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationStartedParams {
    #[doc = "Animation that was started."]
    #[serde(rename = "animation")]
    pub animation: super::types::Animation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationStartedMethod {
    #[serde(rename = "Animation.animationStarted")]
    AnimationStarted,
}
#[doc = "Event for animation that has been started.\n[animationStarted](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationStarted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationStarted {
    pub method: AnimationStartedMethod,
    pub params: AnimationStartedParams,
}
impl AnimationStarted {
    pub const IDENTIFIER: &'static str = "Animation.animationStarted";
}
#[doc = "Event for animation that has been updated.\n[animationUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationUpdatedParams {
    #[doc = "Animation that was updated."]
    #[serde(rename = "animation")]
    pub animation: super::types::Animation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationUpdatedMethod {
    #[serde(rename = "Animation.animationUpdated")]
    AnimationUpdated,
}
#[doc = "Event for animation that has been updated.\n[animationUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationUpdated {
    pub method: AnimationUpdatedMethod,
    pub params: AnimationUpdatedParams,
}
impl AnimationUpdated {
    pub const IDENTIFIER: &'static str = "Animation.animationUpdated";
}
group_enum ! (AnimationEvents { AnimationCanceled (AnimationCanceled) , AnimationCreated (AnimationCreated) , AnimationStarted (AnimationStarted) , AnimationUpdated (AnimationUpdated) });
