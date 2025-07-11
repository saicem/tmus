use std::sync::OnceLock;
use tokio::runtime::{Handle, Runtime};

static RUNTIME: OnceLock<GlobalRuntime> = OnceLock::new();

#[derive(Debug)]
struct GlobalRuntime {
    _runtime: Option<Runtime>,
    handle: Handle,
}

fn default_runtime() -> GlobalRuntime {
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle().clone();
    GlobalRuntime {
        _runtime: Some(runtime),
        handle,
    }
}

pub fn handle<'a>() -> &'a Handle {
    &RUNTIME.get_or_init(default_runtime).handle
}

pub fn set_runtime(handle: Handle) {
    RUNTIME
        .set(GlobalRuntime {
            _runtime: None,
            handle,
        })
        .expect("runtime already initialized");
}
