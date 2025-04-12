use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(&'static str);

fn hello_world() {
    println!("Hello world");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Sans Undertale")));
    commands.spawn((Person, Name("Lorenzo Alvisi")));
    commands.spawn((Person, Name("PPAP")));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
