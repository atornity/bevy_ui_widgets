use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{color::Color, view::Visibility};
use bevy_text::*;
use bevy_transform::{components::*, hierarchy::DespawnRecursiveExt};
use bevy_ui::{entity::*, *};
use bevy_window::*;

pub mod builder;
pub mod slider_tooltip;

use crate::{
    tooltip::{builder::TooltipBuilder, TooltipPosition, TooltipTextUiNode, TooltipUiNodes},
    utils::get_uinode_clipped_rect,
};

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(slider_test)
            .add_system(slider_thumb_update)
            .add_system(slider_thumb_grab)
            .add_system(slider_thumb_move)
            .add_system_to_stage(CoreStage::PreUpdate, slider_tooltip)
            .add_system(slider_tooltip_update)
            .add_system(slider_tooltip_text_update)
            .add_system(slider_tooltip_visibility);
    }
}

// TODO: we may want to separate the "settings" from the "value"
// it could help especially with simplifying Changed<> queries.
#[derive(Component, Clone, Debug)]
pub struct Slider {
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: 0,
            min: 0,
            max: 100,
            step: 1,
        }
    }
}

/// When present, describes the visual appearance for the slider tooltip.
/// Remove this component to disable the slider's tooltip.
///
/// TODO: Improve this. It's not ideal to have to copy all those fields here.
#[derive(Component, Default, Clone)]
pub struct SliderTooltip {
    pub text_style: TextStyle,
    pub color: Color,
    pub corner_radius: CornerRadius,
}

/// Marker component for Slider's thumb
#[derive(Component)]
pub struct SliderThumbNode;

/// Marker component for Slider's track
#[derive(Component)]
pub struct SliderTrackNode;

/// Marker component for Slider's tooltip
#[derive(Component)]
pub struct SliderTooltipNode;

/// Marker component for Slider's tooltip text
#[derive(Component)]
pub struct SliderTooltipTextNode;

/// Marker component added to slider thumb while it's getting moved.
#[derive(Component)]
pub struct SliderThumbActive;

/// Added to an entity that's managed by a Widget.
/// It references the root entity for that Widget. The root is not necessarily the direct parent
/// of the node and in some cases it could be under another tree entirely (e.g. tooltips).
#[derive(Component)]
pub struct WidgetRoot(Entity);

fn slider_test(query: Query<&Node, Added<SliderTooltip>>) {
    for node in query.iter() {
        println!("{:?}", node);
    }
}

fn slider_thumb_update(
    mut thumb_q: Query<(&WidgetRoot, &Node, &mut Style), With<SliderThumbNode>>,
    track_q: Query<
        (
            &WidgetRoot,
            &Node,
            &GlobalTransform,
            Option<&CalculatedClip>,
        ),
        With<SliderTrackNode>,
    >,
    slider_q: Query<&Slider, Or<(Changed<Slider>, Changed<Node>, Changed<GlobalTransform>)>>,
) {
    for (root, thumb_node, mut thumb_style) in thumb_q.iter_mut() {
        let thumb_width = thumb_node.size.y;

        if let Ok(slider) = slider_q.get(root.0) {
            if let Some((_, node, global_transform, clip)) = track_q
                .iter()
                .find(|(track_root, ..)| track_root.0 == root.0)
            {
                //println!("Updated thumb");

                let (min, max) = get_uinode_clipped_rect(global_transform, node, clip);
                let x = (slider.value as f32 * (max.x - min.x)) / (slider.max - slider.min) as f32;

                thumb_style.position = Rect {
                    left: Val::Px(x - thumb_width / 2.0),
                    ..Default::default()
                };
            }
        }
    }
}

fn slider_thumb_grab(
    mut commands: Commands,
    thumbs_q: Query<(Entity, &Interaction, Option<&SliderThumbActive>), Changed<Interaction>>,
) {
    for (entity, interaction, active) in thumbs_q.iter() {
        match interaction {
            Interaction::Clicked => {
                commands.entity(entity).insert(SliderThumbActive);
            }
            Interaction::Hovered | Interaction::None if active.is_some() => {
                commands.entity(entity).remove::<SliderThumbActive>();
            }
            _ => (),
        }
    }
}

