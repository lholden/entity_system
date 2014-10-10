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

use std::default::Default;

// using the component macro:
component!(MyComponent
    x: i32,
    y: i32
)

fn main() {
    let mut em = entity_system::EntitySystem::new();
    let entity = entity_system::EntityManager::new();
    let component = MyComponent::new();
    em.insert(entity, component);

    let immutable = em.find_for::<MyComponent>(entity);
    immutable[0].get_id() == component.get_id();
}
```

*/

#![feature(if_let)]
#![feature(macro_rules, phase)]

extern crate uuid;

use uuid::Uuid;
use std::intrinsics::TypeId;
use std::collections::hashmap::HashMap;
use std::collections::hashmap::{Vacant, Occupied};
use std::any::{Any, AnyRefExt, AnyMutRefExt};

/// A type for unique identifiers
pub type EsId = Uuid;

type ComponentVec = Vec<Box<Any>>;
type EntityMap = HashMap<EsId, ComponentVec>;


/// Pure data that is used to compose various discrete aspects on an entity.
pub trait Component {
    fn get_id(&self) -> EsId;
}

/// Generates a unique id of type EsId
pub fn generate_id() -> EsId
{
    Uuid::new_v4()
}

/// The EntityManager manages the relationships between entities and components.
pub struct EntityManager {
    named_entities: HashMap<&'static str, EsId>,
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

    /// Generate an anonymous entity
    ///
    /// # Example
    /// 
    /// ```rust
    /// let mut em = EntityManager::new();
    /// let entity = em.create_entity();
    /// ```
    pub fn create_entity(&self) -> EsId 
    {
        generate_id()
    }

    /// Generate an entity that can be looked up later
    pub fn create_named_entity(&mut self, name: &'static str) -> EsId 
    {
        let entity = self.create_entity();
        self.named_entities.insert(name, entity);
        entity
    }

    /// Retrieve a named entity
    ///
    /// # Example
    /// 
    /// ```rust
    /// let mut em = EntityManager::new();
    /// let entity = em.create_named_entity("example");
    /// let found = em.get_named_entity("example");
    /// ```
    pub fn get_named_entity(&self, name: &'static str) -> Result<EsId, String> 
    {
        match self.named_entities.find(&name) {
            None => Err(format!("could not find entity for name: {}", name)),
            Some(entity) => Ok(*entity),
        }
    }

    /// Inserts a Component into the system for the specified entity
    pub fn insert<T>(&mut self, entity:EsId, component:T) 
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

    /// Find all of the entities having Components of the specified type
    ///
    /// # Example
    /// 
    /// ```rust
    /// let mut em = EntityManager::new();
    /// let my_entity = em.create_entity();
    /// let my_component = MyComponent::new();
    ///
    /// em.insert(my_entity, my_component);
    ///
    /// let entities = em.find_entities<MyComponent>();
    /// for entity in entities.iter() {
    ///     println!("My EsId is: {}", entity);
    /// }
    /// ```
    pub fn find_entities<T>(&self) -> Vec<EsId> 
        where T: Component+'static
    {
        let mut result:Vec<Uuid> = Vec::new();
        if let Some(entity_map) = self.components.find(&TypeId::of::<T>()) {
            for (entity, _) in entity_map.iter() {
                result.push(*entity);
            }
        }
        result
    }

    /// Retrieve a list of components for the specified type. Must be used 
    /// before any find_*mut calls.
    pub fn find<T>(&self) -> Vec<T>
        where T: Component+Clone+'static
    {
        let mut result:Vec<T> = Vec::new();

        if let Some(entity_map) = self.components.find(&TypeId::of::<T>()) {
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

        result
    }
    
    /// Retrieve a list of mutable components for the specified type
    ///
    /// # Example
    /// 
    /// ```rust
    /// component!(MyComponent
    ///     name: &'static str
    /// )
    /// let mut em = EntityManager::new();
    /// let my_entity = em.create_entity();
    /// let my_component = MyComponent::new();
    ///
    /// em.insert(my_entity, my_component);
    ///
    /// let components = em.find_mut<MyComponent>();
    /// for component in components.iter() {
    ///     println!("Changing components name from: {}", component.name);
    ///     component.name = "A new name for all MyComponents";
    /// }
    /// ```
    pub fn find_mut<T>(&mut self) -> Vec<&mut T>
        where T: Component+'static
    {
        let mut result:Vec<&mut T> = Vec::new();

        if let Some(entity_map) = self.components.find_mut(&TypeId::of::<T>()) {
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

        result
    }

    /// Retrieves a list of components of the specified type for a specific 
    /// Entity. Must be used before any find_*mut calls.
    pub fn find_for<T>(&self, entity:EsId) -> Vec<T> 
        where T: Component+Clone+'static
    {
        let mut result:Vec<T> = Vec::new();

        if let Some(entity_map) = self.components.find(&TypeId::of::<T>()) {
            if let Some(component_vec) = entity_map.find(&entity) {
                result.reserve_additional(component_vec.len());
                for component in component_vec.iter() {
                    result.push(component.downcast_ref::<T>().unwrap().clone());
                }
            }
        }

        result
    }

    /// Retrieves a list of mutable components of the specified type for a specific Entity
    pub fn find_for_mut<T>(&mut self, entity:EsId) -> Vec<&mut T>
        where T: Component+'static
    {
        let mut result:Vec<&mut T> = Vec::new();

        if let Some(entity_map) = self.components.find_mut(&TypeId::of::<T>()) {
            if let Some(component_vec) = entity_map.find_mut(&entity) {
                for component in component_vec.iter_mut() {
                    result.push(component.downcast_mut::<T>().unwrap())
                }
            }
        }

        result
    }
}

/// A simple macro for generating a named Component struct.
///
/// # Arguments
///     
///     * The name you want your structure to have
///     * A list of field:type pairs
///
/// # Example
///
/// ```rust 
/// component!(MyComponent
///     x: i32,
///     y: i32
/// )
/// ```
/// 
/// Expands to:
///
/// ```rust
/// struct MyComponent {
///     id: entity_system::EsId,
///     x: i32,
///     y: i32
/// }
///
/// impl MyComponent {
///     pub fn new() -> MyComponent {
///         MyComponent {
///             id: entity_system::generate_id(),
///             ..Default::default()
///         }
///     }
/// }
///
/// impl entity_system::Component for MyComponent {
///     fn get_id(&self) -> entity_system::EsId {
///         self.id
///     }
/// }
/// ```
#[macro_export]
macro_rules! component {
    ($name:ident $($element: ident: $ty: ty),*) => {
        #[deriving(Default,Clone)]
        struct $name {
            id: entity_system::EsId,
            $($element: $ty),* 
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                    id: entity_system::generate_id(),
                    ..Default::default()
                }
            }
        }

        impl entity_system::Component for $name {
            fn get_id(&self) -> entity_system::EsId {
                self.id
            }
        }
    }
}
