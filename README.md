<h1 align="center">
 <a href="https://sansio.org"><img src="https://raw.githubusercontent.com/sansio-org/sansio/master/doc/sansio-black.png" alt="sansio"></a>
 <br>

## Minimal Example

This example builds a simple pipeline that decodes lines, converts to strings, and echoes them back. It shows the new typed builder prototype that enforces compile-time handler adjacency while still using the same runtime pipeline.

```rust
use bytes::BytesMut;
use sansio::{Handler, Context};
use sansio::{TypedPipelineBuilder, TypedStart};

// A very small handler that passes inbound strings through and appends CRLF on outbound
struct Echo;
impl Handler for Echo {
        type Rin = String;
        type Rout = String;
        type Win = String;
        type Wout = String;
        fn name(&self) -> &str { "Echo" }
        fn handle_read(&mut self, ctx: &Context<Self::Rin, Self::Rout, Self::Win, Self::Wout>, msg: Self::Rin) {
                // Inbound: forward to next handler (or end)
                ctx.fire_handle_read(msg);
        }
        fn poll_write(&mut self, ctx: &Context<Self::Rin, Self::Rout, Self::Win, Self::Wout>) -> Option<Self::Wout> {
                // Outbound: take next message and append CRLF
                ctx.fire_poll_write().map(|mut s| { s.push_str("\r\n"); s })
        }
}

fn build_typed_pipeline() -> std::rc::Rc<sansio::Pipeline<BytesMut, String>> {
        // Transport read type = BytesMut, app write type = String
        // Start marker encodes these boundary types for compile-time checks
        let builder = TypedPipelineBuilder::<BytesMut, String, TypedStart<BytesMut, String>>::new();

        // Replace the following with your own handlers or reuse helpers from examples
        // For illustration only; see full examples under examples/ for real transports
        // use examples::helpers::byte_to_message_decoder::{LineBasedFrameDecoder, TerminatorType, TaggedByteToMessageCodec};
        // use examples::helpers::string_codec::TaggedStringCodec;
        // let builder = builder
        //     .add_back(TaggedByteToMessageCodec::new(Box::new(LineBasedFrameDecoder::new(8192, true, TerminatorType::BOTH))))
        //     .add_back(TaggedStringCodec::new());

        let pipeline = builder
                .add_back(Echo)
                .build();

        pipeline
}
```

Once built, drive the pipeline from your transport loop:

```rust
// Pseudocode inside a UDP/TCP loop
pipeline.transport_active();
pipeline.handle_read(incoming_bytes_mut);
while let Some(out) = pipeline.poll_write() {
        // send out over your socket
}
```

## Troubleshooting

- Type alignment errors when adding a handler:
    - The typed builder enforces that for adjacent handlers A then B: `A::Rout == B::Rin` (inbound) and `B::Wout == A::Win` (outbound).
    - Fix by inserting a converter handler or reordering handlers so inbound/outbound types match.

- Panic: `msg can't downcast::<...> in ... handler`:
    - This comes from the dynamic runtime when types don’t align. Prefer the typed builder to catch these at compile time.

- Forgot to finalize/update the pipeline:
    - The typed builder’s `build()` finalizes automatically. If you’re using the untyped API directly, call `pipeline.finalize()` (or `update()` on an `Rc<Pipeline>`).

- Nothing comes out of `poll_write()`:
    - Ensure an earlier handler is writing outbound messages (e.g., by buffering and returning them from `poll_write`), and that your transport loop is polling regularly.

- Single-threaded by design:
    - Pipelines use `Rc<RefCell<...>>`. To drive multiple connections concurrently in a single thread, see `LocalExecutorBuilder` and `spawn_local` in `local_executor`.

  <img src="https://github.com/sansio-org/sansio/workflows/cargo/badge.svg">
 </a>
 <a href="https://deps.rs/repo/github/sansio-org/sansio">
  <img src="https://deps.rs/repo/github/sansio-org/sansio/status.svg">
 </a>
 <a href="https://crates.io/crates/sansio">
  <img src="https://img.shields.io/crates/v/sansio.svg">
 </a>
 <a href="https://docs.rs/sansio">
  <img src="https://docs.rs/sansio/badge.svg">
 </a>
 <a href="https://doc.rust-lang.org/1.6.0/complement-project-faq.html#why-dual-mitasl2-license">
  <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue" alt="License: MIT/Apache 2.0">
 </a>
</p>
<p align="center">
 Rust in Sans-IO
</p>
