use bevy::{app::AppExit, core::FixedTimestep, prelude::*};

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_startup_system(setup.system())
        .add_system(count_children.system())
        .add_system(despawn_children.system())
        .add_system_set(
            SystemSet::new()
                .with_system(stop.system())
                .with_run_criteria(FixedTimestep::step(0.01)),
        )
        .run();
}

struct SomeParent {}
struct SomeChild {}

fn stop(mut exit_events: EventWriter<AppExit>) {
    exit_events.send(AppExit);
}

fn despawn_children(mut commands: Commands, child_query: Query<(&SomeChild, Entity)>) {
    // First time 3, after that 0. (All good)
    println!("child_query.iter().len(): {}", child_query.iter().len());
    // Despawns all children
    child_query
        .iter()
        .for_each(|(_, e)| commands.entity(e).despawn());
}

fn count_children(q: Query<&Children, With<SomeParent>>) {
    // First time 3, after that... 3 ???
    q.iter().for_each(|children| {
        println!("children.len(): {}", children.len());
    });
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle((SomeParent {},)).with_children(|p| {
        p.spawn_bundle((SomeChild {},));
        p.spawn_bundle((SomeChild {},));
        p.spawn_bundle((SomeChild {},));
    });
}
