use std::any::type_name;

use common::{
    eyre::{bail, Result},
    serde::{de::DeserializeOwned, Serialize},
    serde_json, tracing,
};
use node_address::Address;

use crate::{
    differ::Differ,
    operation::{Add, Copy, Move, Operation, Remove, Replace, Transform},
    prelude::{invalid_patch_operation, Patch},
    value::Value,
};

pub trait Patchable: Serialize + DeserializeOwned {
    /// Generate the operations needed to mutate this node so that it is equal
    /// to a node of the same type.
    fn diff(&self, other: &Self, differ: &mut Differ);

    /// Apply a patch to this node.
    fn apply_patch(&mut self, patch: Patch) -> Result<()> {
        tracing::trace!("Applying patch to type `{}`", type_name::<Self>());
        for op in patch.ops {
            self.apply_op(op)?
        }
        Ok(())
    }

    /// Apply an operation to this node.
    fn apply_op(&mut self, op: Operation) -> Result<()> {
        match op {
            Operation::Add(Add {
                mut address, value, ..
            }) => self.apply_add(&mut address, value),
            Operation::Remove(Remove { mut address, items }) => {
                self.apply_remove(&mut address, items)
            }
            Operation::Replace(Replace {
                mut address,
                items,
                value,
                ..
            }) => self.apply_replace(&mut address, items, value),
            Operation::Move(Move {
                mut from,
                items,
                mut to,
            }) => self.apply_move(&mut from, items, &mut to),
            Operation::Copy(Copy {
                mut from,
                items,
                mut to,
            }) => self.apply_copy(&mut from, items, &mut to),
            Operation::Transform(Transform {
                mut address,
                from,
                to,
            }) => self.apply_transform(&mut address, &from, &to),
        }
    }

    /// Apply an `Add` patch operation
    fn apply_add(&mut self, _address: &mut Address, _value: Value) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Add"))
    }

    /// Apply a `Remove` patch operation
    fn apply_remove(&mut self, _address: &mut Address, _items: usize) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Remove"))
    }

    /// Apply a `Replace` patch operation
    fn apply_replace(
        &mut self,
        _address: &mut Address,
        _items: usize,
        _value: Value,
    ) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Replace"))
    }

    /// Apply a `Move` patch operation
    fn apply_move(&mut self, _from: &mut Address, _items: usize, _to: &mut Address) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Move"))
    }

    /// Apply a `Copy` patch operation
    fn apply_copy(&mut self, _from: &mut Address, _items: usize, _to: &mut Address) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Copy"))
    }

    /// Apply a `Transform` patch operation
    fn apply_transform(&mut self, _address: &mut Address, _from: &str, _to: &str) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("Transform"))
    }

    /// Create a [`Value`] from an instance of the type
    ///
    /// This default implementation uses the `Json` variant.
    /// Implementations of `Patchable` should use specific variants
    /// for the type, if available.
    fn to_value(&self) -> Value {
        Value::Json(serde_json::to_value(self).unwrap_or_default())
    }

    /// Create an instance of the type from a [`Value`]
    ///
    /// This default implementation assumes the use of the `Json` variant.
    /// Implementations should have a `from_value` that matches their `to_value`.
    fn from_value(value: Value) -> Result<Self> {
        if let Value::Json(json) = value {
            Ok(serde_json::from_value::<Self>(json)?)
        } else {
            bail!("Expected a JSON value, got a `{}` value", value.as_ref())
        }
    }
}
