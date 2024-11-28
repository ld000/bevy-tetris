use bevy::asset::Assets;
use bevy::color::palettes::css::{BLACK, WHITE_SMOKE};
use bevy::color::{Color, Gray, LinearRgba};
use bevy::math::{Isometry3d, UVec2, Vec2, Vec3};
use bevy::prelude::{
    BuildChildren, Camera2d, ChildBuild, Commands, Gizmos, Mesh, Mesh2d, Query, Rectangle, ResMut,
    Text, Transform, With,
};
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use bevy::text::{TextColor, TextFont};
use bevy::ui::{AlignItems, BorderColor, Display, JustifyContent, Node, PositionType, UiRect, Val};
use bevy::utils::default;
use bevy::window::{PrimaryWindow, Window};

const MAIN_COLOR: Color = Color::srgb(62.0 / 255.0, 209.0 / 255.0, 185.0 / 255.0);
const INNER_WINDOW_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

const BORDER_WIDTH: f32 = 8.0;
const SINGLE_GRID_SIZE: f32 = 25.0;
const GRID_WIDTH: f32 = SINGLE_GRID_SIZE * 10.0;
const GRID_HEIGHT: f32 = SINGLE_GRID_SIZE * 20.0;
const CENTER_BOX_WIDTH: f32 = GRID_WIDTH;
const CENTER_BOX_HEIGHT: f32 = GRID_HEIGHT + 2.0 * BORDER_WIDTH;

const TITLE_FONT_SIZE: f32 = 15.0;
const TEXT_BOX_HEIGHT: f32 = 30.0;

const SIDE_BOX_WIDTH: f32 = 80.0;
const SIDE_INNER_BOX_WIDTH: f32 = SIDE_BOX_WIDTH - 2.0 * BORDER_WIDTH;
const HOLD_BOX_HEIGHT: f32 = 80.0;
const NEXT_BOX_HEIGHT: f32 = 200.0;
const SCORE_BOX_HEIGHT: f32 = 220.0;

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_camera(&mut commands);
    spawn_background(&mut commands, &mut meshes, &mut materials, window_query);
}

pub fn setup_background_grid(mut gizmos: Gizmos) {
    gizmos.grid(
        Isometry3d::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        UVec2::new(10, 20),
        Vec2::new(25.0, 25.0),
        LinearRgba::gray(0.05),
    );
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d);
}

