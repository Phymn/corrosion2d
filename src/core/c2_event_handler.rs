use winit::event_loop::EventLoop;

pub(crate) fn event_builder() -> EventLoop<()> {
    EventLoop::new()
}
