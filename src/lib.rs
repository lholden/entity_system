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
    component_map: HashMap<TypeId, EntityMap>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            component_map: HashMap::new()
        }
    } 
    
    pub fn create_entity(&self) -> Uuid {
        Uuid::new_v4()
    }

    pub fn insert<T:Component+'static>(&mut self, entity:Uuid, component:T) {
        let entity_map = match self.component_map.entry(TypeId::of::<T>()) {
            Vacant(entry) => entry.set(HashMap::new()),
            Occupied(entry) => entry.into_mut(),
        };

        let component_vec = match entity_map.entry(entity) {
            Vacant(entry) => entry.set(Vec::new()),
            Occupied(entry) => entry.into_mut(),
        };

        component_vec.push(box component as Box<Any>);
    }

    pub fn find<T:Component+'static>(&self) -> Vec<Box<&T>> {
        let mut result:Vec<Box<&T>> = Vec::new();

        match self.component_map.find(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                for (_, component_vec) in entity_map.iter() {
                    for component in component_vec.iter() {
                        result.push(box component.downcast_ref::<T>().unwrap());
                    }
                }
            }                    
        }

        result
    }
    
    pub fn find_mut<T:Component+'static>(&mut self) -> Vec<Box<&mut T>> {
        let mut result:Vec<Box<&mut T>> = Vec::new();

        match self.component_map.find_mut(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                for (_, component_vec) in entity_map.iter_mut() {
                    for component in component_vec.iter_mut() {
                        result.push(box component.downcast_mut::<T>().unwrap());
                    } 
                }
            }                    
        }

        result
    }

    pub fn find_for<T:Component+'static>(&self, entity:Uuid) -> Vec<Box<&T>> {
        let mut result:Vec<Box<&T>> = Vec::new();

        match self.component_map.find(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                match entity_map.find(&entity) {
                    None => {},
                    Some(component_vec) => {
                        for component in component_vec.iter() {
                            result.push(box component.downcast_ref::<T>().unwrap())
                        }
                    }
                }
            }

        }

        result
    }

    pub fn find_for_mut<T:Component+'static>(&mut self, entity:Uuid) -> Vec<Box<&mut T>> {
        let mut result:Vec<Box<&mut T>> = Vec::new();

        match self.component_map.find_mut(&TypeId::of::<T>()) {
            None => {},
            Some(entity_map) => {
                match entity_map.find_mut(&entity) {
                    None => {},
                    Some(component_vec) => {
                        for component in component_vec.iter_mut() {
                            result.push(box component.downcast_mut::<T>().unwrap())
                        }
                    }
                }
            }

        }

        result
    }
}
