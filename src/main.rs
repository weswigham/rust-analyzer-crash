use std::marker::PhantomData;
use bevy::{
    ecs::system::{BoxedSystem, Command},
    prelude::*,
};

mod other;

pub type SystemFactory<T> = dyn Fn() -> BoxedSystem<T, ()> + Send + Sync + 'static;

pub type OptionalEventHandler<T> = Option<Box<SystemFactory<T>>>;

pub struct ExecuteOnChange<
    T: Send + Sync + 'static,
    U: GetOnChangeSystem<In = T> + Send + Sync + 'static,
> {
    pub event: T,
    entity: Entity,
    _phantom_component: PhantomData<U>,
}

impl<T, U> ExecuteOnChange<T, U>
where
    T: OnChangeEvent<Component = U> + Send + Sync + 'static,
    U: GetOnChangeSystem<In = T> + Send + Sync + 'static,
{
    pub fn new(entity: Entity, event: T) -> ExecuteOnChange<T, U> {
        ExecuteOnChange {
            event,
            entity,
            _phantom_component: PhantomData::<U>,
        }
    }
}

impl<T: Send + Sync + 'static, U: GetOnChangeSystem<In = T> + Send + Sync + 'static> Command
    for ExecuteOnChange<T, U>
{
    fn write(self, world: &mut World) {
        let res = world.query::<&U>().get(world, self.entity).unwrap();
        let sys = res.get_on_change_system();
        match sys {
            None => (),
            Some(mut sys) => {
                sys.initialize(world);
                sys.run(self.event, world);
                sys.apply_buffers(world);
            }
        }
    }
}

pub trait GetOnChangeSystem: Component {
    type In: OnChangeEvent<Component = Self>;
    fn get_on_change_system(&self) -> Option<BoxedSystem<Self::In, ()>>;
}

pub trait OnChangeEvent {
    type Component: GetOnChangeSystem<In = Self>;
}

fn main() {
    let a = "";
}
