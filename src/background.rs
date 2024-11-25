use bevy::color::palettes::css::{BLACK, WHITE_SMOKE};
use bevy::color::Color;
use bevy::prelude::{BuildChildren, Camera2d, ChildBuild, Commands, Text};
use bevy::text::{TextColor, TextFont};
use bevy::ui::{
    AlignItems, BackgroundColor, BorderColor, BoxShadow, Display, FlexDirection, JustifyContent,
    Node, PositionType, UiRect, Val,
};
use bevy::utils::default;

const MAIN_COLOR: Color = Color::srgb(62.0 / 255.0, 209.0 / 255.0, 185.0 / 255.0);
const INSET_BOX_SHADOW: BoxShadow = BoxShadow {
    color: MAIN_COLOR,
    x_offset: Val::Px(0.0),
    y_offset: Val::Px(0.0),
    spread_radius: Val::Px(1.0),
    blur_radius: Val::Px(1.0),
};

const BORDER_PX: f32 = 8.0;
const SINGLE_GRID_PX: f32 = 25.0;
const GRID_WIDTH_BASE_PX: f32 = SINGLE_GRID_PX * 10.0;
const GRID_HEIGHT_BASE_PX: f32 = SINGLE_GRID_PX * 20.0;
const GRID_WIDTH_PX: f32 = GRID_WIDTH_BASE_PX + 2.0 * BORDER_PX;
const GRID_HEIGHT_PX: f32 = GRID_HEIGHT_BASE_PX + 2.0 * BORDER_PX;
const GRID_LINE_WIDTH_PX: f32 = 1.0;

const TITLE_FONT_SIZE: f32 = 15.0;

const SIDE_WIDTH_PX: f32 = 80.0;
const INNER_BOX_WIDTH_PX: f32 = SIDE_WIDTH_PX - 2.0 * BORDER_PX;
const HOLD_HEIGHT_PX: f32 = 80.0;
const NEXT_HEIGHT_PX: f32 = 200.0;
const SCORE_HEIGHT_PX: f32 = 150.0;

