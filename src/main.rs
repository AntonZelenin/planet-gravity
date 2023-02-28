use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Force(Vec3);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mass1 = Mass(18.);
    let velocity1 = Velocity(Vec3::new(0., -5., 0.));
    let mesh1 = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(2. * mass1.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ORANGE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    };

    let mass2 = Mass(4.);
    let velocity2 = Velocity(Vec3::new(0., 55., 0.));
    let mesh2 = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(2. * mass2.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(250., 50., 0.)),
        ..default()
    };

    let mass3 = Mass(2.);
    let velocity3 = Velocity(Vec3::new(0., -60., 0.));
    let mesh3 = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(2. * mass3.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(-200., 50., 0.)),
        ..default()
    };

    let mass4 = Mass(1.);
    let velocity4 = Velocity(Vec3::new(50., 55., 0.));
    let mesh4 = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(2. * mass4.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(250., 5., 0.)),
        ..default()
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        mesh1,
        Particle,
        mass1,
        velocity1,
        Force(Vec3::new(0., 0., 0.)),
    ));
    commands.spawn((
        mesh2,
        Particle,
        mass2,
        velocity2,
        Force(Vec3::new(0., 0., 0.)),
    ));
    commands.spawn((
        mesh3,
        Particle,
        mass3,
        velocity3,
        Force(Vec3::new(0., 0., 0.)),
    ));
    commands.spawn((
        mesh4,
        Particle,
        mass4,
        velocity4,
        Force(Vec3::new(0., 0., 0.)),
    ));
}

fn movement(
    time: Res<Time>,
    mut particles_query: Query<(&mut Velocity, &mut Transform, &mut Mass, &mut Force), With<Particle>>,
) {
    let mut combinations = particles_query.iter_combinations_mut();
    while let Some([p1, p2]) = combinations.fetch_next() {
        let (_velocity1, transform1, mass1, mut force_accum_1) = p1;
        let (_velocity2, transform2, mass2, mut force_accum_2) = p2;

        let force1 = calculate_force(&transform1, &transform2, &mass1, &mass2);
        force_accum_1.0 += force1;

        let force2 = calculate_force(&transform2, &transform1, &mass2, &mass1);
        force_accum_2.0 += force2;
    }

    for p in particles_query.iter_mut() {
        let (mut velocity, mut transform, mass, mut force) = p;
        let acceleration = force.0 / mass.0;
        velocity.0 += acceleration * time.delta_seconds();
        let translation_delta = velocity.0 * time.delta_seconds();
        transform.translation -= translation_delta;
        *force = Force(Vec3::new(0., 0., 0.));
    }
}

fn calculate_force(transform1: &Transform, transform2: &Transform, mass1: &Mass, mass2: &Mass) -> Vec3 {
    let mut distance = transform1.translation.distance(transform2.translation);
    if distance < 10. {
        distance = 10.;
    }
    let force = 40000. * mass1.0 * mass2.0 / distance.powi(2);
    let direction = transform1.translation - transform2.translation;
    direction.normalize() * force
}
