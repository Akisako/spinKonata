use bevy::prelude::*;

#[derive(Component)]
struct KonataRotate 
{
    rotation_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spin_konata)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let konata_handle = asset_server.load("imagekonata.png");

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(konata_handle),
        KonataRotate {
            rotation_speed: f32::to_radians(360.0),
        },
    ));
}

fn spin_konata(time: Res<Time>, query: Single<(&KonataRotate, &mut Transform)>,)
{
    let (konata, mut transform) = query.into_inner();

    transform.rotate_z(1.0 * konata.rotation_speed * time.delta_secs());
    //transform.rotate_x(1.0 * konata.rotation_speed * time.delta_secs());
}