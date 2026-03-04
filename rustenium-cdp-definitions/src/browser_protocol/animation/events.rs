use serde::{Deserialize, Serialize};
#[doc = "Event for when an animation has been cancelled.\n[animationCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCanceled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCanceled {
    #[doc = "Id of the animation that was cancelled."]
    #[serde(rename = "id")]
    pub id: String,
}
impl AnimationCanceled {
    pub const IDENTIFIER: &'static str = "Animation.animationCanceled";
}
#[doc = "Event for each animation that has been created.\n[animationCreated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationCreated {
    #[doc = "Id of the animation that was created."]
    #[serde(rename = "id")]
    pub id: String,
}
impl AnimationCreated {
    pub const IDENTIFIER: &'static str = "Animation.animationCreated";
}
#[doc = "Event for animation that has been started.\n[animationStarted](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationStarted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationStarted {
    #[doc = "Animation that was started."]
    #[serde(rename = "animation")]
    pub animation: super::types::Animation,
}
impl AnimationStarted {
    pub const IDENTIFIER: &'static str = "Animation.animationStarted";
}
#[doc = "Event for animation that has been updated.\n[animationUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#event-animationUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationUpdated {
    #[doc = "Animation that was updated."]
    #[serde(rename = "animation")]
    pub animation: super::types::Animation,
}
impl AnimationUpdated {
    pub const IDENTIFIER: &'static str = "Animation.animationUpdated";
}
group_enum ! (Event { AnimationCanceled (AnimationCanceled) , AnimationCreated (AnimationCreated) , AnimationStarted (AnimationStarted) , AnimationUpdated (AnimationUpdated) });
