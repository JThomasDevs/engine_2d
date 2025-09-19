/// Example demonstrating the input system
/// 
/// This example shows how to use the hybrid macro + type-safe input system
/// for defining and managing game actions.

use engine_2d::input::*;

fn main() {
    println!("🎮 Input System Demo");
    println!("===================");
    
    // Create an input manager
    let mut input_manager = InputManager::new();
    
    // Register predefined actions
    let actions = get_predefined_actions();
    input_manager.register_actions(actions);
    
    println!("\n📋 Registered Actions:");
    println!("---------------------");
    
    // Display all registered actions
    for action in input_manager.get_actions() {
        println!("• {} ({})", action.display_name, action.id);
        println!("  Category: {:?}", action.category);
        println!("  Type: {:?}", action.input_type);
        println!("  Bindings: {} binding(s)", action.default_bindings.len());
        if let Some(desc) = &action.metadata.description {
            println!("  Description: {}", desc);
        }
        if !action.metadata.tags.is_empty() {
            println!("  Tags: {}", action.metadata.tags.join(", "));
        }
        println!();
    }
    
    // Demonstrate action categories
    println!("🏃 Movement Actions:");
    println!("-------------------");
    for action in input_manager.get_actions_by_category(ActionCategory::Movement) {
        println!("• {}", action.display_name);
    }
    
    println!("\n⚔️ Combat Actions:");
    println!("------------------");
    for action in input_manager.get_actions_by_category(ActionCategory::Combat) {
        println!("• {}", action.display_name);
    }
    
    println!("\n🖥️ UI Actions:");
    println!("--------------");
    for action in input_manager.get_actions_by_category(ActionCategory::UI) {
        println!("• {}", action.display_name);
    }
    
    // Demonstrate context system
    println!("\n🎯 Context System Demo:");
    println!("----------------------");
    
    // Create a debug context
    let debug_context = InputContext::new("debug_mode".to_string(), 1)
        .enable_action("DEBUG_CONSOLE".to_string())
        .enable_action("DEBUG_TOGGLE_WIREFRAME".to_string());
    
    input_manager.push_context(debug_context);
    println!("• Pushed debug context");
    
    // Create a menu context
    let menu_context = InputContext::new("menu_mode".to_string(), 2)
        .disable_action("FIRE_WEAPON".to_string())
        .disable_action("MOVE_FORWARD".to_string());
    
    input_manager.push_context(menu_context);
    println!("• Pushed menu context (disables combat and movement)");
    
    // Simulate some input
    println!("\n🎮 Simulating Input:");
    println!("-------------------");
    
    // Simulate pressing W key
    input_manager.set_physical_input_state(
        PhysicalInput::Keyboard(KeyCode::W), 
        true
    );
    println!("• Pressed W key");
    
    // Simulate mouse movement
    input_manager.set_physical_input_value(
        PhysicalInput::MouseAxis(MouseAxis::X), 
        0.5
    );
    println!("• Mouse moved right (0.5)");
    
    // Update the input manager
    input_manager.update(0.016); // 60 FPS delta time
    
    // Check action states
    println!("\n📊 Action States:");
    println!("-----------------");
    
    if input_manager.is_action_pressed("MOVE_FORWARD") {
        println!("✅ MOVE_FORWARD is pressed!");
    } else {
        println!("❌ MOVE_FORWARD is not pressed");
    }
    
    let mouse_look_x = input_manager.get_action_value("MOUSE_LOOK_X");
    println!("🖱️ Mouse Look X value: {:.2}", mouse_look_x);
    
    // Check if debug actions are enabled in current context
    if input_manager.is_action_enabled("DEBUG_CONSOLE") {
        println!("✅ DEBUG_CONSOLE is enabled in current context");
    } else {
        println!("❌ DEBUG_CONSOLE is disabled in current context");
    }
    
    // Check if combat actions are disabled in menu context
    if input_manager.is_action_enabled("FIRE_WEAPON") {
        println!("✅ FIRE_WEAPON is enabled");
    } else {
        println!("❌ FIRE_WEAPON is disabled (menu context)");
    }
    
    // Pop contexts
    println!("\n🔄 Context Management:");
    println!("---------------------");
    
    if let Some(context) = input_manager.pop_context() {
        println!("• Popped context: {}", context.name);
    }
    
    if let Some(context) = input_manager.pop_context() {
        println!("• Popped context: {}", context.name);
    }
    
    // Check if combat is re-enabled
    if input_manager.is_action_enabled("FIRE_WEAPON") {
        println!("✅ FIRE_WEAPON is now enabled again");
    } else {
        println!("❌ FIRE_WEAPON is still disabled");
    }
    
    println!("\n🎉 Input system demo completed!");
    println!("The system supports:");
    println!("• Type-safe action definitions with macros");
    println!("• Rich metadata (descriptions, tags, priorities)");
    println!("• Context-aware input processing");
    println!("• Digital and analog input handling");
    println!("• Multiple input bindings per action");
    println!("• Simultaneous multi-type input processing");
}
