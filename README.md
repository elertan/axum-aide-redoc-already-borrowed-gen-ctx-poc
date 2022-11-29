# axum-aide-redoc-already-borrowed-gen-ctx-poc

```
 Compiling axum-aide-redoc-already-borrowed-gen-ctx-poc v0.1.0 (/Users/denniskievits/Projects/axum-aide-redoc-already-borrowed-gen-ctx-poc)
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/axum-aide-redoc-already-borrowed-gen-ctx-poc`
thread 'main' panicked at 'already borrowed: BorrowMutError', /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/gen.rs:23:36
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
   3: core::result::Result<T,E>::expect
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1064:23
   4: core::cell::RefCell<T>::borrow_mut
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/cell.rs:956:9
   5: aide::gen::in_context::{{closure}}
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/gen.rs:23:32
   6: std::thread::local::LocalKey<T>::try_with
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/thread/local.rs:445:16
   7: std::thread::local::LocalKey<T>::with
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/thread/local.rs:421:9
   8: aide::gen::in_context
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/gen.rs:23:5
   9: aide::axum::outputs::<impl aide::operation::OperationOutput for axum::json::Json<T>>::inferred_responses
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/axum/outputs.rs:52:13
  10: aide::axum::routing::get::{{closure}}
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/axum/routing.rs:160:36
  11: aide::gen::in_context::{{closure}}
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/gen.rs:23:24
  12: std::thread::local::LocalKey<T>::try_with
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/thread/local.rs:445:16
  13: std::thread::local::LocalKey<T>::with
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/thread/local.rs:421:9
  14: aide::gen::in_context
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/gen.rs:23:5
  15: aide::axum::routing::get
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/aide-0.8.1/src/axum/routing.rs:157:13
  16: axum_aide_redoc_already_borrowed_gen_ctx_poc::make_router
             at ./src/main.rs:32:29
  17: axum_aide_redoc_already_borrowed_gen_ctx_poc::main::{{closure}}
             at ./src/main.rs:10:18
  18: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/future/mod.rs:91:19
  19: tokio::runtime::park::CachedParkThread::block_on::{{closure}}
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/park.rs:272:63
  20: tokio::runtime::coop::with_budget
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/coop.rs:102:5
  21: tokio::runtime::coop::budget
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/coop.rs:68:5
  22: tokio::runtime::park::CachedParkThread::block_on
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/park.rs:272:31
  23: tokio::runtime::context::BlockingRegionGuard::block_on
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/context.rs:255:13
  24: tokio::runtime::scheduler::multi_thread::MultiThread::block_on
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/scheduler/multi_thread/mod.rs:66:9
  25: tokio::runtime::runtime::Runtime::block_on
             at /Users/denniskievits/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.22.0/src/runtime/runtime.rs:281:45
  26: axum_aide_redoc_already_borrowed_gen_ctx_poc::main
             at ./src/main.rs:14:5
  27: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