fn spawn_background(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    // center
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(CENTER_BOX_WIDTH, CENTER_BOX_HEIGHT))),
            MeshMaterial2d(materials.add(MAIN_COLOR)),
        ))
        .with_children(|center| {
            center
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(GRID_WIDTH, GRID_HEIGHT))),
                    MeshMaterial2d(materials.add(INNER_WINDOW_COLOR)),
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ))
                // .with_children(|center_grid| {
                //     let line = Segment2d::new(
                //         Dir2::from_xy(-GRID_WIDTH / 2.0 + SINGLE_GRID_SIZE, -GRID_HEIGHT / 2.0)
                //             .unwrap(),
                //         GRID_HEIGHT,
                //     );

                //     // (0..19).for_each(|i| {
                //     //     center_grid.spawn((
                //     //         Mesh2d(
                //     //             meshes.add(),
                //     //         ),
                //     //         MeshMaterial2d(materials.add(Color::WHITE)),
                //     //     ));
                //     // });
                //     // (1..10).for_each(|i| {
                //     //     center_grid.spawn((
                //     //         Mesh2d(meshes.add(Rectangle::new(GRID_LINE_WIDTH_PX, GRID_HEIGHT_BASE_PX))),
                //     //         MeshMaterial2d(materials.add(WHITE_SMOKE)),
                //     //     ));
                // })
                ;
        });

    // left
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(SIDE_BOX_WIDTH, CENTER_BOX_HEIGHT))),
            MeshMaterial2d(materials.add(MAIN_COLOR)),
            Transform::from_xyz(-(CENTER_BOX_WIDTH / 2.0 + SIDE_BOX_WIDTH / 2.0), 0.0, 0.0),
        ))
        .with_children(|left| {
            // left.spawn((
            //     Text::new("HOLD"),
            //     TextColor(BLACK.into()),
            //     TextFont {
            //         font_size: TITLE_FONT_SIZE,
            //         ..default()
            //     },
            //     // Transform::from_xyz(-(GRID_WIDTH_PX / 2.0 + SIDE_WIDTH_PX / 2.0), 0.0, 0.0),
            // ));
            left.spawn((
                Mesh2d(meshes.add(Rectangle::new(SIDE_INNER_BOX_WIDTH, HOLD_BOX_HEIGHT))),
                MeshMaterial2d(materials.add(INNER_WINDOW_COLOR)),
                Transform::from_xyz(
                    0.0,
                    CENTER_BOX_HEIGHT / 2.0
                        - HOLD_BOX_HEIGHT / 2.0
                        - BORDER_WIDTH
                        - TEXT_BOX_HEIGHT,
                    0.1,
                ),
            ));
        });
    spawn_title(
        commands,
        "HOLD",
        window.height() / 2.0 - CENTER_BOX_HEIGHT / 2.0 + BORDER_WIDTH,
        window.width() / 2.0 - CENTER_BOX_WIDTH / 2.0 - SIDE_BOX_WIDTH,
    );

    // right
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(SIDE_BOX_WIDTH, CENTER_BOX_HEIGHT))),
            MeshMaterial2d(materials.add(MAIN_COLOR)),
            Transform::from_xyz(CENTER_BOX_WIDTH / 2.0 + SIDE_BOX_WIDTH / 2.0, 0.0, 0.0),
        ))
        .with_children(|right| {
            right.spawn((
                Mesh2d(meshes.add(Rectangle::new(SIDE_INNER_BOX_WIDTH, NEXT_BOX_HEIGHT))),
                MeshMaterial2d(materials.add(INNER_WINDOW_COLOR)),
                Transform::from_xyz(
                    0.0,
                    CENTER_BOX_HEIGHT / 2.0
                        - NEXT_BOX_HEIGHT / 2.0
                        - BORDER_WIDTH
                        - TEXT_BOX_HEIGHT,
                    0.1,
                ),
            ));
            right.spawn((
                Mesh2d(meshes.add(Rectangle::new(SIDE_INNER_BOX_WIDTH, SCORE_BOX_HEIGHT))),
                MeshMaterial2d(materials.add(INNER_WINDOW_COLOR)),
                Transform::from_xyz(
                    0.0,
                    -CENTER_BOX_HEIGHT / 2.0 + SCORE_BOX_HEIGHT / 2.0 + BORDER_WIDTH,
                    0.1,
                ),
            ));
        });
    spawn_title(
        commands,
        "NEXT",
        window.height() / 2.0 - CENTER_BOX_HEIGHT / 2.0 + BORDER_WIDTH,
        window.width() / 2.0 + CENTER_BOX_WIDTH / 2.0,
    );
    spawn_title(
        commands,
        "SCORE",
        window.height() / 2.0 + CENTER_BOX_HEIGHT / 2.0
            - SCORE_BOX_HEIGHT
            - BORDER_WIDTH
            - TEXT_BOX_HEIGHT,
        window.width() / 2.0 + CENTER_BOX_WIDTH / 2.0,
    );
}

fn spawn_title(commands: &mut Commands, title: &str, top: f32, left: f32) {
    commands
        .spawn((
            Node {
                width: Val::Px(SIDE_BOX_WIDTH),
                height: Val::Px(TEXT_BOX_HEIGHT),
                position_type: PositionType::Absolute,
                top: Val::Px(top),
                left: Val::Px(left),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                border: UiRect::all(Val::Px(1.0)), // for test
                ..default()
            },
            BorderColor(WHITE_SMOKE.into()), // for test
        ))
        .with_children(|text_node| {
            text_node.spawn((
                Text::new(title),
                TextColor(BLACK.into()),
                TextFont {
                    font_size: TITLE_FONT_SIZE,
                    ..default()
                },
            ));
        });
}
