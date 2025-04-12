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

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "PPAP" {
            name.0 = "Pen Pineapple Apple Pen";
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
