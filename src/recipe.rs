use bevy::{
    prelude::{Deref, DerefMut, Resource},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Material {
    Ore,
    Iron,
}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Material::Ore => "ore",
            Material::Iron => "iron",
        })
    }
}

#[derive(Resource, Deref, DerefMut, Default, Serialize, Deserialize)]
pub struct Materials(HashMap<Material, usize>);

impl Materials {
    pub fn take(&mut self, amounts: &Materials, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        let amnt = min(
            max,
            amounts
                .iter()
                .map(|(material, n)| self[material] / n)
                .min()
                .unwrap_or(usize::MAX),
        );
        if amnt == 0 {
            return 0;
        }
        amounts
            .iter()
            .for_each(|(material, n)| *self.get_mut(material).unwrap() -= n * amnt);

        amnt
    }

    pub fn add(&mut self, amounts: &Materials, amnt: usize) {
        amounts
            .iter()
            .for_each(|(material, n)| *self.entry(*material).or_insert(0) += n * amnt);
    }

    pub fn default_populated() -> Self {
        Self(Material::iter().map(|m| (m, 0)).collect())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Recipe {
    Mine,
    Smelt,
}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Recipe::Mine => "mine",
            Recipe::Smelt => "smelt",
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecipeInfo {
    pub inputs: Materials,
    pub outputs: Materials,
    pub time: Duration,
}

#[derive(Resource, Deref, DerefMut, Serialize, Deserialize)]
pub struct Recipes(HashMap<Recipe, RecipeInfo>);

impl Default for Recipes {
    fn default() -> Self {
        Self(
            [
                (
                    Recipe::Mine,
                    RecipeInfo {
                        inputs: Default::default(),
                        outputs: Materials([(Material::Ore, 1)].into_iter().collect()),
                        time: Duration::from_secs_f32(3.0),
                    },
                ),
                (
                    Recipe::Smelt,
                    RecipeInfo {
                        inputs: Materials([(Material::Ore, 2)].into_iter().collect()),
                        outputs: Materials([(Material::Iron, 1)].into_iter().collect()),
                        time: Duration::from_secs_f32(5.0),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        )
    }
}
