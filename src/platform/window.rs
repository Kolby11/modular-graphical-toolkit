use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_output, delegate_registry, delegate_seat,
    delegate_shm, delegate_xdg_shell, delegate_xdg_window,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{Capability, SeatHandler, SeatState},
    shell::{
        WaylandSurface,
        xdg::{
            window::{Window, WindowConfigure, WindowDecorations, WindowHandler},
            XdgShell,
        },
    },
    shm::{slot::SlotPool, Shm, ShmHandler},
};
use wayland_client::{
    globals::registry_queue_init,
    protocol::{wl_output, wl_seat, wl_shm, wl_surface},
    Connection, QueueHandle,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct WaylandPlatform {
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,
    compositor_state: CompositorState,
    shm: Shm,
    xdg_shell: XdgShell,

    pool: SlotPool,
    window: Window,

    width: u32,
    height: u32,
    should_exit: bool,
    needs_redraw: bool,
}

impl WaylandPlatform {
    pub fn new() -> anyhow::Result<(Self, wayland_client::EventQueue<Self>)> {
        let conn = Connection::connect_to_env()?;
        let (globals, event_queue) = registry_queue_init::<Self>(&conn)?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)?;
        let xdg_shell = XdgShell::bind(&globals, &qh)?;
        let shm = Shm::bind(&globals, &qh)?;
        let seat_state = SeatState::new(&globals, &qh);
        let output_state = OutputState::new(&globals, &qh);

        let surface = compositor_state.create_surface(&qh);
        let window = xdg_shell.create_window(surface, WindowDecorations::ServerDefault, &qh);
        window.set_title("MGT Window");
        window.set_app_id("mgt.app");
        window.set_min_size(Some((320, 240)));
        window.commit();

        let pool = SlotPool::new(WIDTH as usize * HEIGHT as usize * 4, &shm)?;

        Ok((
            Self {
                registry_state: RegistryState::new(&globals),
                seat_state,
                output_state,
                compositor_state,
                shm,
                xdg_shell,
                pool,
                window,
                width: WIDTH,
                height: HEIGHT,
                should_exit: false,
                needs_redraw: true,
            },
            event_queue,
        ))
    }

    pub fn run(&mut self, event_queue: &mut wayland_client::EventQueue<Self>) {
        while !self.should_exit {
            event_queue.blocking_dispatch(self).expect("Wayland dispatch failed");

            if self.needs_redraw {
                self.draw();
                self.needs_redraw = false;
            }
        }
    }

    fn draw(&mut self) {
        let stride = self.width as i32 * 4;
        let (buffer, canvas) = self.pool
            .create_buffer(
                self.width as i32,
                self.height as i32,
                stride,
                wl_shm::Format::Argb8888,
            )
            .expect("Failed to create buffer");

        // Fill background: dark grey
        canvas.chunks_exact_mut(4).for_each(|px| {
            px[0] = 40;  // B
            px[1] = 40;  // G
            px[2] = 40;  // R
            px[3] = 255; // A
        });

        // Draw a simple button rect (blue, 100x40 at position 50,50)
        fill_rect(canvas, self.width, self.height, 50, 50, 150, 40, [60, 120, 220, 255]);

        let surface = self.window.wl_surface();
        surface.attach(Some(buffer.wl_buffer()), 0, 0);
        surface.damage_buffer(0, 0, self.width as i32, self.height as i32);
        surface.commit();
    }

}

/// Fill a rectangle into the ARGB8888 canvas.
fn fill_rect(canvas: &mut [u8], surface_w: u32, surface_h: u32,
             x: u32, y: u32, w: u32, h: u32, color: [u8; 4]) {
    for row in y..(y + h).min(surface_h) {
        for col in x..(x + w).min(surface_w) {
            let offset = ((row * surface_w + col) * 4) as usize;
            if offset + 3 < canvas.len() {
                canvas[offset]     = color[0]; // B
                canvas[offset + 1] = color[1]; // G
                canvas[offset + 2] = color[2]; // R
                canvas[offset + 3] = color[3]; // A
            }
        }
    }
}

// --- Trait implementations required by smithay-client-toolkit ---

impl CompositorHandler for WaylandPlatform {
    fn scale_factor_changed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface, _new_factor: i32) {}

    fn transform_changed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface, _new_transform: wl_output::Transform) {}

    fn frame(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface, _time: u32) {
        self.needs_redraw = true;
    }

    fn surface_enter(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface, _output: &wl_output::WlOutput) {}

    fn surface_leave(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface, _output: &wl_output::WlOutput) {}
}

impl OutputHandler for WaylandPlatform {
    fn output_state(&mut self) -> &mut OutputState { &mut self.output_state }
    fn new_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: wl_output::WlOutput) {}
    fn update_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: wl_output::WlOutput) {}
    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: wl_output::WlOutput) {}
}

impl WindowHandler for WaylandPlatform {
    fn request_close(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _window: &Window) {
        self.should_exit = true;
    }

    fn configure(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _window: &Window, configure: WindowConfigure, _serial: u32) {
        let (new_w, new_h) = configure.new_size;
        if let (Some(w), Some(h)) = (new_w, new_h) {
            self.width = w.get();
            self.height = h.get();
            let bytes = self.width as usize * self.height as usize * 4;
            self.pool.resize(bytes).expect("Failed to resize pool");
        }
        self.needs_redraw = true;
    }
}

impl SeatHandler for WaylandPlatform {
    fn seat_state(&mut self) -> &mut SeatState { &mut self.seat_state }
    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {}
    fn new_capability(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat, _capability: Capability) {}
    fn remove_capability(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>,
        _seat: wl_seat::WlSeat, _capability: Capability) {}
    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: wl_seat::WlSeat) {}
}

impl ShmHandler for WaylandPlatform {
    fn shm_state(&mut self) -> &mut Shm { &mut self.shm }
}

impl ProvidesRegistryState for WaylandPlatform {
    fn registry(&mut self) -> &mut RegistryState { &mut self.registry_state }
    registry_handlers![OutputState, SeatState];
}

delegate_compositor!(WaylandPlatform);
delegate_output!(WaylandPlatform);
delegate_registry!(WaylandPlatform);
delegate_seat!(WaylandPlatform);
delegate_shm!(WaylandPlatform);
delegate_xdg_shell!(WaylandPlatform);
delegate_xdg_window!(WaylandPlatform);
