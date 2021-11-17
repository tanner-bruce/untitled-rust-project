use bevy::{
    asset::HandleId,
    gltf::Gltf,
    prelude::{AssetServer, Res, ResMut, Scene, Handle},
    utils::HashMap
};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_yaml::Deserializer;
use std::{
    borrow::Borrow,
    fs::File
};

use lazy_static::*;
use bevy::utils::StableHashMap;

// #[macro_use]
// extern crate lazy_static;


lazy_static! {
    pub static ref Buildings: StableHashMap<String, BuildingData> = load_manifest_data::<BuildingData>();
    pub static ref Entities: StableHashMap<String, LivingData> = load_manifest_data::<LivingData>();
    pub static ref Races: StableHashMap<String, RaceData> = load_manifest_data::<RaceData>();
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RaceData{
    pub name: String,
    pub description: String,
    pub properties: Option<Vec<String>>
}

impl ManifestData for RaceData {
    fn kind() -> ManifestType {
        ManifestType::RaceData
    }

    fn id(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LivingData{
    pub name: String,
    pub description: String,
    pub race: String,
    pub path: String,
    pub id: String,
    // pub tags: Vec<String>,
    // pub species: String
}

impl LivingData {
    pub fn scene(&self) -> String {
        self.path.clone() + "#Scene0"
    }
}

impl ManifestData for LivingData {
    fn kind() -> ManifestType {
        ManifestType::LivingData
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

impl HasAsset for LivingData {
    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Clone)]
pub struct EntityData {
    pub data: StableHashMap<&'static str, HandleId>,
}

impl EntityData {
    pub fn new(asset_server: &Res<AssetServer>) -> Self{
        let mut d: StableHashMap<&'static str, HandleId> = Default::default();
        Buildings.iter().for_each(|(s, mut v)| {
            let h = asset_server.load::<Gltf, &str>((v.path()).as_str());
            d.insert(v.id.as_str(), h.id);
        });
        Entities.iter().for_each(|(s, v)| {
            let h = asset_server.load::<Gltf, &str>((v.path()).as_str());
            d.insert(v.id.as_str(), h.id);
        });
        Self{
            data: d,
        }
    }
}

fn load_manifest_data<'a, T>() -> StableHashMap<String, T>
    where
        T: ManifestData + Default + Clone + Serialize + DeserializeOwned + 'static {
    let mf = "data/manifest.yml";
    let f = std::fs::File::open(mf);
    if let Err(e) = f {
        panic!("bad manifest file: '{}'", e);
    }
    let ds = serde_yaml::from_reader::<File, ManifestFile<ManifestEntry>>(f.unwrap());

    if let Err(e) = ds {
        panic!("bad manifest file contents: '{}'", e);
    }
    let mut bdm: StableHashMap<String, T> = Default::default();
    ds.unwrap().entries.iter().for_each(|f| {
        if f.kind == T::kind() {
            let path = f.path.clone();
            let file = std::fs::File::open(path.as_str());
            if let Err(e) = file {
                panic!("bad manifest file: '{}': '{}'", path, e);
            }
            let ds = serde_yaml::from_reader::<File, ManifestFile::<T>>(file.unwrap());
            if let Err(e) = ds {
                panic!("bad manifest file contents: '{}': '{}'", path, e);
            }
            let fs = ds.unwrap();
            fs.entries.iter().for_each(|bd| {
                bdm.insert(bd.id(), bd.clone());
            })
        }
    });
    bdm
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ManifestType {
    ManifestData,
    BuildingData,
    LivingData,
    RaceData,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    path: String,
    kind: ManifestType,
}

impl ManifestData for ManifestEntry {
    fn kind() -> ManifestType {
        ManifestType::ManifestData
    }

    fn id(&self) -> String {
        self.path.clone()
    }
}

impl Default for ManifestEntry {
    fn default() -> Self {
        Self{
            path: "".to_string(),
            kind: ManifestType::BuildingData,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Default + Clone + Serialize + for<'a> Deserialize<'a>")]
pub struct ManifestFile<T: Default + Clone + Serialize + for<'a> Deserialize<'a>> {
    entries: Vec<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Properties {
    Usable,
    Toggleable,
    Powered
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub level: i32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub name: String,
    pub value: i32,
}

pub trait HasAsset {
    fn path(&self) -> String;
    // fn kind(self) -> AssetType
}

pub trait ManifestData {
    fn kind() -> ManifestType;
    fn id(&self) -> String;
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BuildingData {
    pub name: String,
    pub id: String,
    pub path: String,
    pub base_hp: i32,
    pub base_time: i32,
    pub power_used: Option<i32>,
    pub cost: Option<Vec<Cost>>,
    pub handle: Option<HandleId>,

    // the base capabilities this building has
    pub caps: Option<Vec<Capability>>,
    // environmental effects
    pub effects: Option<Vec<Effects>>,
    // what can be done with the building
    pub properties: Option<Vec<Properties>>,

    pub tags: Option<Vec<String>>
}

impl ManifestData for BuildingData {
    fn kind() -> ManifestType {
        ManifestType::BuildingData
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
impl HasAsset for BuildingData {
    fn path(&self) -> String {
        self.path.clone()
    }
}


#[derive(Default, Clone, Serialize, Deserialize)]
pub struct MaterialData {
    pub name: String,
    pub id: String,
    pub description: String,
    pub hp_mod: f64,
    pub weight_mod: f64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub name: String,
    pub value: i32,
}
