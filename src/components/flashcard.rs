use crate::action_executor::{use_action_executor, ActionContext, ActionExecutor, EventHandler};
use dioxus::prelude::*;
use serde_json::json;

#[derive(Props, Clone, PartialEq)]
pub struct FlashcardProps {
    pub id: String,
    pub content: String,
    pub on_click: Option<EventHandler>,
    pub animations: Option<serde_json::Value>,
}

#[component]
pub fn Flashcard(props: FlashcardProps) -> Element {
    let action_executor = use_action_executor();
    let is_flipped = use_signal(|| false);

    // Parse the click event handler from the JSON schema
    let click_handler = props.on_click.clone().unwrap_or(EventHandler {
        action: "destroy".to_string(), // Generic destroy action
        target: Some(props.id.clone()),
        payload: None,
        condition: None,
    });

    // Handle the click event
    let handle_click = move |_| {
        // First trigger flip animation
        is_flipped.set(true);

        // Create action context
        let context = ActionContext {
            component_id: props.id.clone(),
            event_type: "click".to_string(),
            user_data: None,
        };

        // Execute the action after a short delay (for animation)
        let executor = action_executor.clone();
        let handler = click_handler.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            executor.execute_action(&handler, context);
        });
    };

    // Check if this card should be removed based on action executor state
    let ui_state = action_executor.get_ui_state();
    let is_visible = ui_state
        .components
        .read()
        .get(&props.id)
        .map(|c| c.visible)
        .unwrap_or(true);

    if !is_visible {
        return rsx! {};
    }

    // Get animation state
    let animation_active = ui_state
        .animations
        .read()
        .get(&props.id)
        .map(|anim| anim.active)
        .unwrap_or(false);

    let card_class = format!(
        "flashcard {} {}",
        if *is_flipped.read() { "flipped" } else { "" },
        if animation_active { "animating" } else { "" }
    );

    rsx! {
        div {
            class: "{card_class}",
            onclick: handle_click,
            div {
                class: "card-front",
                "{props.content}"
            }
            div {
                class: "card-back",
                "✓ Completed!"
            }
        }
    }
}

// Example usage showing different generic actions
#[component]
pub fn FlashcardQuiz() -> Element {
    let action_executor = use_action_executor();

    // This would come from your LLM-generated JSON using generic actions
    let flashcard_data = json!({
        "ui_elements": [
            {
                "id": "flashcard-1",
                "type": "card",
                "content": "What is the capital of France?",
                "events": {
                    "onClick": {
                        "action": "destroy",  // Generic destroy instead of removeComponent
                        "target": "flashcard-1"
                    }
                },
                "animations": {
                    "click": "flip",
                    "exit": "slideOut"
                }
            },
            {
                "id": "flashcard-2",
                "type": "card",
                "content": "What is 2 + 2?",
                "events": {
                    "onClick": {
                        "action": "hide",  // Generic hide action
                        "target": "flashcard-2"
                    }
                }
            }
        ]
    });

    // Parse the JSON into components
    let flashcards = flashcard_data["ui_elements"].as_array().unwrap();

    rsx! {
        div {
            class: "quiz-container",
            h1 { "Flashcard Quiz" }

            // Example of a form with submit action
            div {
                class: "quiz-form",
                input {
                    id: "student-name",
                    placeholder: "Enter your name",
                    oninput: move |evt| {
                        // Store value in component state for later collection
                        let context = ActionContext {
                            component_id: "student-name".to_string(),
                            event_type: "input".to_string(),
                            user_data: Some(json!({"value": evt.value()})),
                        };
                        let handler = EventHandler {
                            action: "setState".to_string(),
                            target: Some("student-name".to_string()),
                            payload: Some(json!({"value": evt.value()})),
                            condition: None,
                        };
                        action_executor.execute_action(&handler, context);
                    }
                }

                button {
                    onclick: move |_| {
                        // Example submit action with flexible data collection
                        let context = ActionContext {
                            component_id: "quiz-form".to_string(),
                            event_type: "submit".to_string(),
                            user_data: None,
                        };
                        let submit_handler = EventHandler {
                            action: "submit".to_string(),
                            target: None,
                            payload: Some(json!({
                                "fields": ["student-name"],  // LLM specifies what fields to collect
                                "endpoint": "/api/quiz-results",  // Where to send data
                                "submissionId": "quiz-submission",
                                "includeGlobalState": true,  // Include global quiz state
                                "onSuccess": {
                                    "action": "navigate",
                                    "payload": { "route": "/results" }
                                }
                            })),
                            condition: None,
                        };
                        action_executor.execute_action(&submit_handler, context);
                    },
                    "Submit Quiz"
                }
            }

            div {
                class: "flashcard-stack",
                for (index, card) in flashcards.iter().enumerate() {
                    {
                        let id = card["id"].as_str().unwrap().to_string();
                        let content = card["content"].as_str().unwrap().to_string();
                        let click_event = card["events"]["onClick"].clone();

                        let event_handler = serde_json::from_value::<EventHandler>(click_event)
                            .unwrap_or(EventHandler {
                                action: "destroy".to_string(),  // Generic destroy
                                target: Some(id.clone()),
                                payload: None,
                                condition: None,
                            });

                        rsx! {
                            Flashcard {
                                key: "{id}",
                                id: id,
                                content: content,
                                on_click: event_handler,
                                animations: card.get("animations").cloned()
                            }
                        }
                    }
                }
            }

            button {
                onclick: move |_| {
                    // Example validate action
                    let context = ActionContext {
                        component_id: "quiz".to_string(),
                        event_type: "click".to_string(),
                        user_data: None,
                    };
                    let validate_handler = EventHandler {
                        action: "validate".to_string(),
                        target: None,
                        payload: Some(json!({
                            "rules": {
                                "student-name": {
                                    "required": true,
                                    "minLength": 2
                                }
                            },
                            "onValid": {
                                "action": "update",
                                "target": "status",
                                "payload": { "content": "✅ All fields valid!" }
                            },
                            "onInvalid": {
                                "action": "update",
                                "target": "status",
                                "payload": { "content": "❌ Please fill all required fields" }
                            }
                        })),
                        condition: None,
                    };
                    action_executor.execute_action(&validate_handler, context);
                },
                "Validate Form"
            }

            div {
                id: "status",
                "Ready to submit"
            }
        }
    }
}
