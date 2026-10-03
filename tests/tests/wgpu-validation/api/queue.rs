//! Tests of [`wgpu::Queue`].

use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

#[test]
fn as_hal_exclusive() {
    let (_device, queue) = wgpu::Device::noop(&wgpu::DeviceDescriptor::default());

    let found = unsafe {
        queue.as_hal_exclusive::<wgpu_hal::api::Noop, _, _>(|hal_queue| hal_queue.is_some())
    };
    assert!(found);
}

#[test]
fn as_hal_exclusive_blocks_submit() {
    let (_device, queue) = wgpu::Device::noop(&wgpu::DeviceDescriptor::default());
    let submitted = AtomicBool::new(false);

    std::thread::scope(|scope| unsafe {
        queue.as_hal_exclusive::<wgpu_hal::api::Noop, _, _>(|_| {
            scope.spawn(|| {
                queue.submit([]);
                submitted.store(true, Ordering::SeqCst);
            });
            std::thread::sleep(Duration::from_millis(100));
            assert!(!submitted.load(Ordering::SeqCst));
        });
    });

    assert!(submitted.load(Ordering::SeqCst));
}
