use crate::{input::*, lobby::LobbyPlugin};
use args::*;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ggrs::{
    prelude::*, GgrsComponentChecksumHashPlugin, GgrsComponentMapEntitiesPlugin,
    GgrsComponentSnapshotClonePlugin, GgrsResourceSnapshotClonePlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_matchbox::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

mod args;
mod input;
mod lobby;

const FPS: usize = 60;

pub type GgrsConfig = bevy_ggrs::GgrsConfig<InferiInput, PeerId>;

#[derive(Component)]
struct Actor;

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

/// just used for desync detection for now
#[derive(Component, Clone, Copy, Default, Reflect)]
#[reflect(Component, Hash)]
struct PrevPos(Vec2);

#[derive(Component)]
pub struct MainCamera;

impl std::hash::Hash for PrevPos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
    }
}

#[derive(Resource, Clone, Copy, Debug, Default, Reflect, Hash, Deref, DerefMut)]
#[reflect(Resource, Hash)]
struct FrameCount {
    frame: usize,
}

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_scene(mut commands: Commands, frame: Res<FrameCount>, asset_server: Res<AssetServer>) {
    // Spawn box arround players, this code is found in many places like xpdb example programs
    if **frame != 0 {
        return;
    }

    info!("Setting up scene");
    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Spawn the background sprite
    // Hard coded the size for now, same with img cause we dont have a secondary stage
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Stage Backgrounds/Background.png"),
        transform: Transform::from_scale(Vec3::new(1.76, 2.2, 1.0)),
        ..Default::default()
    });
    // Ceiling
    commands
        .spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_scale(Vec3::new(20.0, 1.0, 1.0)),
                ..default()
            },
            RigidBody::Static,
            Position(Vector::Y * 50.0 * 6.0),
            Collider::cuboid(50.0 * 20.0, 50.0),
        ))
        .add_rollback();

    // Floor
    commands
        .spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_scale(Vec3::new(20.0, 1.0, 1.0)),
                ..default()
            },
            RigidBody::Static,
            Position(Vector::NEG_Y * 50.0 * 6.0),
            Collider::cuboid(50.0 * 20.0, 50.0),
        ))
        .add_rollback();

    // Left wall
    commands
        .spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_scale(Vec3::new(1.0, 11.0, 1.0)),
                ..default()
            },
            RigidBody::Static,
            Position(Vector::NEG_X * 50.0 * 9.5),
            Collider::cuboid(50.0, 50.0 * 11.0),
        ))
        .add_rollback();

    // Right wall
    commands
        .spawn((
            SpriteBundle {
                sprite: square_sprite,
                transform: Transform::from_scale(Vec3::new(1.0, 11.0, 1.0)),
                ..default()
            },
            RigidBody::Static,
            Position(Vector::X * 50.0 * 9.5),
            Collider::cuboid(50.0, 50.0 * 11.0),
        ))
        .add_rollback();
}

