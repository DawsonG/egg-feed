use bevy::{
    color::palettes::css::WHITE,
    input::{common_conditions::input_just_released},
    prelude::*,
    window::PrimaryWindow,
};
use std::fmt::Debug;

#[derive(Resource, Default)]
struct GameState {
    score: u32,
    has_egg: bool,
}

// see https://bevy.org/examples/picking/sprite-picking/
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<GameState>()
        .add_systems(Startup, (setup, set_window_title))
        .add_systems(
            Update,
            (
                handle_release.run_if(input_just_released(MouseButton::Left)),
                game_tick,
            ),
        )
        .run();
}

#[derive(Component)]
struct HoverSprites {
    default: Handle<Image>,
    hovered: Handle<Image>,
}

#[derive(Component, Debug)]
struct Eggy;

#[derive(Component)]
struct Egg;

#[derive(Component)]
struct ScoreText;

fn toggle_mouth<E: EntityEvent + Debug + Clone + Reflect>()
-> impl Fn(On<E>, Query<&mut Sprite>, Query<&mut HoverSprites>, Res<GameState>) {
    move |ev, mut sprite, mut hover_sprites, game_state| {
        let mut sprite = sprite.get_mut(ev.event_target()).unwrap();
        let hover_sprites = hover_sprites.get_mut(ev.event_target()).unwrap();

        if sprite.image == hover_sprites.default && game_state.has_egg {
            sprite.image = hover_sprites.hovered.clone();
        } else {
            sprite.image = hover_sprites.default.clone();
        }
    }
}

fn grab_egg<E: EntityEvent + Debug + Clone + Reflect>() -> impl Fn(On<E>, ResMut<GameState>) {
    move |_ev, mut game_state| {
        game_state.has_egg = true;
    }
}

fn drop_egg<E: EntityEvent + Debug + Clone + Reflect>()
-> impl Fn(On<E>, Query<&mut Transform, With<Egg>>, ResMut<GameState>) {
    move |_ev, mut q_egg, mut game_state| {
        let event_type = std::any::type_name::<E>();
        if event_type.contains("Release") && game_state.has_egg {
            game_state.has_egg = false;
            game_state.score += 1;
            if let Ok(mut transform) = q_egg.single_mut() {
                transform.translation.x = -5000.0;
            }
            println!("Score: {}", game_state.score);
        }
    }
}

fn handle_release(mut game_state: ResMut<GameState>) {
    game_state.has_egg = false;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    let eggy_closed = asset_server.load("eggy_pixel.png");
    let eggy_open = asset_server.load("eggy_open_pixel.png");
    commands.spawn(Camera2d);

    // Set score and egg holding state
    game_state.score = 0;
    game_state.has_egg = false;

    commands.spawn((
        Sprite::from_image(asset_server.load("egg_pixel.png")),
        Transform::from_xyz(-5000.0, 0.0, 1.0).with_scale(Vec3::splat(8.0)), // way offscreen
        Egg,
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
    ));

    commands
        .spawn((
            Name::new("Eggy"),
            Sprite {
                image: eggy_closed.clone(),
                ..default()
            },
            Transform::from_xyz(-500.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)),
            Eggy,
            HoverSprites {
                default: eggy_closed,
                hovered: eggy_open,
            },
            Pickable::default(),
        ))
        .observe(toggle_mouth::<Pointer<Over>>()) // change image to open mouth
        .observe(toggle_mouth::<Pointer<Out>>())
        .observe(drop_egg::<Pointer<Release>>()); // add point

    commands
        .spawn((
            Sprite::from_image(asset_server.load("egg_bowl_pixel.png")),
            Transform::from_xyz(500.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)),
            Pickable::default(),
        ))
        .observe(grab_egg::<Pointer<Press>>());

    commands
        .spawn((
            Text::new("Eggs fed: "),
            TextFont {
                font: asset_server.load("slkscrb.ttf"),
                font_size: 42.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                bottom: px(5),
                left: px(15),
                ..Default::default()
            },
        )).with_child((
            TextSpan::default(),
            TextFont {
                font: asset_server.load("slkscrb.ttf"),
                font_size: 46.0,
                ..default()
            },
            TextColor(WHITE.into()),
            ScoreText,
        ));
}

fn game_tick(
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    game_state: ResMut<GameState>,
    mut q_egg: Query<&mut Transform, With<Egg>>,
    mut q_score_text: Query<&mut TextSpan, With<ScoreText>>,
) {
    // update score text
    for mut span in &mut q_score_text {
        // Update the value of the second section
        **span = format!("{}", game_state.score.clone());
    }


    // If we're carrying an egg, show it to the player
    if game_state.has_egg {
        let (camera, camera_transform) = *q_camera;

        if let Some(cursor_pos) = window.cursor_position() {
            // Correct for potential custom viewports (important in 0.15+)
            let viewport_pos = if let Some(rect) = camera.logical_viewport_rect() {
                cursor_pos - rect.min
            } else {
                cursor_pos
            };

            // Convert the adjusted position to world space
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, viewport_pos) {
                if let Ok(mut transform) = q_egg.single_mut() {
                    transform.translation.x = world_pos.x;
                    transform.translation.y = world_pos.y;
                }
            }
        }
    } else {
        if let Ok(mut transform) = q_egg.single_mut() && transform.translation.y > -5000.0 {
            transform.translation.y -= 7.0;
        }
    }
}

fn set_window_title(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.title = "Feed the egg".to_string();
    }
}
