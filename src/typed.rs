//! Typed pipeline builder prototype
//!
//! This module provides a typed builder that enforces compile-time
//! adjacency between handlers in a pipeline:
//! - Inbound: Prev::Rout == Next::Rin
//! - Outbound: Next::Wout == Prev::Win
//!
//! It builds on top of the existing dynamic `Pipeline`, so runtime
//! behavior and performance remain the same while offering compile-time
//! safety during composition.

use crate::{Handler, Pipeline};
use std::{marker::PhantomData, rc::Rc};

/// Projects the associated types of a `Handler` without requiring method impls.
pub trait HandlerSig {
    /// Associated read input message type
    type Rin: 'static;
    /// Associated read output message type
    type Rout: 'static;
    /// Associated write input message type
    type Win: 'static;
    /// Associated write output message type for
    type Wout: 'static;
}

impl<T> HandlerSig for T
where
    T: Handler + ?Sized,
{
    type Rin = T::Rin;
    type Rout = T::Rout;
    type Win = T::Win;
    type Wout = T::Wout;
}

/// Start marker that encodes the initial pipeline boundary types `R` and `W`.
/// It behaves as a zero-sized type carrying only type information.
pub struct Start<R: 'static, W: 'static>(PhantomData<(R, W)>);

impl<R: 'static, W: 'static> HandlerSig for Start<R, W> {
    type Rin = R;
    type Rout = R;
    type Win = W;
    type Wout = W;
}

/// Type equality assertion helper.
/// Usage: add a bound `(): AssertEqual<A, B>` to force `A == B`.
pub trait AssertEqual<A: 'static, B: 'static> {}
impl<T: 'static> AssertEqual<T, T> for () {}

/// A typed builder that composes handlers with compile-time adjacency checks
/// while building a regular dynamic `Pipeline` underneath.
#[derive(Default)]
pub struct TypedPipelineBuilder<R: 'static, W: 'static, Prev: HandlerSig> {
    inner: Pipeline<R, W>,
    _prev: PhantomData<Prev>,
}

impl<R: 'static, W: 'static> TypedPipelineBuilder<R, W, Start<R, W>> {
    /// Creates a new typed pipeline builder for transport types `R` and `W`.
    pub fn new() -> Self {
        Self {
            inner: Pipeline::new(),
            _prev: PhantomData,
        }
    }
}

impl<R: 'static, W: 'static, Prev: HandlerSig> TypedPipelineBuilder<R, W, Prev> {
    /// Adds a handler to the back of the pipeline, enforcing that:
    /// - Inbound types align: `Prev::Rout == H::Rin`
    /// - Outbound types align: `H::Wout == Prev::Win`
    pub fn add_back<H>(self, handler: H) -> TypedPipelineBuilder<R, W, H>
    where
        H: Handler + 'static,
        (): AssertEqual<<Prev as HandlerSig>::Rout, <H as HandlerSig>::Rin>,
        (): AssertEqual<<H as HandlerSig>::Wout, <Prev as HandlerSig>::Win>,
    {
        self.inner.add_back(handler);
        TypedPipelineBuilder {
            inner: self.inner,
            _prev: PhantomData::<H>,
        }
    }

    /// Finalizes and returns the underlying `Rc<Pipeline<R, W>>`.
    pub fn build(self) -> Rc<Pipeline<R, W>> {
        self.inner.finalize()
    }

    /// Returns a mutable reference to the underlying untyped pipeline.
    /// Useful for advanced scenarios before `build()`.
    pub fn as_untyped_mut(&mut self) -> &mut Pipeline<R, W> {
        &mut self.inner
    }
}
