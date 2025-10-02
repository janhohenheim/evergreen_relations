use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use bevy_ecs::entity::{Entity, EntityHashSet};
use bevy_platform::collections::HashMap;
use smallvec::{SmallVec, smallvec};

/// A container for the other entities that this entity is related to.
///
/// These containers store the relationship data, and is held inside the
/// [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait EntityContainer<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static>:
    Clone + PartialEq + Eq + Debug + Send + Sync + 'static
{
    /// Creates a new entity container with the initial given entity.
    fn new(entity: Entity, data: T) -> Self;

    /// Returns `true` if this entity is not related to any other entities.
    fn is_empty(&self) -> bool;

    /// Returns `true` if the given entity is related to this entity.
    fn contains(&self, entity: Entity) -> bool;

    /// Returns `T` if the given entity is related to this entity.
    fn get(&self, entity: Entity) -> Option<&T>;

    /// Adds the given entity to the list of entities that this entity is related to.
    fn push(&mut self, entity: Entity, data: T);

    /// Removes the given entity from the list of entities that this entity is related to.
    /// Returns `T` if successfully removed something
    fn remove(&mut self, entity: Entity) -> Option<T>;

    /// Consumes the entity container and returns an iterator over the entities
    /// that this entity is related to.
    fn into_iter(self) -> impl Iterator<Item = (Entity, T)>;

    /// Returns an iterator over the entities that this entity is related to.
    fn iter(&self) -> impl Iterator<Item = (Entity, &T)>;
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static> EntityContainer<T>
    for HashMap<Entity, T>
{
    fn new(entity: Entity, data: T) -> Self {
        let mut map = HashMap::new();
        map.insert(entity, data);
        map
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.contains_key(&entity)
    }

    fn push(&mut self, entity: Entity, data: T) {
        self.insert(entity, data);
    }

    fn remove(&mut self, entity: Entity) -> Option<T> {
        self.remove(&entity)
    }

    fn into_iter(self) -> impl Iterator<Item = (Entity, T)> {
        IntoIterator::into_iter(self)
    }

    fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.iter().map(|(&entity, data)| (entity, data))
    }

    fn get(&self, entity: Entity) -> Option<&T> {
        self.get(&entity)
    }
}
