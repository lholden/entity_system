extern crate uuid;

use uuid::Uuid;
use std::intrinsics::TypeId;
use std::collections::hashmap::HashMap;
use std::collections::hashmap::{Vacant, Occupied};
use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub trait Component {
    fn get_id(&self) -> Uuid;
}

type ComponentVec = Vec<Box<Any>>;
type EntityMap = HashMap<Uuid, ComponentVec>;

pub struct EntityManager {
    named_entities: HashMap<&'static str, Uuid>,
    components: HashMap<TypeId, EntityMap>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            named_entities: HashMap::new(),
            components: HashMap::new(),
        }
    } 
    
    pub fn create_entity(&self) -> Uuid {
        Uuid::new_v4()
    }

    pub fn create_named_entity(&mut self, name: &'static str) -> Uuid {
        let entity = self.create_entity();
        self.named_entities.insert(name, entity);
        entity
    }

    pub fn get_named_entity(&self, name: &'static str) -> Result<Uuid, String> {
        match self.named_entities.find(&name) {
            None => Err(format!("could not find entity for name: {}", name)),
            Some(entity) => Ok(entity.clone()),
        }
    }

    pub fn insert<T:Component+'static>(&mut self, entity:Uuid, component:T) {
        let entity_map = match self.components.entry(TypeId::of::<T>()) {
            Vacant(entry) => entry.set(HashMap::new()),
            Occupied(entry) => entry.into_mut(),
        };

        let component_vec = match entity_map.entry(entity) {
            Vacant(entry) => entry.set(Vec::new()),
            Occupied(entry) => entry.into_mut(),
        };

        component_vec.push(box component as Box<Any>);
    }

    pub fn find<T:Component+Clone+'static>(&self) -> Vec<T> {
        let mut result:Vec<T> = Vec::new();

        match self.components.find(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                result.reserve_additional(entity_map.len());
                for (_, component_vec) in entity_map.iter() {
                    if component_vec.len() > 1 {
                        result.reserve_additional(component_vec.len()-1);
                    }
                    for component in component_vec.iter() {
                        result.push(component.downcast_ref::<T>().unwrap().clone());
                    }
                }
            }                    
        }

        result
    }
    
    pub fn find_mut<T:Component+'static>(&mut self) -> Vec<&mut T> {
        let mut result:Vec<&mut T> = Vec::new();

        match self.components.find_mut(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                result.reserve_additional(entity_map.len());
                for (_, component_vec) in entity_map.iter_mut() {
                    if component_vec.len() > 1 {
                        result.reserve_additional(component_vec.len()-1);
                    }
                    for component in component_vec.iter_mut() {
                        result.push(component.downcast_mut::<T>().unwrap());
                    } 
                }
            }                    
        }

        result
    }

    pub fn find_for<T:Component+Clone+'static>(&self, entity:Uuid) -> Vec<T> {
        let mut result:Vec<T> = Vec::new();

        match self.components.find(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                match entity_map.find(&entity) {
                    None => {},
                    Some(component_vec) => {
                        result.reserve_additional(component_vec.len());
                        for component in component_vec.iter() {
                            result.push(component.downcast_ref::<T>().unwrap().clone());
                        }
                    }
                }
            }

        }

        result
    }

    pub fn find_for_mut<T:Component+'static>(&mut self, entity:Uuid) -> Vec<&mut T> {
        let mut result:Vec<&mut T> = Vec::new();

        match self.components.find_mut(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                match entity_map.find_mut(&entity) {
                    None => {},
                    Some(component_vec) => {
                        for component in component_vec.iter_mut() {
                            result.push(component.downcast_mut::<T>().unwrap())
                        }
                    }
                }
            }

        }

        result
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    extern crate uuid;
    extern crate entity_system;

    use std::default::Default;
    use uuid::Uuid;

    #[deriving(Default,Clone)]
    struct TestComponent {
        id: Uuid,
        pub name: &'static str,
    }

    impl TestComponent {
        pub fn new() -> TestComponent {
            TestComponent {
                id: Uuid::new_v4(),
                ..Default::default()
            }
        }
    }

    impl entity_system::Component for TestComponent {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }

    #[deriving(Default,Clone)]
    struct OtherComponent {
        id: Uuid,
        pub name: &'static str,
    }

    impl OtherComponent {
        pub fn new() -> OtherComponent {
            OtherComponent {
                id: Uuid::new_v4(),
                ..Default::default()
            }
        }
    }

    impl entity_system::Component for OtherComponent {
        fn get_id(&self) -> Uuid {
            self.id
        }
    }
    #[bench]
    fn bench_insert(b: &mut test::Bencher) {

        let mut em = entity_system::EntityManager::new();
        let entity = em.create_entity();

        b.iter(|| {
            let tc = TestComponent::new();
            em.insert(entity, tc);
        });
    }

    #[bench]
    fn bench_find(b: &mut test::Bencher) {
        let mut em = entity_system::EntityManager::new();
        for _ in range(0u, 1000) {
            let entity = em.create_entity();
            let tc = TestComponent::new();
            em.insert(entity, tc);
            for _ in range(0u, 10) {
                let oc = OtherComponent::new();
                em.insert(entity, oc);
            }
        }

        b.iter(|| {
            em.find::<TestComponent>();
        });
    }

    #[bench]
    fn bench_find_multiple_per_entity(b: &mut test::Bencher) {
        let mut em = entity_system::EntityManager::new();
        for _ in range(0u, 100) {
            let entity = em.create_entity();
            for _ in range(0u, 100) {
                let tc = TestComponent::new();
                em.insert(entity, tc);
            }
        }

        b.iter(|| {
            em.find::<TestComponent>();
        });
    }

    #[bench]
    fn bench_find_for(b: &mut test::Bencher) {

        let mut em = entity_system::EntityManager::new();
        let known_entity = em.create_entity();
        let tc = TestComponent::new();
        em.insert(known_entity, tc);
        for _ in range(0u, 10) {
            let oc = OtherComponent::new();
            em.insert(known_entity, oc);
        }

        for _ in range(0u, 1000) {
            let entity = em.create_entity();
            let tc = TestComponent::new();
            em.insert(entity, tc);
            for _ in range(0u, 10) {
                let oc = OtherComponent::new();
                em.insert(entity, oc);
            }
        }

        b.iter(|| {
            em.find_for::<TestComponent>(known_entity);
        });
    }

    #[bench]
    fn bench_find_mut(b: &mut test::Bencher) {

        let mut em = entity_system::EntityManager::new();

        for _ in range(0u, 1000) {
            let entity = em.create_entity();
            let tc = TestComponent::new();
            em.insert(entity, tc);
            for _ in range(0u, 10) {
                let oc = OtherComponent::new();
                em.insert(entity, oc);
            }
        }

        b.iter(|| {
            em.find_mut::<TestComponent>();
        });
    }
}
