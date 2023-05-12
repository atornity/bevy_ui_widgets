use std::any::Any;

use bevy_render::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;

pub trait ThemePropertyName
where
    Self: ThemeProperty + Sized,
{
    /// The unique name for this property.
    /// Used for internal storage and serialization.
    const PROPERTY_NAME: &'static str;
}

pub trait ThemeProperty
where
    Self: Any + Send + Sync,
{
    fn as_any(&self) -> &dyn Any;
}

macro_rules! define_property {
    ($type: ident, $inner_type: ty, $name_str: expr) => {
        #[derive(Debug, Clone)]
        pub struct $type(pub $inner_type);

        impl ThemePropertyName for $type {
            const PROPERTY_NAME: &'static str = $name_str;
        }

        impl ThemeProperty for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

// Define Style properties
define_property!(DisplayProperty, Display, "display");
define_property!(PositionTypeProperty, PositionType, "position-type");
define_property!(DirectionProperty, Direction, "direction");
define_property!(FlexDirectionProperty, FlexDirection, "flex-direction");
define_property!(FlexWrapProperty, FlexWrap, "flex-wrap");
define_property!(AlignItemsProperty, AlignItems, "align-items");
define_property!(AlignSelfProperty, AlignSelf, "align-self");
define_property!(AlignContentProperty, AlignContent, "align-content");
define_property!(JustifyContentProperty, JustifyContent, "justify-content");
define_property!(PositionProperty, UiRect, "position");
define_property!(MarginProperty, UiRect, "margin");
define_property!(PaddingProperty, UiRect, "padding");
define_property!(BorderProperty, UiRect, "border");
define_property!(FlexGrowProperty, f32, "flex-grow");
define_property!(FlexShrinkProperty, f32, "flex-shrink");
define_property!(FlexBasisProperty, Val, "flex-basis");
define_property!(SizeProperty, Size, "size");
define_property!(MinSizeProperty, Size, "min-size");
define_property!(MaxSizeProperty, Size, "max-size");
define_property!(AspectRatioProperty, Option<f32>, "aspect-ratio");
define_property!(OverflowProperty, Overflow, "overflow");

// Define text properties
define_property!(TextColorProperty, Color, "color");
define_property!(TextFontPathProperty, String, "font-path");
define_property!(TextFontSizeProperty, f32, "font-size");
define_property!(TextHorizontalAlignProperty, TextAlignment, "text-align");

// Define other properties
define_property!(ColorProperty, Color, "background-color");
