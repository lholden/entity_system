/*!
<a href="https://github.com/lholden/entity_system"><img style="position: absolute; top: 0; left: 0; border: 0;" src="../github.png" alt="Fork me on GitHub"></a>
<style>.sidebar { margin-top: 53px }</style>

# Entity System

This is a rust based Entity System for game development and is designed 
around the "RDBMS with code in systems" approach. 

This API allows the programmer to keep game logic and data separate from each 
other. This allows one to use composition rather than inheritance for the 
code architecture.


1. Entity: A unique identifier of type EsId for a game object. An Entity does 
           not contain data or code.
2. Component: Pure data that is used to compose various discrete aspects on 
              an entity.
3. Processor: Monolithic opaque "Processors" that run continuously, 
              performing global actions such as rendering or input, iterating 
              through and modifying components, and otherwise performing the 
              game logic.
    * e.g. "Physics System" runs once every 10 game-ticks, iterates over all 
                            physical objects, runs a frame of the physics 
                            simulation
    * e.g. "Rendering System" runs once per game-tick, iterates over all 
                              objects that have a 2D/3D representation, and 
                              renders them to screen
    * e.g. "Positioning System" runs once per game-tick, combines physics-sim 
                                data, and player input, and info about the 
                                game-HUD, to set the positions of all 
                                renderable items

For more information on Entity Systems please see http://entity-systems-wiki.t-machine.org/.

## Example Usage
```rust
#[phase(plugin)]
extern crate entity_system;

#[deriving(Clone)]
struct MyComponent {
    name: &'string str,
}

fn main() {
    let mut em = entity_system::EntitySystem::new();
    let mut cm = entity_system::ComponentManager::new();
    let entity = em.create();
    cm.insert(entity, MyComponent{name: "hello"});

    let result = cm.find_for::<MyComponent>(entity);
    println!(result[0].name);
}
```
*/

use std::intrinsics::TypeId;
use std::collections::hash_map::{HashMap, Entry};
use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub type EntityId = u64;

/// A relationship between entity and component
#[deriving(Clone)]
pub struct EntityMeta<T> {
    pub entity: EntityId,
    pub component: T,
}

/// Creates unique entities along and keeps tracked of named entities
pub struct EntityManager {
    id_counter: EntityId,
    named_entities: HashMap<&'static str, EntityId>,
}

impl EntityManager {
    pub fn new() -> EntityManager 
    {
        EntityManager {
            id_counter: 0,
            named_entities: HashMap::new(),
        }
    } 

    /// Generate a unique entity
    ///
    /// # Example
    /// 
    /// ```rust
    /// let mut em = EntityManager::new();
    /// let entity = em.create();
    /// ```
    pub fn create(&mut self) -> EntityId 
    {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn create_named(&mut self, name: &'static str) -> EntityId
    {
        let id = self.create();
        self.named_entities.insert(name, id);
        id
    }

    pub fn get_named(&self, name: &'static str) -> Result<EntityId, String>
    {
        match self.named_entities.get(name) {
            Some(entity) => Ok(*entity),
            None => Err(format!("Could not find named entity: {}", name)),
        }
    }
}

/// The ComponentManager manages the relationships between entities and components.
pub struct ComponentManager {
    components: HashMap<TypeId, Box<Any>>,
    entities: HashMap<EntityId, HashMap<TypeId, Box<Any>>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager
    {
        ComponentManager {
            components: HashMap::new(),
            entities: HashMap::new(),
        }
    } 

    pub fn insert<T>(&mut self, id: EntityId, component: T) 
        where T: 'static 
    {
        let mut components_vec = match self.components.entry(TypeId::of::<T>()) {
            Entry::Vacant(entry) => {
                let vec: Vec<EntityMeta<T>> = Vec::new();
                entry.set(box vec as Box<Any>)
            }
            Entry::Occupied(entry) => entry.into_mut(),
        }.downcast_mut::<Vec<EntityMeta<T>>>()
         .expect("downcast to Vec<(EntityId, T)>");

        let em = EntityMeta{entity:id, component:component};
        components_vec.push(em);

        let mut entity_components_map = match self.entities.entry(id) {
            Entry::Vacant(entry) => entry.set(HashMap::new()),
            Entry::Occupied(entry) => entry.into_mut(),
        };

        let mut entity_components_vec = match entity_components_map.entry(TypeId::of::<T>()) {
            Entry::Vacant(entry) => {
                let vec: Vec<*mut T> = Vec::new();
                entry.set(box vec as Box<Any>)
            }
            Entry::Occupied(entry) => entry.into_mut(),
        }.downcast_mut::<Vec<*mut T>>()
         .expect("downcast to Vec<*mut T>");

        let v = &mut components_vec.last_mut().expect("last component to exist").component;
        entity_components_vec.push(v);
    }


    pub fn find<T>(&self) -> Vec<EntityMeta<T>> 
        where T: Clone+'static
    {
        self.components.get(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_ref::<Vec<EntityMeta<T>>>()
            .expect("downcast to Vec<(EntityId, T)>")
            .iter()
            .map(|meta| meta.clone())
            .collect()
    }

    pub fn find_mut<T>(&mut self) -> Vec<&mut EntityMeta<T>>
        where T: 'static
    {
        self.components.get_mut(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_mut::<Vec<EntityMeta<T>>>()
            .expect("downcast to Vec<(EntityId, &T)>")
            .iter_mut()
            .collect()
    }

    pub fn contains<T>(&self) -> bool
        where T: 'static
    {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn remove<T>(&mut self) -> bool
        where T: 'static
    {
        let result = self.components
            .remove(&TypeId::of::<T>())
            .is_some();

        for (_,v) in self.entities.iter_mut() {
            if v.contains_key(&TypeId::of::<T>()) {
                let result2 = v.remove(&TypeId::of::<T>()).is_some();
                debug_assert_eq!(result, result2);
            } 
        }

        result
    }

    pub fn find_for<T>(&self, id:EntityId) -> Vec<T> 
        where T: Clone+'static
    {
        self.entities.get(&id)
            .expect("entity to exist")
            .get(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_ref::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .iter()
            .map(|&c| unsafe {&*c}.clone() )
            .collect()
    }


    pub fn find_for_mut<T>(&mut self, id:EntityId) -> Vec<&mut T>
        where T: 'static
    {
        self.entities.get_mut(&id)
            .expect("entity to exist")
            .get_mut(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_mut::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .iter_mut()
            .map(|&c| unsafe {&mut *c})
            .collect()
    }


    pub fn get<T>(&self, id:EntityId) -> T 
        where T: Clone+'static
    {
        unsafe{&**self.entities.get(&id)
            .expect("entity to exist")
            .get(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_ref::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .index(&0)}.clone()
    }

    pub fn get_mut<T>(&mut self, id:EntityId) -> &mut T 
        where T: Clone+'static
    {
        unsafe{&mut **self.entities.get_mut(&id)
            .expect("entity to exist")
            .get_mut(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_mut::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .index(&0)}
    }

    pub fn find_entities_for_type<T>(&self) -> Vec<EntityId> 
        where T: 'static
    {
        self.entities
            .iter()
            .filter(|pair| pair.1.contains_key(&TypeId::of::<T>()) )
            .map(|pair| *pair.0 )
            .collect()

    }
}
