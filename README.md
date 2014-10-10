# Entity System

This is rust based Entity System for game development and is designed around the "RDBMS with code in systems" approach. 

## Quickstart

Entity System follows the most current Rust nightly, but may work with the most recent release.

Edit your Cargo.toml file to include:
```toml
[dependencies.entity_system]
git = "https://github.com/lholden/entity_system.git"
```

## Preamble

The name of this API may change to be more memorable in the near future. This API follows the Rust nightlies and may not work on older versions. The API is currently incomplete, missing documentation, and may change without warning.

I am planning to implement a simple example game demonstrating how one uses this system in the near future. Until then, your best example of usage will be the tests and benchmarks for the library.

## Concept

An API that allows the programmer to keep game logic and data separate from each other. This allows one to use composition rather than inheritance for the code architecture.

There are three primary concepts:

1. Entity: A unique identifier that for a game object. An Entity does not contain data or code.
2. Component: Pure data that is used to compose various discrete aspects on an entity.
3. Processor: Monolithic opaque "Processors" that run continuously, performing global actions such as rendering or input, iterating through and modifying components, and otherwise performing the game logic.
    * e.g. "Physics System" runs once every 10 game-ticks, iterates over all physical objects, runs a frame of the physics-simulation
    * e.g. "Rendering System" runs once per game-tick, iterates over all objects that have a 2D/3D representation, and renders them to screen
    * e.g. "Positioning System" runs once per game-tick, combines physics-sim data, and player input, and info about the game-HUD, to set the positions of all renderable items

For more general information on Entity Systems please see http://entity-systems-wiki.t-machine.org/.

## Usage
```rust
#[phase(plugin)]
extern crate entity_system;

use std::default::Default;
use uuid::Uuid;

// using the component macro:
component!(MyComponent
  x: i32,
  y: i32
)

// Expands out to:
struct MyComponent {
  id: Uuid,
  x: i32,
  y: i32
}

impl MyComponent {
  pub fn new() -> MyComponent {
    MyComponent {
      id: Uuid::new_v4(),
      ..Default::default()
    }
  }
}

impl entity_system::Component for MyComponent {
  fn get_id(&self) -> Uuid {
    self.id
  }
}

fn main() {
  let mut em = entity_system::EntitySystem::new();
  let entity = entity_system::EntityManager::new();
  let component = MyComponent::new();
  em.insert(entity, component);

  // find components for an entity
  {
    // Get an immutable copy of a component
    let immutable = em.find_for::<MyComponent>(entity);
    immutable[0].get_id() == component.get_id();

    let mutable = em.find_for_mut::<MyComponent>(entity);
    mutable[0].x = 4;
    mutable[0].y = 10;
  }

  // find components of a specific component type for all entities
  {
    let immutable = em.find::<MyComponent>();
    ...

    let mutable = em.find_mut::<MyComponent>();
    ...
  }
}
```
