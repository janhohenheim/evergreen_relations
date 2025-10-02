use std::{any::type_name, marker::PhantomData};

use crate::{container::EntityContainer, message::RelationMessage, relation::Relatable};
use bevy_ecs::{
    component::{Immutable, StorageType},
    lifecycle::{ComponentHook, HookContext},
    prelude::*,
    world::DeferredWorld,
};
use std::fmt::Debug;

/// [`Component`] used to store [`Relation`] data for a given side of a relationship,
/// i.e. the [`Relatable`].
///
/// [`Relation`]: crate::relation::Relation
pub struct Related<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> {
    pub(crate) container: N::Container,
}
impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> Component
    for Related<T, N>
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    /*fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(associate::<N>);
        hooks.on_replace(disassociate::<N>);
        hooks.on_remove(disassociate::<N>);
    }*/

    fn on_insert() -> Option<ComponentHook> {
        Some(associate::<T, N>)
    }

    fn on_replace() -> Option<ComponentHook> {
        Some(disassociate::<T, N>)
    }

    fn on_remove() -> Option<ComponentHook> {
        Some(disassociate::<T, N>)
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> Related<T, N> {
    pub fn new(node: impl Into<N::Container>) -> Self {
        Self {
            container: node.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> + '_ {
        self.container.iter()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.container.contains(entity)
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> Clone
    for Related<T, N>
{
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
        }
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> PartialEq
    for Related<T, N>
{
    fn eq(&self, other: &Self) -> bool {
        self.container == other.container
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> Eq
    for Related<T, N>
{
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> std::fmt::Debug
    for Related<T, N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Related")
            .field(&type_name::<N>())
            .field(&self.container)
            .finish()
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>> From<(Entity, T)>
    for Related<T, N>
{
    fn from(entity: (Entity, T)) -> Self {
        Self {
            container: N::Container::new(entity.0, entity.1),
        }
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>>
    FromIterator<(Entity, T)> for Related<T, N>
where
    N::Container: FromIterator<(Entity, T)>,
{
    fn from_iter<A: IntoIterator<Item = (Entity, T)>>(iter: A) -> Self {
        Self {
            container: N::Container::from_iter(iter),
        }
    }
}

fn associate<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>>(
    mut world: DeferredWorld,
    HookContext { entity: a_id, .. }: HookContext,
) {
    world.commands().queue(move |world: &mut World| {
        // Get the IDs of the other entities that this entity is related to.
        let Some(a_related) = world.get::<Related<T, N>>(a_id).cloned() else {
            return;
        };

        // For each other related entity, associate them with this entity.
        for (b_id, data) in a_related.iter() {
            let Ok(mut b) = world.get_entity_mut(b_id) else {
                return;
            };

            let b_related = b.get::<Related<T, N::Opposite>>();

            let b_points_to_a = b_related.is_some_and(|b| b.contains(a_id));
            if !b_points_to_a {
                if let Some(b_related) = b_related {
                    // The other entity is already related to some entities, so add this entity to the list.
                    let mut b_related = b_related.clone();
                    b_related.container.push(a_id, data.clone());
                    b.insert(b_related);
                } else {
                    // The other entity is not yet related to any entities, so relate it to this entity.
                    let b_related = Related::<T, N::Opposite>::from((a_id, data.clone()));
                    b.insert(b_related);
                }

                if let Some(mut messages) =
                    world.get_resource_mut::<Messages<RelationMessage<T, N::Relation>>>()
                {
                    messages.write(RelationMessage::Added(a_id, b_id, PhantomData, PhantomData));
                }
            }
        }
    });
}

fn disassociate<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, N: Relatable<T>>(
    mut world: DeferredWorld,
    HookContext { entity: a_id, .. }: HookContext,
) {
    // Gets the IDs of the entities that this entity is no longer related to.
    let Some(b_ids) = world.get::<Related<T, N>>(a_id).cloned() else {
        return;
    };

    world.commands().queue(move |world: &mut World| {
        // For each related entity, disassociate it from this entity.
        for (b_id, data) in b_ids.iter() {
            let a_points_to_b = world
                .get::<Related<T, N>>(a_id)
                .is_some_and(|a_related| a_related.contains(b_id));

            let Ok(mut b) = world.get_entity_mut(b_id) else {
                return;
            };

            let b_related = b.get::<Related<T, N::Opposite>>();

            let b_points_to_a = b_related.is_some_and(|b| b.contains(a_id));
            if b_points_to_a && !a_points_to_b {
                if let Some(b_related) = b_related {
                    // The other entity is related to some entities, so make sure this entity is removed from the list.
                    let mut b_related = b_related.clone();
                    b_related.container.remove(a_id);

                    // If the other entity is no longer related to any entities, remove the component.
                    if b_related.container.is_empty() {
                        b.remove::<Related<T, N::Opposite>>();
                    } else {
                        b.insert(b_related);
                    }
                }

                if let Some(mut messages) =
                    world.get_resource_mut::<Messages<RelationMessage<T, N::Relation>>>()
                {
                    messages.write(RelationMessage::Removed(
                        a_id,
                        b_id,
                        PhantomData,
                        PhantomData,
                    ));
                }
            }
        }
    });
}
