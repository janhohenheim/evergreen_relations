use crate::{prelude::Relation, related::Related};
use bevy_ecs::query::{QueryData, ReadOnlyQueryData};
use std::fmt::Debug;

/// [`QueryData`] wrapper for fetching the sides of a [`Relation`].
///
/// The `S` and `T` type parameters are [`Selector`]s that determine whether the
/// `Source` and `Target` sides of the relation are required, optional, or not
/// fetched at all, respectively.
#[derive(QueryData)]
pub struct SelectRelated<
    T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static,
    R: Relation<T>,
    S: Selector,
    A: Selector,
> {
    /// The source side of the relation.
    pub source: S::Item<&'static Related<T, R::Source>>,
    /// The target side of the relation.
    pub target: A::Item<&'static Related<T, R::Target>>,
}

/// A [`SelectRelated`] variant that fetches both sides of the relation as-is,
/// requiring both to be present.
pub type BothRelated<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R> =
    SelectRelated<T, R, Required, Required>;

/// A [`SelectRelated`] variant that fetches both sides of the relation as
/// [`Option`]s, allowing either or both to be absent.
pub type EitherRelated<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static, R> =
    SelectRelated<T, R, Optional, Optional>;

/// A trait providing a type function that determines the type of the item
/// fetched by a [`SelectRelated`] query.
pub trait Selector {
    /// A type function that determines the type of the item fetched by the
    /// selector.
    type Item<D: ReadOnlyQueryData>: ReadOnlyQueryData;
}

/// A [`Selector`] that fetches the item as-is, requiring it to be present.
pub struct Required;

impl Selector for Required {
    type Item<D: ReadOnlyQueryData> = D;
}

/// A [`Selector`] that fetches the item as an [`Option`].
pub struct Optional;

impl Selector for Optional {
    type Item<D: ReadOnlyQueryData> = Option<D>;
}

/// A [`Selector`] that fetches no item at all.
pub struct Nothing;

impl Selector for Nothing {
    type Item<D: ReadOnlyQueryData> = ();
}
