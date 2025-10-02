use crate::container::EntityContainer;
use std::fmt::Debug;

pub use evergreen_relations_macros::{Relatable, Relation};

/// Trait for types that represent a relationship between entities.
///
/// Entity pointer data is stored in the [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait Relation<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static> {
    /// The "source" node of the relation.
    type Source: Relatable<T, Relation = Self, Opposite = Self::Target>;

    /// The "target" node of the relation.
    type Target: Relatable<T, Relation = Self, Opposite = Self::Source>;
}

/// Trait for types that represent a node in a relationship.
///
/// Entity pointer data is stored in the [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait Relatable<T: Clone + PartialEq + Eq + Debug + Send + Sync + 'static>: 'static {
    /// The relation type that this node is part of.
    type Relation: Relation<T>;

    /// The opposite side of this node's [`Relation`].
    type Opposite: Relatable<T, Relation = Self::Relation, Opposite = Self>;

    /// The container type that holds the related entities.
    type Container: EntityContainer<T>;
}
