use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    heroes::SqlHero,
    location::SqlLocation,
    villains::SqlVillain,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fighters {
    pub hero: SqlHero,
    pub villain: SqlVillain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FightRequest {
    pub hero: SqlHero,
    pub villain: SqlVillain,
    pub location: SqlLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FightResult {
    id: String,
    fight_date: String, // use timestamp data structure?
    winner_name: String,
    winner_level: i32,
    winner_powers: String,
    winner_picture: String,
    loser_name: String,
    loser_level: i32,
    loser_powers: String,
    loser_picture: String,
    winner_team: String,
    loser_team: String,
    location: SqlLocation,
}

#[derive(Clone, Copy)]
pub enum Winner {
    Heroes,
    Villains,
}

impl FightResult {
    pub fn new(
        winner: Winner,
        hero: &SqlHero,
        villain: &SqlVillain,
        location: &SqlLocation,
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();
        let winner_name = match winner {
            Winner::Heroes => hero.name.clone(),
            Winner::Villains => villain.name.clone(),
        };
        let winner_level = match winner {
            Winner::Heroes => hero.level.clone(),
            Winner::Villains => villain.level.clone(),
        };
        let winner_powers = match winner {
            Winner::Heroes => hero.powers.clone(),
            Winner::Villains => villain.powers.clone(),
        };
        let winner_picture = match winner {
            Winner::Heroes => hero.picture.clone(),
            Winner::Villains => villain.picture.clone(),
        };

        let loser_name = match winner {
            Winner::Heroes => villain.name.clone(),
            Winner::Villains => hero.name.clone(),
        };
        let loser_level = match winner {
            Winner::Heroes => villain.level,
            Winner::Villains => hero.level,
        };
        let loser_powers = match winner {
            Winner::Heroes => villain.powers.clone(),
            Winner::Villains => hero.powers.clone(),
        };
        let loser_picture = match winner {
            Winner::Heroes => villain.picture.clone(),
            Winner::Villains => hero.picture.clone(),
        };
        let winner_team = match winner {
            Winner::Heroes => "heroes",
            Winner::Villains => "villains",
        };
        let loser_team = match winner {
            Winner::Villains => "heroes",
            Winner::Heroes => "villains",
        };

        FightResult {
            id: id.to_string(),
            fight_date: now,
            winner_name,
            winner_level,
            winner_powers,
            winner_picture,
            loser_name,
            loser_level,
            loser_powers,
            loser_picture,
            winner_team: winner_team.to_owned(),
            loser_team: loser_team.to_owned(),
            location: location.clone(),
        }
    }
}
// {
//     "id": "67dd8fbe355b694fbec93dee",
//     "fightDate": "2025-03-21T16:11:42.316336007Z",
//     "winnerName": "Desghidorah (Rebirth of Mothra)",
//     "winnerLevel": 1000,
//     "winnerPowers": "Animal Attributes, Cold Resistance, Energy Beams, Fire Control, Fire Resistance, Flight, Large Size, Lava Manipulation, Longevity, Magma Manipulation, Natural Armor, Natural Weapons, Toxin and Disease Resistance, Electricity Resistance, Electrokinesis, Endurance, Energy Absorption, Energy Blasts, Enhanced Senses, Explosion Manipulation, Heat Generation, Invulnerability, Self-Sustenance, Spaceflight, Death Touch, Energy Resistance, Radiation Immunity, Terrakinesis, Transformation, Weather Control",
//     "winnerPicture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/desghidorah-(rebirth-of-mothra)-7621412497910022678.jpg",
//     "loserName": "Jarad",
//     "loserLevel": 719,
//     "loserPowers": "Dark Magic, Dexterity, Enhanced Hearing, Marksmanship, Necromancy, Reflexes, Toxin and Disease Resistance, Zombie Physiology, Accelerated Healing, Agility, Death Touch, Endurance, Enhanced Sight, Gadget Usage, Grappling/Climbing, Hunters Instinct, Jump, Longevity, Possession Resistance, Toxin and Disease Control, Weapon-based Powers, Acrobatics, Animal Control, Curse Manipulation, Curse Resistance, Energy Blasts, Energy Manipulation, Immortality, Indomitable Will, Information Analysis, Magic, Magic Resistance, Plant Control, Possession, Shadow Manipulation, Vitakinesis",
//     "loserPicture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/jarad-vod-savo--2332551804562092846.jpg",
//     "winnerTeam": "villains",
//     "loserTeam": "heroes",
//     "location": {
//         "name": "Tyr",
//         "description": "Located in the Tablelands, Tyr is one of the city-states in the harsh world of Athas. Tyr is known for its oppressive sorcerer-king, Kalak, who rules the city with an iron fist.",
//         "picture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/tyr.jpg"
//     }
// }
