use core::fmt;
use std::marker::PhantomData;

use bevy_ecs::prelude::*;
use std::fmt::Debug;

use crate::relation::Relation;

/// A [`Message`] that is emitted when a [`Relation`] is added or removed between
/// two entities.
#[derive(Message)]
pub enum RelationMessage<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R: Relation<T>>
{
    Added(Entity, Entity, PhantomData<fn(R)>, PhantomData<fn(T)>),
    Removed(Entity, Entity, PhantomData<fn(R)>, PhantomData<fn(T)>),
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R: Relation<T>> fmt::Debug
    for RelationMessage<T, R>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Added(arg0, arg1, ..) => f.debug_tuple("Added").field(arg0).field(arg1).finish(),
            Self::Removed(arg0, arg1, ..) => {
                f.debug_tuple("Removed").field(arg0).field(arg1).finish()
            }
        }
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R: Relation<T>> PartialEq
    for RelationMessage<T, R>
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Added(l0, l1, ..), Self::Added(r0, r1, ..))
            | (Self::Removed(l0, l1, ..), Self::Removed(r0, r1, ..)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R: Relation<T>> Eq
    for RelationMessage<T, R>
{
}

impl<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R: Relation<T>> Clone
    for RelationMessage<T, R>
{
    fn clone(&self) -> Self {
        match self {
            Self::Added(arg0, arg1, ..) => Self::Added(*arg0, *arg1, PhantomData, PhantomData),
            Self::Removed(arg0, arg1, ..) => Self::Removed(*arg0, *arg1, PhantomData, PhantomData),
        }
    }
}
