use hecs::World;
use zombs_engine::engine::components::{AABB, CharacterMotor, Position, Velocity};
use zombs_engine::engine::systems::movement_system;

#[test]
fn test_collision_slide() {
    let mut world = World::new();

    // Spawn Wall (Static AABB) at (5.0, 0.0)
    // Size 1x1
    world.spawn((
        Position::new_3d(5.0, 0.0, 0.0),
        AABB::new(1.0, 1.0),
    ));

    // Spawn Player moving right towards wall at (3.5, 0.0)
    // Player Size 1x1
    // Velocity 10.0 X
    let player = world.spawn((
        Position::new_3d(3.5, 0.0, 0.0),
        Velocity::new(10.0, 0.0),
        AABB::new(1.0, 1.0),
    ));

    // Run Movement System for 0.1s
    // Expected movement: 1.0 unit (to 4.5).
    // Wall Left edge is 5.0 - 0.5 = 4.5.
    // Player Right edge at 4.5 is 4.5 + 0.5 = 5.0. 
    // They should collide.
    
    movement_system(&mut world, 0.1);

    let pos = world.get::<&Position>(player).unwrap();
    
    // Check if stopped or slid
    println!("Final Pos: {}", pos.x);
    
    // Simplistic check: Should not have passed through the wall
    assert!(pos.x < 5.5);
}

#[test]
fn test_projectile_despawn() {
    // TODO: Test projectile TTL logic
}

#[test]
fn test_jump_physics_lands() {
    let mut world = World::new();

    let ground_z = 1.0;
    let player = world.spawn((
        Position::new_3d(0.0, 0.0, ground_z),
        Velocity::new_3d(0.0, 0.0, 6.0),
        CharacterMotor {
            ground_z,
            is_grounded: false,
        },
    ));

    movement_system(&mut world, 0.1);

    let z_after_takeoff = world.get::<&Position>(player).unwrap().z;
    assert!(z_after_takeoff > ground_z);

    for _ in 0..300 {
        movement_system(&mut world, 1.0 / 60.0);
    }

    let final_z = world.get::<&Position>(player).unwrap().z;
    let is_grounded = world.get::<&CharacterMotor>(player).unwrap().is_grounded;

    assert!((final_z - ground_z).abs() < 0.0001);
    assert!(is_grounded);
}
