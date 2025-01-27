use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_cursor, rotate_camera, spawn_tree))
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Tree model


}

fn spawn_tree(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Get the cursor position
        let Some(cursor_position) = windows.cursor_position() else {
            return;
        };

        // Get the camera and its global transform
        let (camera, camera_transform) = camera_query.single();

        // Calculate the ray from the camera through the cursor position
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Get the ground's transform to ensure correct positioning
        let ground_transform = ground_query.single();

        // Calculate the intersection of the ray with the ground plane
        let Some(distance) =
            ray.intersect_plane(ground_transform.translation(), InfinitePlane3d::new(ground_transform.up()))
        else {
            return;
        };

        let point = ray.get_point(distance);

        // Spawn the tree at the intersection point
        commands.spawn((
            SceneRoot(asset_server.load("Tree1.gltf#Scene0")), // Specify the scene within the GLTF file
            Transform {
                translation: point,
                scale: Vec3::splat(0.2), // Scale the tree to 20% of its original size
                ..Default::default()
            },
            GlobalTransform::default(),
        ));
    }
}

fn rotate_camera(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut camera_transform = camera_query.single_mut();

    let rotation_speed = std::f32::consts::PI / 2.0; // Radians per second
    let orbit_center = Vec3::ZERO; // Center of rotation (the origin)

    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        // Rotate counterclockwise
        rotate_around_point(&mut camera_transform, orbit_center, rotation_speed * time.delta_secs());
    }

    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        // Rotate clockwise
        rotate_around_point(&mut camera_transform, orbit_center, -rotation_speed * time.delta_secs());
    }
}

// Helper function to rotate a transform around a point
fn rotate_around_point(transform: &mut Transform, point: Vec3, angle: f32) {
    // Get the direction vector from the point to the camera
    let direction = transform.translation - point;

    // Rotate the direction vector around the Y-axis
    let rotation = Quat::from_rotation_y(angle);
    let new_direction = rotation * direction;

    // Update the camera position
    transform.translation = point + new_direction;

    // Keep the camera looking at the point
    transform.look_at(point, Vec3::Y);
}
