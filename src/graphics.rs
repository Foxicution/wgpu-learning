use crate::vertex::{Vertex, INDICES, VERTICES};
use bytemuck::cast_slice;
use std::borrow::Cow;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt, RenderEncoder},
    Adapter, Buffer, BufferUsages, Color, CommandEncoderDescriptor, Device, DeviceDescriptor,
    Features, FragmentState, IndexFormat, Instance, Limits, LoadOp, MemoryHints, Operations,
    PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, StoreOp,
    Surface, SurfaceConfiguration, TextureFormat, TextureViewDescriptor, VertexState,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

pub async fn create_graphics(window: Rc<Window>, proxy: EventLoopProxy<Graphics>) {
    let instance = Instance::default();
    let surface = instance.create_surface(Rc::clone(&window)).unwrap();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(), // Power preference for the device
            force_fallback_adapter: false, // Indicates that only a fallback ("software") adapter can be used
            compatible_surface: Some(&surface), // Guarantee that the adapter can render to this surface
        })
        .await
        .expect("Could not get an adapter (GPU).");

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(), // Specifies the required features by the device request. Fails if the adapter can't provide them.
                // WebGL doesn't support all of wgpu features, disabling some
                #[cfg(target_arch = "wasm32")]
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                #[cfg(not(target_arch = "wasm32"))]
                required_limits: Limits::default().using_resolution(adapter.limits()),
                memory_hints: MemoryHints::Performance,
            },
            None,
        )
        .await
        .expect("Failed to get device");

    // Get physical pixel dimensiosn inside the window
    let size = window.inner_size();
    // Make the dimensions at least size 1, otherwise wgpu would panic
    let width = size.width.max(1);
    let height = size.height.max(1);
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    surface.configure(&device, &surface_config);

    let render_pipeline = create_pipeline(&device, surface_config.format);

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: cast_slice(VERTICES),
        usage: BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: cast_slice(INDICES),
        usage: BufferUsages::INDEX,
    });

    let gfx = Graphics {
        window: window.clone(),
        instance,
        surface,
        surface_config,
        adapter,
        device,
        queue,
        render_pipeline,
        vertex_buffer,
        index_buffer,
        color: Color::GREEN,
    };

    let _ = proxy.send_event(gfx);
}

fn create_pipeline(device: &Device, swap_chain_format: TextureFormat) -> RenderPipeline {
    // could use wgpu::include_wgsl! macro here
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(swap_chain_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview: None,
        cache: None,
    })
}

#[derive(Debug)]
pub struct Graphics {
    pub window: Rc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub color: Color,
}

impl Graphics {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to aquire next swap chain texture.");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            r_pass.set_pipeline(&self.render_pipeline);
            r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            r_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            r_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        } // `r_pass` dropped here

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
