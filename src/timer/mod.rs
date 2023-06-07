use bevy::prelude::*;

// TODO rename
pub fn spawn_timer<T: Event>(
    event: T,
    timer: &mut Timer,
    time: Res<Time>,
    mut event_writer: EventWriter<T>,
) {
    timer.tick(time.delta());
    if timer.finished() {
        event_writer.send(event);
        timer.reset();
    }
}