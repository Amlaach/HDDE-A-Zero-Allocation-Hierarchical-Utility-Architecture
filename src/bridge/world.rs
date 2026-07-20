use crate::core::id::EntityId;
use crate::core::math::Vec3;

pub trait Perceivable {
    fn position(&self) -> Vec3;
    fn visibility_profile(&self) -> f32;
}

pub trait Communicator {
    fn can_receive(&self, from: EntityId) -> bool;
}

pub trait WorldBridge {
    fn get_perceivable(&self, id: EntityId) -> Option<&dyn Perceivable>;
    fn get_communicator(&self, id: EntityId) -> Option<&dyn Communicator>;
}
