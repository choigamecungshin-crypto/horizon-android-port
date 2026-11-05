//! Android-only winit integration.
use std::cell::RefCell;
use winit::{
    error::EventLoopError,
    event_loop::EventLoop,
    platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp},
};

thread_local! {
    static ANDROID_APP: RefCell<Option<AndroidApp>> = const { RefCell::new(None) };
}

/// Installs the handle for the current android_main invocation.
pub fn install_android_app(app: AndroidApp) {
    ANDROID_APP.with(|slot| *slot.borrow_mut() = Some(app));
}

/// Builds winit's event loop with the Activity associated with this invocation.
pub(crate) fn event_loop() -> Result<EventLoop<()>, EventLoopError> {
    let app = ANDROID_APP.with(|slot| {
        slot.borrow().as_ref()
            .expect("android_main must install AndroidApp before starting Horizon")
            .clone()
    });
    let mut builder = EventLoop::builder();
    builder.with_android_app(app);
    builder.build()
}
