use bevy::{
    input::{ButtonState, common_conditions::input_just_released, mouse::MouseButtonInput},
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

fn toggle_mouth<E: Debug + Clone + Reflect>()
-> impl Fn(Trigger<E>, Query<&mut Sprite>, Query<&mut HoverSprites>, Res<GameState>) {
    move |ev, mut sprite, mut hover_sprites, game_state| {
        let mut sprite = sprite.get_mut(ev.target()).unwrap();
        let hover_sprites = hover_sprites.get_mut(ev.target()).unwrap();

        if sprite.image == hover_sprites.default && game_state.has_egg {
            sprite.image = hover_sprites.hovered.clone();
        } else {
            sprite.image = hover_sprites.default.clone();
        }
    }
}

fn toggle_egg<E: Debug + Clone + Reflect>()
-> impl Fn(Trigger<E>, Query<&mut Sprite>, ResMut<GameState>) {
    move |ev, mut sprites, mut game_state| {
        let event_type = std::any::type_name::<E>();
        println!("Event type: {}", event_type);
        if event_type.contains("Released") && game_state.has_egg {
            println!("Releasing egg");
            game_state.has_egg = false;
            game_state.score += 1;
            println!("Score: {}", game_state.score);
        } else {
            let Ok(_sprite) = sprites.get_mut(ev.target()) else {
                return;
            };
            println!("Grabbing egg");
            game_state.has_egg = true;
        }
    }
}

fn handle_release(mut game_state: ResMut<GameState>) {
    println!("Handling release");
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
        .observe(toggle_egg::<Pointer<Released>>()); // add point

    commands
        .spawn((
            Sprite::from_image(asset_server.load("egg_bowl_pixel.png")),
            Transform::from_xyz(500.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)),
            Pickable::default(),
        ))
        .observe(toggle_egg::<Pointer<Pressed>>());

    commands.spawn((
        Sprite::from_image(asset_server.load("egg_pixel.png")),
        Transform::from_xyz(-5000.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)), // way offscreen
        Egg,
        Pickable::default(),
    ));
}

fn game_tick(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    game_state: Res<GameState>,
    mut q_egg: Query<&mut Transform, With<Egg>>,
) {
    // TODO: If mouse button is released, toggle egg state
    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Released {
            toggle_egg::<Pointer<Released>>();
        }
    }

    if game_state.has_egg {
        let mut follower_transform = q_egg.single_mut();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera
                .get()
                .viewport_to_world_2d(camera.get().transform, cursor_pos)
            {
                follower_transform.unwrap().translation = Vec3::new(world_pos.x, world_pos.y, 0.0);
            }
        }
    }
}

fn set_window_title(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.title = "Feed the egg".to_string();
    }
}
