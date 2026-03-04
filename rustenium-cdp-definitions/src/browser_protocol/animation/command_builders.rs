use super::commands::*;
impl GetCurrentTime {
    pub fn builder() -> GetCurrentTimeBuilder {
        GetCurrentTimeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCurrentTimeBuilder {
    id: Option<String>,
}
impl GetCurrentTimeBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<GetCurrentTime, String> {
        Ok(GetCurrentTime {
            method: GetCurrentTimeMethod::GetCurrentTime,
            params: GetCurrentTimeParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            },
        })
    }
}
impl ReleaseAnimations {
    pub fn builder() -> ReleaseAnimationsBuilder {
        ReleaseAnimationsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ReleaseAnimationsBuilder {
    animations: Option<Vec<String>>,
}
impl ReleaseAnimationsBuilder {
    pub fn animation(mut self, animation: impl Into<String>) -> Self {
        let v = self.animations.get_or_insert(Vec::new());
        v.push(animation.into());
        self
    }
    pub fn animations<I, S>(mut self, animations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.animations.get_or_insert(Vec::new());
        for val in animations {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ReleaseAnimations, String> {
        Ok(ReleaseAnimations {
            method: ReleaseAnimationsMethod::ReleaseAnimations,
            params: ReleaseAnimationsParams {
                animations: self.animations.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(animations))
                })?,
            },
        })
    }
}
impl ResolveAnimation {
    pub fn builder() -> ResolveAnimationBuilder {
        ResolveAnimationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResolveAnimationBuilder {
    animation_id: Option<String>,
}
impl ResolveAnimationBuilder {
    pub fn animation_id(mut self, animation_id: impl Into<String>) -> Self {
        self.animation_id = Some(animation_id.into());
        self
    }
    pub fn build(self) -> Result<ResolveAnimation, String> {
        Ok(ResolveAnimation {
            method: ResolveAnimationMethod::ResolveAnimation,
            params: ResolveAnimationParams {
                animation_id: self.animation_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(animation_id))
                })?,
            },
        })
    }
}
impl SeekAnimations {
    pub fn builder() -> SeekAnimationsBuilder {
        SeekAnimationsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SeekAnimationsBuilder {
    animations: Option<Vec<String>>,
    current_time: Option<f64>,
}
impl SeekAnimationsBuilder {
    pub fn animation(mut self, animation: impl Into<String>) -> Self {
        let v = self.animations.get_or_insert(Vec::new());
        v.push(animation.into());
        self
    }
    pub fn animations<I, S>(mut self, animations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.animations.get_or_insert(Vec::new());
        for val in animations {
            v.push(val.into());
        }
        self
    }
    pub fn current_time(mut self, current_time: impl Into<f64>) -> Self {
        self.current_time = Some(current_time.into());
        self
    }
    pub fn build(self) -> Result<SeekAnimations, String> {
        Ok(SeekAnimations {
            method: SeekAnimationsMethod::SeekAnimations,
            params: SeekAnimationsParams {
                animations: self.animations.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(animations))
                })?,
                current_time: self.current_time.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(current_time))
                })?,
            },
        })
    }
}
impl SetPaused {
    pub fn builder() -> SetPausedBuilder {
        SetPausedBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPausedBuilder {
    animations: Option<Vec<String>>,
    paused: Option<bool>,
}
impl SetPausedBuilder {
    pub fn animation(mut self, animation: impl Into<String>) -> Self {
        let v = self.animations.get_or_insert(Vec::new());
        v.push(animation.into());
        self
    }
    pub fn animations<I, S>(mut self, animations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.animations.get_or_insert(Vec::new());
        for val in animations {
            v.push(val.into());
        }
        self
    }
    pub fn paused(mut self, paused: impl Into<bool>) -> Self {
        self.paused = Some(paused.into());
        self
    }
    pub fn build(self) -> Result<SetPaused, String> {
        Ok(SetPaused {
            method: SetPausedMethod::SetPaused,
            params: SetPausedParams {
                animations: self.animations.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(animations))
                })?,
                paused: self
                    .paused
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(paused)))?,
            },
        })
    }
}
impl SetPlaybackRate {
    pub fn builder() -> SetPlaybackRateBuilder {
        SetPlaybackRateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPlaybackRateBuilder {
    playback_rate: Option<f64>,
}
impl SetPlaybackRateBuilder {
    pub fn playback_rate(mut self, playback_rate: impl Into<f64>) -> Self {
        self.playback_rate = Some(playback_rate.into());
        self
    }
    pub fn build(self) -> Result<SetPlaybackRate, String> {
        Ok(SetPlaybackRate {
            method: SetPlaybackRateMethod::SetPlaybackRate,
            params: SetPlaybackRateParams {
                playback_rate: self.playback_rate.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(playback_rate))
                })?,
            },
        })
    }
}
impl SetTiming {
    pub fn builder() -> SetTimingBuilder {
        SetTimingBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetTimingBuilder {
    animation_id: Option<String>,
    duration: Option<f64>,
    delay: Option<f64>,
}
impl SetTimingBuilder {
    pub fn animation_id(mut self, animation_id: impl Into<String>) -> Self {
        self.animation_id = Some(animation_id.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn delay(mut self, delay: impl Into<f64>) -> Self {
        self.delay = Some(delay.into());
        self
    }
    pub fn build(self) -> Result<SetTiming, String> {
        Ok(SetTiming {
            method: SetTimingMethod::SetTiming,
            params: SetTimingParams {
                animation_id: self.animation_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(animation_id))
                })?,
                duration: self.duration.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(duration))
                })?,
                delay: self
                    .delay
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(delay)))?,
            },
        })
    }
}