fn slider_thumb_move(
    thumb_q: Query<&WidgetRoot, With<SliderThumbActive>>,
    track_q: Query<
        (
            &WidgetRoot,
            &Node,
            &GlobalTransform,
            Option<&CalculatedClip>,
        ),
        With<SliderTrackNode>,
    >,
    mut slider_q: Query<&mut Slider>,
    windows: Res<Windows>,
) {
    let cursor_position = windows
        .get_primary()
        .and_then(|window| window.cursor_position());

    for root in thumb_q.iter() {
        if let Ok(mut slider) = slider_q.get_mut(root.0) {
            if let Some((_, node, global_transform, clip)) = track_q
                .iter()
                .find(|(track_root, ..)| track_root.0 == root.0)
            {
                if let Some(cursor_position) = cursor_position {
                    let (min, max) = get_uinode_clipped_rect(global_transform, node, clip);
                    let x = f32::clamp(cursor_position.x, min.x, max.x) - min.x;
                    let mut value =
                        ((x * (slider.max - slider.min) as f32) / (max.x - min.x)) as i32;

                    assert!(slider.step > 0 && slider.step <= slider.max - slider.min);
                    if slider.step > 1 {
                        value = ((value + slider.step / 2) / slider.step) * slider.step;
                    }

                    if slider.value != value {
                        slider.value = value;
                    }
                }
            }
        }
    }
}

// Adds and removes tooltip nodes.
fn slider_tooltip(
    mut commands: Commands,
    added_slider_q: Query<(Entity, &SliderTooltip), Added<SliderTooltip>>,
    tooltip_q: Query<(Entity, &WidgetRoot)>,
    removed: RemovedComponents<SliderTooltip>,
) {
    for (root, slider_tooltip) in added_slider_q.iter() {
        let text = commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "0",
                    slider_tooltip.text_style.clone(),
                    Default::default(),
                ),
                ..Default::default()
            })
            .insert(SliderTooltipTextNode)
            .insert(WidgetRoot(root))
            .id();

        let tooltip = TooltipBuilder::new(&mut commands)
            .with_position(TooltipPosition::FollowCursor)
            .with_content(text)
            .spawn();

        commands
            .entity(tooltip)
            .insert(UiColor(slider_tooltip.color))
            .insert(slider_tooltip.corner_radius)
            .insert(SliderTooltipNode)
            .insert(WidgetRoot(root));
    }

    for entity in removed.iter() {
        if let Some((entity, _)) = tooltip_q.iter().find(|(_, root)| root.0 == entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Updates the internal tooltip text style
///
/// TODO: Fix the change detection query. Otherwise this copies the style object every frame.
fn slider_tooltip_update(
    slider_q: Query<&SliderTooltip /*, Or<(Changed<SliderTooltip>, Changed<Children>)>*/>,
    mut tooltip_q: Query<(
        &WidgetRoot,
        &TooltipUiNodes,
        &mut UiColor,
        &mut CornerRadius,
    )>,
    mut tooltip_text_q: Query<&mut Text, With<TooltipTextUiNode>>,
) {
    for (root, nodes, mut color, mut corner_radius) in tooltip_q.iter_mut() {
        if let Ok(mut text) = tooltip_text_q.get_mut(nodes.text) {
            if let Ok(slider_tooltip) = slider_q.get(root.0) {
                text.sections[0].style = slider_tooltip.text_style.clone();
                color.0 = slider_tooltip.color;
                *corner_radius = slider_tooltip.corner_radius;
            }
        }
    }
}

fn slider_tooltip_visibility(
    thumb_q: Query<
        (&WidgetRoot, &Interaction, Option<&SliderThumbActive>),
        Or<(Changed<Interaction>, Changed<SliderThumbActive>)>,
    >,
    mut tooltip_q: Query<(&WidgetRoot, &mut Style), With<SliderTooltipNode>>,
) {
    for (root, interaction, _) in thumb_q.iter() {
        if let Some((_, mut style)) = tooltip_q.iter_mut().find(|(r, ..)| r.0 == root.0) {
            style.display = match interaction {
                Interaction::Clicked | Interaction::Hovered => Display::Flex,
                _ => Display::None,
            };
        }
    }
}

fn slider_tooltip_text_update(
    mut tooltip_text_q: Query<(&WidgetRoot, &mut Text), With<SliderTooltipTextNode>>,
    slider_q: Query<&Slider, (With<SliderTooltip>, Changed<Slider>)>,
) {
    for (root, mut text) in tooltip_text_q.iter_mut() {
        if let Ok(slider) = slider_q.get(root.0) {
            println!("Changing text to {}", slider.value);
            text.sections[0].value = slider.value.to_string();
        }
    }
}

/// A UI node that is a slider
#[derive(Bundle, Clone, Debug)]
/// Describes the value properties of the slider
pub struct SliderBundle {
    pub slider: Slider,
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes the color of the node
    pub color: UiColor,
    /// Describes the image of the node
    pub image: UiImage,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Describes the radius of corners for the node
    pub corner_radius: CornerRadius,
    /// Describes the visual properties of the node's border
    pub border: Border,
}

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            slider: Default::default(),
            node: Default::default(),
            style: Self::default_style(),
            color: Color::NONE.into(),
            image: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            corner_radius: Default::default(),
            border: Default::default(),
        }
    }
}

impl SliderBundle {
    pub fn default_style() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Stretch,
            ..Default::default()
        }
    }
}
