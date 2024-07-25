use crate::constants::ObjectCategory;
use std::collections::{HashMap, HashSet};

static OBJECT_CAT_ELEMS: [ObjectCategory; 9] = [
    ObjectCategory::Player,
    ObjectCategory::Obstacle,
    ObjectCategory::DeathMarker,
    ObjectCategory::Loot,
    ObjectCategory::Building,
    ObjectCategory::Decal,
    ObjectCategory::Parachute,
    ObjectCategory::ThrowableProjectile,
    ObjectCategory::SyncedParticle
];

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct GameObject {
    r#type: ObjectCategory,
    id: u64
}

pub struct ObjectPool {
    objects: HashMap<u64, GameObject>,
    by_category: HashMap<ObjectCategory, HashSet<GameObject>>
}

impl ObjectPool {
    pub fn get_category(&mut self, key: ObjectCategory) -> &mut HashSet<GameObject> {
        self.by_category.get_mut(&key).unwrap()
    }
    pub fn new() -> Self {
        let mut temp: HashMap<ObjectCategory, HashSet<GameObject>> = HashMap::new();
        for cat in &OBJECT_CAT_ELEMS {
            temp.insert(*cat, HashSet::new());
        }

        Self {
            objects: HashMap::new(),
            by_category: temp.clone()
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
        for (_, cat) in self.by_category.iter_mut() {
            cat.clear();
        }
    }
    pub fn add(&mut self, object: GameObject) {
        self.objects.insert(object.id, object);
        self.get_category(object.r#type).insert(object);
    }
    pub fn delete(&mut self, object: GameObject) {
        self.get_category(object.r#type).remove(&object);
        self.objects.remove(&object.id);
    }
    pub fn has(&self, object: GameObject) -> bool {
        self.objects.contains_key(&object.id)
    }
    pub fn category_has(&mut self, object: GameObject) -> bool {
        self.get_category(object.r#type).contains(&object)
    }
    pub fn get(&mut self, id: u64) -> Option<&mut GameObject> {
        self.objects.get_mut(&id)
    }
    pub fn has_id(&self, id: u64) -> bool {
        self.objects.contains_key(&id)
    }
    pub fn get_size(&self) -> usize {
        self.objects.len()
    }
    // FIXME: this is temporary
    pub fn iter(&self) -> std::collections::hash_map::Values<'_, u64, GameObject> {
        self.objects.values()
    }
}
/* TODO: implement this (i couldnt do it)
impl IntoIterator for ObjectPool {
    type Item = GameObject;
    type IntoIter = std::collections::hash_map::Values<'_, u64, GameObject>;
    fn into_iter(&self) -> Self::IntoIter {
        self.objects.values()
    }
}*/
