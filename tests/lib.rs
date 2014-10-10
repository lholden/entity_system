#![feature(phase)]

extern crate test;
extern crate uuid;
extern crate entity_system;

#[phase(plugin)]
extern crate entity_system;

use std::default::Default;
use uuid::Uuid;

component!(TestComponent 
    name: &'static str
)

component!(OtherComponent 
    name: &'static str
)

mod entity_manager {
    extern crate entity_system;
    use entity_system::Component;

    #[test]
    fn new_entities_are_unique() {
        let em = entity_system::EntityManager::new();
        let entity = em.create_entity();
        let entity2 = em.create_entity();

        assert!(entity != entity2);
    }

    #[test]
    fn named_entities() {
        let mut em = entity_system::EntityManager::new();
        let entity = em.create_named_entity("One");
        let entity2 = em.create_named_entity("Two");

        let result = em.get_named_entity("One").unwrap();
        let result2 = em.get_named_entity("Two").unwrap();

        assert!(result != result2);
        assert_eq!(entity, result);
        assert_eq!(entity2, result2)
    }

    #[test]
    fn finds_components_for_entity() {
        let mut em = entity_system::EntityManager::new();
        let entity = em.create_entity();
        let entity_other = em.create_entity();
        let component = super::TestComponent::new();
        let component2 = super::OtherComponent::new();
        let component_other = super::TestComponent::new();

        em.insert(entity, component);
        em.insert(entity, component);
        em.insert(entity, component2);
        em.insert(entity_other, component_other);

        {
            let result = em.find_for::<super::TestComponent>(entity);
            assert_eq!(result.len(), 2);
            assert_eq!(component.get_id(), result[0].get_id());
            assert_eq!(component.get_id(), result[1].get_id());
        }
        {
            let result = em.find_for_mut::<super::TestComponent>(entity);
            assert_eq!(result.len(), 2);
            assert_eq!(component.get_id(), result[0].get_id());
            assert_eq!(component.get_id(), result[1].get_id());
        }
    }

    #[test]
    fn finds_components_for_type() {
        let mut em = entity_system::EntityManager::new();
        let entity = em.create_entity();
        let entity2 = em.create_entity();
        let component = super::TestComponent::new();
        let component_other = super::OtherComponent::new();

        em.insert(entity, component);
        em.insert(entity, component_other);
        em.insert(entity2, component);

        {
            let result = em.find::<super::TestComponent>();
            assert_eq!(result.len(), 2);
            assert_eq!(component.get_id(), result[0].get_id());
            assert_eq!(component.get_id(), result[1].get_id());
        }
        {
            let result = em.find_mut::<super::TestComponent>();
            assert_eq!(result.len(), 2);
            assert_eq!(component.get_id(), result[0].get_id());
            assert_eq!(component.get_id(), result[1].get_id());
        }
    }

    #[test]
    fn find_immutable_before_find_mut() {
        let mut em = entity_system::EntityManager::new();
        let entity = em.create_entity();
        let component = super::TestComponent::new();
        em.insert(entity, component);
        {
            let immutable = em.find_for::<super::TestComponent>(entity);
            assert_eq!(immutable[0].get_id(), component.get_id());

            let mutable = em.find_for_mut::<super::TestComponent>(entity);
            assert_eq!(mutable[0].get_id(), component.get_id())
        }
        {
            let immutable = em.find::<super::TestComponent>();
            assert_eq!(immutable[0].get_id(), component.get_id());

            let mutable = em.find_mut::<super::TestComponent>();
            assert_eq!(mutable[0].get_id(), component.get_id())
        }
    }

    #[test]
    fn find_entities_for_type() {
        let mut em = entity_system::EntityManager::new();
        let entity = em.create_entity();
        let entity2 = em.create_entity();

        em.insert(entity, super::TestComponent::new());
        em.insert(entity, super::TestComponent::new());
        em.insert(entity2, super::TestComponent::new());
        em.insert(entity2, super::OtherComponent::new());

        let result = em.find_entities::<super::TestComponent>();
        assert_eq!(result.len(), 2);
        assert!(result[0] != result[1]);
        for result_entity in result.iter() {
            assert!(result_entity == &entity || result_entity == &entity2);
        }
    }
}
