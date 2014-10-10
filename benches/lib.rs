#![feature(phase)]

extern crate test;
extern crate entity_system;

#[phase(plugin)]
extern crate entity_system;

use std::default::Default;

component!(TestComponent 
    name: &'static str
)

component!(OtherComponent 
    name: &'static str
)

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