pub fn setup_screen(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(WHITE_SMOKE.into()),
        ))
        .with_children(|main| {
            main.spawn((
                Node {
                    // width: Val::Percent(100.0),
                    // height: Val::Percent(100.0),
                    margin: UiRect {
                        top: Val::Px(50.0),
                        ..default()
                    },
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                BackgroundColor(WHITE_SMOKE.into()),
            ))
            .with_children(|main_center| {
                // left
                main_center
                    .spawn((
                        Node {
                            // width: Val::Px(SIDE_WIDTH_PX),
                            margin: UiRect {
                                // right: Val::Px(-BORDER_PX),
                                bottom: Val::Px(BORDER_PX),
                                ..default()
                            },
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(MAIN_COLOR),
                    ))
                    .with_children(|left| {
                        left.spawn((
                            Node {
                                // width: Val::Px(SIDE_WIDTH_PX),
                                margin: UiRect {
                                    // left: Val::Px(-BORDER_PX),
                                    left: Val::Px(BORDER_PX),
                                    bottom: Val::Px(BORDER_PX),
                                    ..default()
                                },
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(MAIN_COLOR),
                        ))
                        .with_children(|left_hold| {
                            left_hold.spawn((
                                Text::new("HOLD"),
                                TextColor(BLACK.into()),
                                TextFont {
                                    font_size: TITLE_FONT_SIZE,
                                    ..default()
                                },
                            ));
                            left_hold.spawn((
                                Node {
                                    width: Val::Px(INNER_BOX_WIDTH_PX),
                                    height: Val::Px(HOLD_HEIGHT_PX),
                                    // margin: UiRect {
                                    //     bottom: Val::Px(BORDER_PX),
                                    //     ..default()
                                    // },
                                    ..default()
                                },
                                BackgroundColor(BLACK.into()),
                                INSET_BOX_SHADOW,
                            ));
                        });
                    });
                // grid
                main_center
                    .spawn((
                        Node {
                            width: Val::Px(GRID_WIDTH_PX),
                            height: Val::Px(GRID_HEIGHT_PX),
                            border: UiRect::all(Val::Px(BORDER_PX)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BorderColor(MAIN_COLOR),
                        BackgroundColor(BLACK.into()),
                    ))
                    .with_children(|grid| {
                        (0..19).for_each(|i| {
                            grid.spawn((
                                Node {
                                    width: Val::Px(GRID_WIDTH_BASE_PX),
                                    border: UiRect {
                                        top: Val::Px(GRID_LINE_WIDTH_PX),
                                        ..default()
                                    },
                                    margin: UiRect {
                                        top: Val::Px(SINGLE_GRID_PX - GRID_LINE_WIDTH_PX),
                                        ..default()
                                    },
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(SINGLE_GRID_PX * i as f32),
                                    ..default()
                                },
                                BorderColor(WHITE_SMOKE.into()),
                            ));
                        });
                        (1..10).for_each(|i| {
                            grid.spawn((
                                Node {
                                    height: Val::Px(GRID_HEIGHT_BASE_PX),
                                    border: UiRect {
                                        right: Val::Px(GRID_LINE_WIDTH_PX),
                                        ..default()
                                    },
                                    margin: UiRect {
                                        right: Val::Px(SINGLE_GRID_PX - GRID_LINE_WIDTH_PX),
                                        ..default()
                                    },
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(SINGLE_GRID_PX * i as f32),
                                    ..default()
                                },
                                BorderColor(WHITE_SMOKE.into()),
                            ));
                        });
                    });
                // right
                main_center
                    .spawn((
                        Node {
                            // width: Val::Px(SIDE_WIDTH_PX),
                            height: Val::Px(GRID_HEIGHT_PX),
                            // margin: UiRect {
                            //     left: Val::Px(-BORDER_PX),
                            //     ..default()
                            // },
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(MAIN_COLOR),
                    ))
                    .with_children(|right| {
                        right
                            .spawn((
                                Node {
                                    // width: Val::Px(SIDE_WIDTH_PX),
                                    margin: UiRect {
                                        // left: Val::Px(-BORDER_PX),
                                        right: Val::Px(BORDER_PX),
                                        bottom: Val::Px(BORDER_PX),
                                        ..default()
                                    },
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(MAIN_COLOR),
                            ))
                            .with_children(|right_next| {
                                right_next.spawn((
                                    Text::new("NEXT"),
                                    TextColor(BLACK.into()),
                                    TextFont {
                                        font_size: TITLE_FONT_SIZE,
                                        ..default()
                                    },
                                ));
                                right_next.spawn((
                                    Node {
                                        width: Val::Px(INNER_BOX_WIDTH_PX),
                                        height: Val::Px(NEXT_HEIGHT_PX),
                                        // margin: UiRect {
                                        //     bottom: Val::Px(BORDER_PX),
                                        //     ..default()
                                        // },
                                        ..default()
                                    },
                                    BackgroundColor(BLACK.into()),
                                    INSET_BOX_SHADOW,
                                ));
                            });
                        right
                            .spawn((
                                Node {
                                    // width: Val::Px(SIDE_WIDTH_PX),
                                    margin: UiRect {
                                        // left: Val::Px(-BORDER_PX),
                                        right: Val::Px(BORDER_PX),
                                        bottom: Val::Px(BORDER_PX),
                                        ..default()
                                    },
                                    display: Display::Flex,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(MAIN_COLOR),
                            ))
                            .with_children(|right_score| {
                                right_score.spawn((
                                    Text::new("SCORE"),
                                    TextColor(BLACK.into()),
                                    TextFont {
                                        font_size: TITLE_FONT_SIZE,
                                        ..default()
                                    },
                                ));
                                right_score.spawn((
                                    Node {
                                        width: Val::Px(INNER_BOX_WIDTH_PX),
                                        height: Val::Px(SCORE_HEIGHT_PX),
                                        // margin: UiRect {
                                        //     bottom: Val::Px(BORDER_PX),
                                        //     ..default()
                                        // },
                                        ..default()
                                    },
                                    BackgroundColor(BLACK.into()),
                                    INSET_BOX_SHADOW,
                                ));
                            });
                    });
            });
        });
}
