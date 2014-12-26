#![feature(phase)]

extern crate test;
extern crate entity_system;

#[phase(plugin)]
extern crate entity_system;

mod test_entity_manager {
    extern crate entity_system;
    use entity_system::{EntityManager};

    #[test]
    fn new_entities_are_unique() {
        let mut em = EntityManager::new();
        let entity = em.create();
        let entity2 = em.create();

        assert!(entity != entity2);
    }

    #[test]
    fn named_entities() {
        let mut em = EntityManager::new();
        let entity = em.create_named("One");
        let entity2 = em.create_named("Two");

        let result = em.get_named("One").unwrap();
        let result2 = em.get_named("Two").unwrap();

        assert!(result != result2);
        assert_eq!(entity, result);
        assert_eq!(entity2, result2)
    }
}
mod test_component_manager {
    extern crate entity_system;
    use entity_system::{EntityManager, ComponentManager};

    #[deriving(Clone)]
    pub struct TestComponent {
        pub name: &'static str,
    }

    #[deriving(Clone)]
    pub struct OtherComponent {
        pub name: &'static str,
    }


    #[test]
    fn finds_components_for_type() {
        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let entity = em.create();
        let entity2 = em.create();
        let component = TestComponent{name: "one"};
        let component2 = TestComponent{name: "two"};
        let component_other = OtherComponent{name: "other"};

        cm.insert(entity, component.clone());
        cm.insert(entity2, component2.clone());
        cm.insert(entity, component_other.clone());

        {
            assert!(cm.contains::<TestComponent>());
            let result = cm.find::<TestComponent>();
            assert_eq!(result.len(), 2);

            assert_eq!(component.name, result[0].component.name);
            assert_eq!(component2.name, result[1].component.name);
        }
        {
            let mut result = cm.find_mut::<TestComponent>();
            assert_eq!(result.len(), 2);
            assert_eq!(component.name, result[0].component.name);
            assert_eq!(component2.name, result[1].component.name);
            result[1].component.name = "modified";
        }
        {
            let result2 = cm.find::<TestComponent>();
            assert_eq!("modified", result2[1].component.name);
        }
    }

    #[test]
    pub fn can_delete_components_of_type() {
        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let entity = em.create();
        let entity2 = em.create();
        let component = TestComponent{name: "one"};
        let component2 = TestComponent{name: "two"};
        let component_other = OtherComponent{name: "other"};

        cm.insert(entity, component.clone());
        cm.insert(entity2, component2.clone());
        cm.insert(entity, component_other.clone());
        
        assert!(cm.contains::<TestComponent>());
        assert!(cm.contains::<OtherComponent>());

        assert!(cm.remove::<TestComponent>());
        assert!(!cm.contains::<TestComponent>(), "Should no longer contain component");
        assert!(cm.contains::<OtherComponent>());

        assert!(!cm.remove::<TestComponent>(), "Removal of non-existent component should return false");
    }

    #[test]
    fn finds_components_for_entity() {
        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let entity = em.create();
        let entity_other = em.create();
        let component = TestComponent{name: "one"};
        let component2 = TestComponent{name: "two"};
        let component_other = OtherComponent{name: "other"};
        let component_entity_other = TestComponent{name: "other_entity"};

        cm.insert(entity, component.clone());
        cm.insert(entity, component2.clone());
        cm.insert(entity, component_other.clone());
        cm.insert(entity_other, component_entity_other.clone());

        {
            let result = cm.find_for::<TestComponent>(entity);
            assert_eq!(result.len(), 2);
            assert_eq!(component.name, result[0].name);
            assert_eq!(component2.name, result[1].name);
        }
        {
            let result = cm.find_for::<TestComponent>(entity_other);
            assert_eq!(result.len(), 1);
            assert_eq!(component_entity_other.name, result[0].name);
        }
        {
            let result = cm.find_for_mut::<TestComponent>(entity_other);
            assert_eq!(result.len(), 1);
            assert_eq!(component_entity_other.name, result[0].name);
        }
        {
            let mut result = cm.find_for_mut::<TestComponent>(entity);
            assert_eq!(result.len(), 2);
            assert_eq!(component.name, result[0].name);
            assert_eq!(component2.name, result[1].name);
            result[0].name = "modified";
        }
        {
            let result = cm.find_for::<TestComponent>(entity);
            assert_eq!("modified", result[0].name);
        }
    }

    #[test]
    fn gets_component_for_entity() {
        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let entity = em.create();
        let entity_other = em.create();
        let component = TestComponent{name: "one"};
        let component2 = TestComponent{name: "two"};
        let component_other = OtherComponent{name: "other"};
        let component_entity_other = TestComponent{name: "other_entity"};

        cm.insert(entity, component.clone());
        cm.insert(entity, component2.clone());
        cm.insert(entity, component_other.clone());
        cm.insert(entity_other, component_entity_other.clone());

        {
            let result = cm.get::<TestComponent>(entity);
            assert_eq!(component.name, result.name);
        }
        {
            let result = cm.get::<TestComponent>(entity_other);
            assert_eq!(component_entity_other.name, result.name);
        }
        {
            let result = cm.get_mut::<TestComponent>(entity_other);
            assert_eq!(component_entity_other.name, result.name);
        }
        {
            let result = cm.get_mut::<TestComponent>(entity);
            assert_eq!(component.name, result.name);
            result.name = "modified";
        }
        {
            let result = cm.get::<TestComponent>(entity);
            assert_eq!("modified", result.name);
        }
    }

    #[test]
    fn find_immutable_before_find_mut() {
        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let entity = em.create();
        let component = TestComponent{name: "one"};
        cm.insert(entity, component.clone());
        {
            let immutable = cm.find::<TestComponent>();
            assert_eq!(immutable[0].component.name, component.name);

            let mutable = cm.find_mut::<TestComponent>();
            assert_eq!(mutable[0].component.name, component.name);
        }
        {
            let immutable = cm.find_for::<TestComponent>(entity);
            assert_eq!(immutable[0].name, component.name);

            let mutable = cm.find_for_mut::<TestComponent>(entity);
            assert_eq!(mutable[0].name, component.name);
        }
        {
            let immutable = cm.get::<TestComponent>(entity);
            assert_eq!(immutable.name, component.name);

            let mutable = cm.get::<TestComponent>(entity);
            assert_eq!(mutable.name, component.name);
        }
    }

    #[test]
    fn find_entities_for_type() {
        let mut em = entity_system::EntityManager::new();
        let mut cm = entity_system::ComponentManager::new();
        let entity = em.create();
        let entity2 = em.create();

        cm.insert(entity, TestComponent{name: "entity_test"});
        cm.insert(entity, TestComponent{name: "entity_test2"});
        cm.insert(entity2, TestComponent{name: "entity2_test"});
        cm.insert(entity2, OtherComponent{name: "entity2_other"});

        let result = cm.find_entities_for_type::<TestComponent>();
        assert_eq!(result.len(), 2);
        assert!(result[0] != result[1]);
        for &e in result.iter() {
            assert!(e == entity || e == entity2);
        }
    }
}
