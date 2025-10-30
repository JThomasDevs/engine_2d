#[allow(clippy::module_inception)]
mod animation;

pub use animation::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_animation_creation() {
        let no_anim = NoAnimation::new();
        assert_eq!(no_anim.name(), "No Animation");
    }

    #[test]
    fn test_no_animation_trait_implementation() {
        let no_anim = NoAnimation::new();

        // Test that it implements the Animation trait
        assert_eq!(no_anim.name(), "No Animation");

        // Test that update doesn't crash (it should do nothing)
        // Note: We can't easily test the update method without a SpriteRenderer
        // but we can test that the trait is properly implemented
        let name = no_anim.name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_animation_trait_contract() {
        // Test that the Animation trait is properly defined
        let no_anim = NoAnimation::new();

        // Test name method
        let name = no_anim.name();
        assert!(name.len() > 0);
        assert_eq!(name, "No Animation");
    }

    #[test]
    fn test_animation_module_structure() {
        // Test that the animation module is properly structured
        use crate::animation::Animation;
        use crate::animation::NoAnimation;

        // Test that we can create and use the types
        let anim: Box<dyn Animation> = Box::new(NoAnimation::new());
        assert_eq!(anim.name(), "No Animation");
    }

    #[test]
    fn test_animation_trait_object_safety() {
        // Test that Animation trait can be used as a trait object
        let animations: Vec<Box<dyn Animation>> = vec![Box::new(NoAnimation::new())];

        for anim in animations {
            assert!(!anim.name().is_empty());
        }
    }
}
