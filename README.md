# Entity System

A Rust based Entity System designed around the "RDBMS with code in systems" ES approach. 

Check out the [Documentation](http://lholden.github.io/entity_system) for more information.

[![Build Status](https://travis-ci.org/lholden/entity_system.png?branch=master)](https://travis-ci.org/lholden/entity_system)

## Preamble

The name of this API may change to be more memorable in the near future. This API follows the Rust nightlies and may not work on older versions. This API is currently incomplete and may change without warning.

I am planning to implement a simple example game demonstrating how one uses this system in the near future. Until then, the API documentation, tests, and benchmarks are the best place to learn how to use the Entity System.

## Quickstart

Entity System follows the most current Rust nightly, but may work with the most recent release.

Edit your Cargo.toml file to include:
```toml
[dependencies.entity_system]
git = "https://github.com/lholden/entity_system.git"
```

## Concept

An API that allows the programmer to keep game logic and data separate from each other. This allows one to use composition rather than inheritance for the code architecture.

There are three primary concepts:

1. Entity: A unique identifier that for a game object. An Entity does not contain data or code.
2. Component: Pure data that is used to compose various discrete aspects on an entity.
3. Processor: Monolithic opaque "Processors" that run continuously, performing global actions such as rendering or input, iterating through and modifying components, and otherwise performing the game logic.
    * e.g. "Physics System" runs once every 10 game-ticks, iterates over all physical objects, runs a frame of the physics-simulation
    * e.g. "Rendering System" runs once per game-tick, iterates over all objects that have a 2D/3D representation, and renders them to screen
    * e.g. "Positioning System" runs once per game-tick, combines physics-sim data, and player input, and info about the game-HUD, to set the positions of all renderable items

For more information on Entity Systems please see http://entity-systems-wiki.t-machine.org/.

## Usage
```rust
#[phase(plugin)]
extern crate entity_system;

use std::default::Default;

// using the component macro:
#[deriving(Clone)]
struct MyComponent {
  x: i32,
  y: i32
}

fn main() {
  let mut em = entity_system::EntitySystem::new();
  let mut cm = entity_system::ComponentSystem::new();
  let entity = entity_system::EntityManager::new();
  cm.insert(entity, MyComponent{x:0, y:0});

  // find components for an entity
  {
    // Get an immutable copy of a component
    let immutable = cm.find_for::<MyComponent>(entity);
    immutable[0].name == component.name;

    // And a mutable copy
    let mutable = cm.find_for_mut::<MyComponent>(entity);
    mutable[0].x = 4;
    mutable[0].y = 10;
  }

  // find components of a specific component type for all entities
  {
    let immutable = cm.find::<MyComponent>();
    ...

    let mutable = cm.find_mut::<MyComponent>();
    ...
  }
}
```
