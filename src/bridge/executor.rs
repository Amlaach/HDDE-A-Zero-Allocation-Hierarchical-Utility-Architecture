use crate::core::id::EntityId;
use crate::core::action::ActionKind;

pub trait ActionExecutorBridge {
    fn execute(&mut self, entity: EntityId, action: ActionKind);
}
