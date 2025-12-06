//! # Type-Safe Pipeline Builder
//!
//! This module provides a compile-time type-safe builder for constructing pipelines
//! without changing the existing Pipeline API. It uses a function-based approach to
//! ensure handler type compatibility at compile time.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use sansio::{PipelineBuilder, Pipeline};
//!
//! // Build a type-safe pipeline
//! let pipeline: Pipeline<Vec<u8>, String> = PipelineBuilder::new()
//!     .add(BytesToFrameHandler)      // Vec<u8> -> Frame
//!     .add(FrameToStringHandler)     // Frame -> String
//!     .add(StringEchoHandler)        // String -> String
//!     .build();
//! ```

use std::marker::PhantomData;
use std::rc::Rc;

use crate::{handler::Handler, pipeline::Pipeline};

/// Type-safe pipeline builder that tracks the current pipeline state at compile time.
///
/// Uses a functional approach where each handler addition returns a new builder
/// with updated type information.
pub struct PipelineBuilder<R, CurrentOut, W>
where
    R: 'static,
    CurrentOut: 'static,
    W: 'static,
{
    build_fn: Box<dyn FnOnce() -> Pipeline<R, W>>,
    _phantom: PhantomData<CurrentOut>,
}
impl<R, W> Default for PipelineBuilder<R, R, W>
where
    R: 'static,
    W: 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R, W> PipelineBuilder<R, R, W>
where
    R: 'static,
    W: 'static,
{
    /// Creates a new type-safe pipeline builder.
    pub fn new() -> Self {
        Self {
            build_fn: Box::new(|| Pipeline::new()),
            _phantom: PhantomData,
        }
    }
}

impl<R, CurrentOut, W> PipelineBuilder<R, CurrentOut, W>
where
    R: 'static,
    CurrentOut: 'static,
    W: 'static,
{
    #[allow(clippy::should_implement_trait)]
    /// Adds a handler to the pipeline, ensuring type compatibility at compile time.
    ///
    /// The handler's input type (`H::Rin`) must match the current pipeline output type.
    pub fn add<H>(self, handler: H) -> PipelineBuilder<R, H::Rout, W>
    where
        H: Handler<Rin = CurrentOut> + 'static,
    {
        let prev_build_fn = self.build_fn;

        PipelineBuilder {
            build_fn: Box::new(move || {
                let pipeline = prev_build_fn();
                pipeline.add_back(handler);
                pipeline
            }),
            _phantom: PhantomData,
        }
    }

    /// Builds the final pipeline.
    pub fn build(self) -> Rc<Pipeline<R, W>> {
        let pipeline = (self.build_fn)();
        pipeline.finalize()
    }
}
