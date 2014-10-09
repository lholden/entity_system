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
    pub fn new() -> EntityManager 
    {
        EntityManager {
            named_entities: HashMap::new(),
            components: HashMap::new(),
        }
    } 
    
    pub fn create_entity(&self) -> Uuid 
    {
        Uuid::new_v4()
    }

    pub fn create_named_entity(&mut self, name: &'static str) -> Uuid 
    {
        let entity = self.create_entity();
        self.named_entities.insert(name, entity);
        entity
    }

    pub fn get_named_entity(&self, name: &'static str) -> Result<Uuid, String> 
    {
        match self.named_entities.find(&name) {
            None => Err(format!("could not find entity for name: {}", name)),
            Some(entity) => Ok(*entity),
        }
    }


    pub fn insert<T>(&mut self, entity:Uuid, component:T) 
        where T: Component+'static
    {
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

    pub fn find_entities<T>(&self) -> Vec<Uuid> 
        where T: Component+'static
    {
        let mut result:Vec<Uuid> = Vec::new();
        match self.components.find(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                for (entity, _) in entity_map.iter() {
                    result.push(*entity);
                }
            }
        }
        result
    }

    pub fn find<T>(&self) -> Vec<T>
        where T: Component+Clone+'static
    {
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
    
    pub fn find_mut<T>(&mut self) -> Vec<&mut T>
        where T: Component+'static
    {
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

    pub fn find_for<T>(&self, entity:Uuid) -> Vec<T> 
        where T: Component+Clone+'static
    {
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

    pub fn find_for_mut<T>(&mut self, entity:Uuid) -> Vec<&mut T>
        where T: Component+'static
    {
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

