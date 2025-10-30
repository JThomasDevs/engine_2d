/// Macro system for defining game actions
///
/// This module provides macros for easily defining game actions with rich metadata
/// and type-safe access patterns.
// Macro definitions - no imports needed here
/// Define game actions with rich metadata
///
/// This macro generates type-safe action definitions with the following syntax:
///
/// ```rust
/// define_actions! {
///     ACTION_ID: {
///         name: "Display Name",
///         category: Category,
///         input_type: InputType,
///         bindings: [InputBinding::Single(...), ...],
///         description: "Optional description",
///         tags: ["tag1", "tag2"],
///         priority: 1,
///         context: "required_context",
///     };
/// }
/// ```
///
/// # Example
///
/// ```rust
/// define_actions! {
///     MOVE_FORWARD: {
///         name: "Move Forward",
///         category: Movement,
///         input_type: Digital,
///         bindings: [
///             InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
///             InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::DPadUp))
///         ],
///         description: "Move the player forward",
///         tags: ["movement", "basic"],
///         priority: 1,
///     };
/// }
/// ```
#[macro_export]
macro_rules! define_actions {
    (
        $(
            $action_id:ident: {
                name: $display_name:expr,
                category: $category:ident,
                input_type: $input_type:ident,
                bindings: [$($binding:expr),*],
                $(description: $description:expr,)?
                $(tags: [$($tag:expr),*],)?
                $(priority: $priority:expr,)?
                $(context: $context:expr,)?
            }
        );* $(;)?
    ) => {
        // Generate the action definitions
        $(
            /// Action constant for type-safe access
            pub const $action_id: &str = stringify!($action_id);

            paste::paste! {
                #[allow(non_snake_case)]
                pub fn [<$action_id _action>]() -> GameAction {
                    GameAction {
                        id: stringify!($action_id).to_string(),
                        display_name: $display_name.to_string(),
                        category: ActionCategory::$category,
                        input_type: InputType::$input_type,
                        default_bindings: vec![$($binding),*],
                        metadata: ActionMetadata {
                            description: None $(.or(Some($description.to_string())))?,
                            tags: vec![$($($tag.to_string(),)*)?],
                            priority: 0 $(.max($priority))?,
                            context_required: None $(.or(Some($context.to_string())))?,
                        },
                    }
                }
            }
        )*

        /// Get all defined actions
        pub fn get_all_actions() -> Vec<GameAction> {
            paste::paste! {
                vec![$([<$action_id _action>]()),*]
            }
        }

        /// Get actions by category
        pub fn get_actions_by_category(category: ActionCategory) -> Vec<GameAction> {
            get_all_actions().into_iter()
                .filter(|action| action.category == category)
                .collect()
        }

        /// Get actions by tag
        pub fn get_actions_by_tag(tag: &str) -> Vec<GameAction> {
            get_all_actions().into_iter()
                .filter(|action| action.metadata.tags.contains(&tag.to_string()))
                .collect()
        }

        /// Get action by ID
        pub fn get_action_by_id(action_id: &str) -> Option<GameAction> {
            get_all_actions().into_iter()
                .find(|action| action.id == action_id)
        }
    };
}

/// Convenience macro for creating common input bindings
#[macro_export]
macro_rules! input_bindings {
    // Single keyboard key
    (key($key:ident)) => {
        InputBinding::Single(PhysicalInput::Keyboard(KeyCode::$key))
    };

    // Single mouse button
    (mouse($button:ident)) => {
        InputBinding::Single(PhysicalInput::Mouse(MouseButton::$button))
    };

    // Single gamepad button
    (gamepad($button:ident)) => {
        InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::$button))
    };

    // Mouse axis with threshold and deadzone
    (mouse_axis($axis:ident, $threshold:expr, $deadzone:expr)) => {
        InputBinding::Analog {
            input: PhysicalInput::MouseAxis(MouseAxis::$axis),
            threshold: $threshold,
            deadzone: $deadzone,
        }
    };

    // Gamepad axis with threshold and deadzone
    (gamepad_axis($axis:ident, $threshold:expr, $deadzone:expr)) => {
        InputBinding::Analog {
            input: PhysicalInput::GamepadAxis(GamepadAxis::$axis),
            threshold: $threshold,
            deadzone: $deadzone,
        }
    };

    // Modifier + key combination
    (modifier($mod:ident, $key:ident)) => {
        InputBinding::Modified {
            modifier: PhysicalInput::Keyboard(KeyCode::$mod),
            key: PhysicalInput::Keyboard(KeyCode::$key),
        }
    };

    // Multiple simultaneous inputs
    (combo($($input:expr),+)) => {
        InputBinding::Combo(vec![$($input),+])
    };
}

/// Macro for creating common action categories
#[macro_export]
macro_rules! action_categories {
    () => {
        pub mod categories {
            use super::*;

            pub const MOVEMENT: ActionCategory = ActionCategory::Movement;
            pub const COMBAT: ActionCategory = ActionCategory::Combat;
            pub const UI: ActionCategory = ActionCategory::UI;
            pub const DEBUG: ActionCategory = ActionCategory::Debug;
            pub const INTERACTION: ActionCategory = ActionCategory::Interaction;

            pub fn custom(name: &str) -> ActionCategory {
                ActionCategory::Custom(name.to_string())
            }
        }
    };
}

/// Macro for creating common input types
#[macro_export]
macro_rules! input_types {
    () => {
        pub mod input_types {
            use super::*;

            pub const DIGITAL: InputType = InputType::Digital;
            pub const ANALOG: InputType = InputType::Analog;
            pub const HYBRID: InputType = InputType::Hybrid;
        }
    };
}
