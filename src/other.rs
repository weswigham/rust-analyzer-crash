use bevy::{ecs::{component::Component, system::BoxedSystem}, prelude::*};

#[derive(Component)]
pub struct Slider {
    pub dragging: bool,
    pub snap: bool,
    pub value: f32,
    pub on_change: super::OptionalEventHandler<SliderValueChangedEvent>,
}

#[derive(Clone, Copy, Debug)]
pub struct SliderValueChangedEvent {
    pub entity: Entity,
    pub value: f32,
}

fn on_value_change(
    mut commands: Commands,
    mut ev_value_changed: EventReader<SliderValueChangedEvent>,
) {
    for ev in ev_value_changed.iter() {
        commands.add(super::ExecuteOnChange::new(ev.entity, *ev));
    }
}

impl super::OnChangeEvent for SliderValueChangedEvent {
    type Component = Slider;
}

impl super::GetOnChangeSystem for Slider {
    type In = SliderValueChangedEvent;
    fn get_on_change_system(&self) -> Option<BoxedSystem<Self::In, ()>> {
        match &self.on_change {
            None => None,
            Some(f) => Some(f()),
        }
    }
}