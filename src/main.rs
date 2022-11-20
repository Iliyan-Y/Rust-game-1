use bevy::prelude::*;
// used for physics
use bevy_rapier2d::prelude::*;
// used for input capturing
use leafwing_input_manager::prelude::*;

fn main() {
  App::new()
    //.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        title: "My Game 1".into(),
        ..default()
      },
      ..default()
    }))
    .add_plugin(InputManagerPlugin::<Action>::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
    .insert_resource(RapierConfiguration {
      gravity: Vec2::new(5.0, -10.0),
      ..default()
    })
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(setup)
    .add_system(move_player)
    .run();
}

// ECS structure is used for query and accessing data like in SQL database
// E is like entity (id on row) in database
// C all other column
// S system (functions)
// when we spawn something it's entity and attaches components to it

#[derive(Component)]
struct Player;
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
  Move,
}

// runs once at start
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // 2D camera
  commands.spawn(Camera2dBundle::default());

  //Spawn Player
  commands
    .spawn(SpriteBundle {
      transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 1.0)),
      texture: asset_server.load("ball_blue_large.png"),
      ..default()
    })
    .insert(InputManagerBundle::<Action> {
      action_state: ActionState::default(),
      input_map: InputMap::new([(KeyCode::Space, Action::Move)]),
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(32.0))
    .insert(ExternalForce {
      force: Vec2::new(5.0, 1.0),
      torque: 0.0,
    })
    .insert(Damping {
      // air resistance
      linear_damping: 0.6,
      angular_damping: 5.0,
    })
    .insert(Restitution::coefficient(1.0)) // bounciness
    .insert(Player); // will tag the entity and we can query it in other systems
}

fn move_player(query: Query<&ActionState<Action>, With<Player>>) {
  let action_state = query.single();
  if action_state.just_pressed(Action::Move) {
    println!("MOVING")
  }
}
