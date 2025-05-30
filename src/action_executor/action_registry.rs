use crate::action_executor::{ActionEventHandler, ActionExecutor, ActionFn};
use dioxus::signals::Writable;
use serde_json::Value;
use std::rc::Rc;

pub trait ActionRegistry {
    fn register_all_actions(&mut self);
    fn register_visibility_actions(&mut self);
    fn register_content_actions(&mut self);
    fn register_lifecycle_actions(&mut self);
    fn register_state_actions(&mut self);
    fn register_animation_actions(&mut self);
    fn register_navigation_actions(&mut self);
    fn register_data_actions(&mut self);
    fn register_action(&mut self, name: &str, action_fn: ActionFn);
}

impl ActionRegistry for ActionExecutor {
    fn register_all_actions(&mut self) {
        self.register_visibility_actions();
        self.register_content_actions();
        self.register_lifecycle_actions();
        self.register_state_actions();
        self.register_animation_actions();
        self.register_navigation_actions();
        self.register_data_actions();
    }

    fn register_action(&mut self, name: &str, _action_fn: ActionFn) {
        // TODO: Implement proper action registry storage
        // For now, this is a stub to fix compilation
        // The action_fn parameter will be used when we implement actual action storage
        println!("Registering action: {}", name);
    }

    fn register_visibility_actions(&mut self) {
        self.register_action(
            "show",
            Rc::new(|executor, handler| {
                let target = handler.target.as_ref().ok_or("no target specified")?;
                let mut components_signal = executor.ui_state.components;
                let mut components_map = components_signal.write();
                if let Some(component) = components_map.get_mut(target) {
                    component.visible = true;
                    Ok(())
                } else {
                    Err(format!("component '{}' not found", target))
                }
            }),
        );

        self.register_action(
            "hide",
            Rc::new(|executor, handler| {
                let target = handler.target.as_ref().ok_or("no target specified")?;
                let mut components_signal = executor.ui_state.components;
                let mut components_map = components_signal.write();
                if let Some(component) = components_map.get_mut(target) {
                    component.visible = false;
                    Ok(())
                } else {
                    Err(format!("component '{}' not found", target))
                }
            }),
        );

        self.register_action(
            "toggle",
            Rc::new(|executor, handler| {
                let target = handler.target.as_ref().ok_or("no target specified")?;
                let mut components_signal = executor.ui_state.components;
                let mut components_map = components_signal.write();
                if let Some(component) = components_map.get_mut(target) {
                    component.visible = !component.visible;
                    Ok(())
                } else {
                    Err(format!("component '{}' not found", target))
                }
            }),
        );
    }

    fn register_content_actions(&mut self) {
        self.register_action(
            "update",
            Rc::new(
                |executor: &mut ActionExecutor, handler: &ActionEventHandler| {
                    let target = handler.target.as_ref().ok_or("no target specified")?;
                    let payload = handler.payload.as_ref().ok_or("no payload provided")?;
                    executor.update_content(target, payload)
                },
            ),
        );
    }

    fn register_lifecycle_actions(&mut self) {
        self.register_action(
            "create",
            Rc::new(
                |executor: &mut ActionExecutor, handler: &ActionEventHandler| {
                    let payload = handler.payload.as_ref().ok_or("no payload provided")?;
                    executor.create_component(payload)
                },
            ),
        );

        self.register_action(
            "destroy",
            Rc::new(
                |executor: &mut ActionExecutor, handler: &ActionEventHandler| {
                    let target = handler.target.as_ref().ok_or("no target specified")?;
                    let mut components_signal = executor.ui_state.components;
                    let mut components_map = components_signal.write();
                    if components_map.remove(target).is_some() {
                        Ok(())
                    } else {
                        Err(format!("component '{}' not found", target))
                    }
                },
            ),
        );
    }

    fn register_state_actions(&mut self) {
        self.register_action(
            "setState",
            Rc::new(|executor, handler| {
                let payload = handler.payload.as_ref().ok_or("no payload provided")?;
                executor.set_state(payload.clone(), handler.target.as_deref())
            }),
        );
    }

    fn register_animation_actions(&mut self) {
        self.register_action(
            "animate",
            Rc::new(
                |executor: &mut ActionExecutor, handler: &ActionEventHandler| {
                    let target = handler.target.as_ref().ok_or("no target specified")?;
                    let payload: Option<&Value> = handler.payload.as_ref();
                    executor.trigger_animation(target, payload)
                },
            ),
        );
    }

    fn register_navigation_actions(&mut self) {
        self.register_action(
            "navigate",
            Rc::new(|executor, handler| {
                let payload = handler.payload.as_ref().ok_or("no payload provided")?;
                executor.navigate(payload.clone())
            }),
        );
    }

    fn register_data_actions(&mut self) {
        self.register_action(
            "submit",
            Rc::new(|executor, handler| executor.handle_submit(handler)),
        );

        self.register_action(
            "collect",
            Rc::new(|executor, handler| executor.handle_collect(handler)),
        );

        self.register_action(
            "validate",
            Rc::new(|executor, handler| executor.handle_validate(handler)),
        );
    }
}
