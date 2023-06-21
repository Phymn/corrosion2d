use winit::event_loop::EventLoop;

pub(crate) fn event_builder() -> EventLoop<()> {
    let event_loop = EventLoop::new();

    event_loop
}
