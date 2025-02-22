//! Contins everything related to audio effects that can be applied to an audio bus.

use crate::{
    effects::filter::{
        AllPassFilterEffect, BandPassFilterEffect, HighPassFilterEffect, HighShelfFilterEffect,
        LowPassFilterEffect, LowShelfFilterEffect,
    },
    effects::reverb::Reverb,
};
use fyrox_core::{reflect::prelude::*, visitor::prelude::*};
use std::ops::{Deref, DerefMut};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

pub mod filter;
pub mod reverb;

/// Attenuation effect.
#[derive(Debug, Clone, PartialEq, Visit, Reflect)]
pub struct Attenuate {
    gain: f32,
}

impl Default for Attenuate {
    fn default() -> Self {
        Self { gain: 1.0 }
    }
}

impl Attenuate {
    /// Creates new attenuation effect.
    pub fn new(gain: f32) -> Self {
        Self {
            gain: gain.max(0.0),
        }
    }
}

impl EffectRenderTrait for Attenuate {
    fn render(&mut self, input: &[(f32, f32)], output: &mut [(f32, f32)]) {
        for ((input_left, input_right), (output_left, output_right)) in
            input.iter().zip(output.iter_mut())
        {
            *output_left = *input_left * self.gain;
            *output_right = *input_right * self.gain;
        }
    }
}

#[doc(hidden)]
#[derive(PartialEq, Debug, Clone, Default, Reflect)]
pub struct EffectWrapper(#[reflect(display_name = "Effect Type")] pub Effect);

impl Deref for EffectWrapper {
    type Target = Effect;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EffectWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Visit for EffectWrapper {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.0.visit(name, visitor)
    }
}

/// Effects is a digital signal processing (DSP) unit that transforms input signal in a specific way.
/// For example, [`LowPassFilterEffect`] could be used to muffle audio sources; to create "underwater"
/// effect.
#[derive(Debug, Clone, PartialEq, Visit, Reflect, AsRefStr, EnumString, EnumVariantNames)]
pub enum Effect {
    /// See [`Attenuate`] docs for more info.
    Attenuate(Attenuate),
    /// See [`Reverb`] docs for more info.
    Reverb(Reverb),
    /// See [`LowPassFilterEffect`] docs for more info.
    LowPassFilter(LowPassFilterEffect),
    /// See [`HighPassFilterEffect`] docs for more info.
    HighPassFilter(HighPassFilterEffect),
    /// See [`BandPassFilterEffect`] docs for more info.
    BandPassFilter(BandPassFilterEffect),
    /// See [`AllPassFilterEffect`] docs for more info.
    AllPassFilter(AllPassFilterEffect),
    /// See [`LowShelfFilterEffect`] docs for more info.
    LowShelfFilter(LowShelfFilterEffect),
    /// See [`HighShelfFilterEffect`] docs for more info.
    HighShelfFilter(HighShelfFilterEffect),
}

impl Default for Effect {
    fn default() -> Self {
        Effect::Attenuate(Default::default())
    }
}

pub(crate) trait EffectRenderTrait {
    fn render(&mut self, input: &[(f32, f32)], output: &mut [(f32, f32)]);
}

macro_rules! static_dispatch {
    ($self:ident, $func:ident, $($args:expr),*) => {
        match $self {
            Effect::Attenuate(v) => v.$func($($args),*),
            Effect::Reverb(v) => v.$func($($args),*),
            Effect::LowPassFilter(v) => v.$func($($args),*),
            Effect::HighPassFilter(v) => v.$func($($args),*),
            Effect::BandPassFilter(v) => v.$func($($args),*),
            Effect::AllPassFilter(v) => v.$func($($args),*),
            Effect::LowShelfFilter(v) => v.$func($($args),*),
            Effect::HighShelfFilter(v) => v.$func($($args),*),
        }
    };
}

impl EffectRenderTrait for Effect {
    fn render(&mut self, input: &[(f32, f32)], output: &mut [(f32, f32)]) {
        static_dispatch!(self, render, input, output)
    }
}