fn spawn_characters(
    mut commands: Commands,
    frame_count: Res<FrameCount>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if **frame_count != 0 {
        info!("not spawning characters on frame {frame_count:?}");
        return;
    }
    info!("Spawning characters");

    // Load the combined texture containing both default character and punch animation
    let texture_handle = asset_server.load("fighters/redNinja.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(64.0, 64.0),
        5,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let default_indices = AnimationIndices { first: 0, last: 0 }; //dont want it to play any animation at the begining

    // Spawn Player 1
    commands
        .spawn((
            Player { handle: 0 },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(default_indices.first),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..Default::default()
            },
            default_indices.clone(),
            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Position(Vector::new(-150.0, 0.0)))
        .insert(PrevPos(Vector::new(-150.0, 0.0)))
        .insert(Rotation::default())
        .insert(Collider::cuboid(30.0, 50.0))
        .insert(Actor)
        .insert(LockedAxes::ROTATION_LOCKED);

    // Spawn player 2
    commands
        .spawn((
            Player { handle: 1 },
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(default_indices.first),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..Default::default()
            },
            default_indices,
            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Position(Vector::new(-150.0, 0.0)))
        .insert(PrevPos(Vector::new(-150.0, 0.0)))
        .insert(Rotation::default())
        .insert(Collider::cuboid(30.0, 50.0))
        .insert(Actor)
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn handle_player_input(
    time: Res<Time>,
    inputs: Res<PlayerInputs<GgrsConfig>>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        Option<&mut LinearVelocity>,
        &Player,
    )>,
) {
    for (mut indices, mut timer, mut sprite, velocity, player) in &mut query.iter_mut() {
        if let Some(input) = inputs.get(player.handle) {
            let buttons = input.0.buttons;

            //let buttons = inputs.iter().next().map_or(0, |input| input.0.buttons);

            // Handle movement logic
            if let Some(mut linear_velocity) = velocity {
                // Handles movement(speed)
                if buttons & INPUT_UP_JUST_PRESSED != 0 {
                    linear_velocity.y += 125.0;
                }
                if buttons & INPUT_DOWN != 0 {
                    linear_velocity.y -= 10.0;
                }
                if buttons & INPUT_LEFT != 0 {
                    linear_velocity.x -= 10.0;
                }
                if buttons & INPUT_RIGHT != 0 {
                    linear_velocity.x += 10.0;
                }

                // Handles changing directions
                if buttons & INPUT_LEFT_JUST_PRESSED != 0 {
                    sprite.flip_x = true;
                }

                if buttons & INPUT_RIGHT_JUST_PRESSED != 0 {
                    sprite.flip_x = false;
                }
            }

            // Check if the punch button is just pressed in the current frame
            let punch_pressed = buttons & INPUT_PUNCH_JUST_PRESSED != 0;

            // Conditionally update animation_indices based on button press
            if punch_pressed {
                // Adjust the start and end index based on your sprite sheet
                indices.first = 1;
                indices.last = 4;
            }

            // Update the sprite index based on the animation_indices
            timer.tick(time.delta());
            if timer.just_finished() {
                if sprite.index == indices.last {
                    sprite.index = 0;
                    indices.first = 0;
                    indices.last = 0;
                } else {
                    sprite.index += 1
                };
            }
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Startup,
    Lobby,
    InGame,
    Paused,
}

#[derive(ScheduleLabel, Clone, Debug, Hash, Eq, PartialEq)]
struct PhysicsSchedule;

fn main() {
    // read query string or command line arguments
    let args = Args::get();
    info!("{args:?}");

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::new(PhysicsSchedule),
            FrameTimeDiagnosticsPlugin,
            LobbyPlugin,
            WorldInspectorPlugin::default(),
        ))
        .add_plugins(GgrsPlugin::<GgrsConfig>::default())
        .add_systems(ReadInputs, input)
        // Ggrs stuff is for handling online rollback multiplayer desync detection
        .add_plugins(GgrsComponentSnapshotClonePlugin::<Transform>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<Position>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<PreviousPosition>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<LinearVelocity>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<Rotation>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<PreviousRotation>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<AngularVelocity>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<DistanceJoint>::default())
        .add_plugins(GgrsComponentMapEntitiesPlugin::<DistanceJoint>::default())
        .add_plugins(GgrsComponentSnapshotClonePlugin::<PrevPos>::default())
        .add_plugins(GgrsComponentChecksumHashPlugin::<PrevPos>::default())
        .add_plugins(GgrsResourceSnapshotClonePlugin::<FrameCount>::default())
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(SubstepCount(12)) // default value is 12, lower if desync issues
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(PhysicsTimestep::FixedOnce(1. / FPS as f32))
        .init_resource::<FrameCount>()
        // Some of our systems need the query parameters
        .insert_resource(args)
        .add_state::<AppState>()
        .add_systems(Startup, (setup, setup_scene, spawn_characters).chain())
        .add_systems(Update, log_ggrs_events.run_if(in_state(AppState::InGame)))
        // these systems will be executed as part of the advance frame update
        .add_systems(
            GgrsSchedule,
            (
                //should have those as part of rollback but has issues atm so doing them on startup
                //setup_scene,
                //spawn_characters,
                step_physics,
                handle_player_input,
                update_previous_position,
                increase_frame_system,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>, args: Res<Args>) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
    if args.players == 1 {
        info!("starting synctest session");
        let mut session_builder = configure_session(1);
        session_builder = session_builder
            .add_player(PlayerType::Local, 0)
            .expect("failed to add player");
        let session = session_builder
            .start_synctest_session()
            .expect("failed to start synctest session");
        commands.insert_resource(Session::SyncTest(session));
        app_state.set(AppState::InGame)
    } else {
        info!("joining multiplayer lobby");
        app_state.set(AppState::Lobby)
    }
}

pub fn configure_session(players: usize) -> SessionBuilder<GgrsConfig> {
    SessionBuilder::<GgrsConfig>::new()
        .with_num_players(players)
        .with_max_prediction_window(12)
        // TODO: re-enable input delay when rollbacks are working properly
        // .with_input_delay(2)
        .with_input_delay(0)
        .with_fps(FPS)
        .expect("invalid fps")
}

fn log_ggrs_events(mut session: ResMut<Session<GgrsConfig>>) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                info!("GGRS Event: {event:?}");
                if let GgrsEvent::DesyncDetected { .. } = event {
                    panic!("desynced!");
                }
            }
        }
        Session::SyncTest(_) => {}
        _ => panic!("This example focuses on p2p and synctest"),
    }
}

fn increase_frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

fn update_previous_position(mut positions: Query<(&mut PrevPos, &Position)>) {
    for (mut previous_position, position) in &mut positions {
        previous_position.0 = position.0;
    }
}

fn step_physics(world: &mut World) {
    world.run_schedule(PhysicsSchedule);
}
