use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window as WinitWindow;

pub struct WindowState {
    pub handle: Arc<WinitWindow>,
    pub surface: Arc<wgpu::Surface<'static>>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub config: wgpu::SurfaceConfiguration,
}

impl WindowState {
    pub fn build(title: &str, event_loop: &ActiveEventLoop) -> Self {
        let attributes = WinitWindow::default_attributes()
            .with_title(title)
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let handle = Arc::new(event_loop.create_window(attributes).unwrap());

        let instance = wgpu::Instance::default();
        let surface = Arc::new(instance.create_surface(handle.clone()).unwrap());

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::DEPTH_CLIP_CONTROL
                    | wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                    | wgpu::Features::PUSH_CONSTANTS,
                required_limits: wgpu::Limits {
                    max_push_constant_size: 128,
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let size = handle.inner_size();
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        return Self {
            handle,
            surface,
            device,
            queue,
            config,
        };
    }
}
