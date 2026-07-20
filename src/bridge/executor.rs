use crate::core::action::ActionKind;
use crate::core::id::EntityId;

pub trait ActionExecutorBridge {
    fn execute(&mut self, entity: EntityId, action: ActionKind);
}
