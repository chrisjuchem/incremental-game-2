use crate::recipe::{Materials, Recipe};
use crate::Recipes;
use bevy::prelude::*;

#[derive(Component)]
pub struct Factory {
    recipe: Option<Recipe>,
    progress: Option<Timer>,
    capacity: usize,
    storage: Materials,
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            recipe: None,
            progress: None,
            capacity: 10,
            storage: Default::default(),
        }
    }
}

impl Factory {
    pub fn with_recipe(mut self, recipe: Recipe) -> Self {
        self.recipe = Some(recipe);
        self
    }
}

#[derive(Resource, Deref)]
pub struct FactorySprite(pub Handle<Image>);

pub fn load_factory_sprite(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(FactorySprite(assets.load("sprites/factory.png")));
}

pub fn spawn_factories(mut commands: Commands, sprite: Res<FactorySprite>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-50.0, 0.0, 1.0).with_scale(Vec3::splat(0.2)),
            texture: sprite.0.clone(),
            ..default()
        },
        Factory::default().with_recipe(Recipe::Mine),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(50.0, 0.0, 1.0).with_scale(Vec3::splat(0.2)),
            texture: sprite.0.clone(),
            ..default()
        },
        Factory::default().with_recipe(Recipe::Smelt),
    ));
}

pub fn tick_factories(
    dt: Res<FixedTime>,
    mut factories: Query<&mut Factory>,
    recipes: Res<Recipes>,
    mut materials: ResMut<Materials>,
) {
    for mut factory in factories.iter_mut() {
        let Some(recipe) = factory.recipe else {continue};
        let recipe_info = recipes.get(&recipe).unwrap();

        if factory.progress.is_none() && materials.take(&recipe_info.inputs, 1) == 1 {
            factory.progress = Some(Timer::new(recipe_info.time, TimerMode::Repeating));
        }

        let Some(timer) = factory
            .progress
            .as_mut() else {continue};

        timer.tick(dt.period);

        let fin_count = timer.times_finished_this_tick() as usize;

        // Materials were already taken for this cycle, but we need to take them for the next time
        let mut actually_built = materials.take(&recipes[&recipe].inputs, fin_count);

        if actually_built < fin_count {
            actually_built += 1; // Oops, next time was actually this time!
            factory.progress = None; // We don't have enough resources to continue building
        }

        // TODO: add to factory storage instead of global, check capacity, click to collect
        // factory.storage.add(&recipe_info.outputs, actually_built);
        materials.add(&recipe_info.outputs, actually_built);
    }
}
