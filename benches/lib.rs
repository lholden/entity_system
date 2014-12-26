extern crate test;
extern crate entity_system;

#[deriving(Clone)]
struct TestComponent {
    name: &'static str,
} 

#[deriving(Clone)]
struct OtherComponent {
    name: &'static str,
}

#[bench]
fn bench_insert_of_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let entity = em.create();

    b.iter(|| {
        let mut cm = entity_system::ComponentManager::new();
        for _ in range(0u32, 5000) {
            cm.insert(entity, TestComponent{name: "test"});
        }
    });
}

#[bench]
fn bench_find_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let results = cm.find::<OtherComponent>();
        for meta in results.iter() {
            meta.component.name;
        }
    })
}

#[bench]
fn bench_find_for_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    let entity = em.create();
    cm.insert(entity, TestComponent{name: "test"});
    cm.insert(entity, OtherComponent{name: "other"});

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let results = cm.find_for::<OtherComponent>(entity);
        for c in results.iter() {
            c.name;
        }
    })
}

#[bench]
fn bench_get_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    let entity = em.create();
    cm.insert(entity, TestComponent{name: "test"});
    cm.insert(entity, OtherComponent{name: "other"});

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let c = cm.get::<OtherComponent>(entity);
        c.name;
    })
}

#[bench]
fn bench_find_mut_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let mut results = cm.find_mut::<OtherComponent>();
        for meta in results.iter_mut() {
            meta.component.name;
        }
    })
}

#[bench]
fn bench_find_for_mut_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    let entity = em.create();
    cm.insert(entity, TestComponent{name: "test"});
    cm.insert(entity, OtherComponent{name: "other"});

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let mut results = cm.find_for_mut::<OtherComponent>(entity);
        for c in results.iter_mut() {
            c.name;
        }
    })
}

#[bench]
fn bench_get_mut_in_5000(b: &mut test::Bencher) {
    let mut em = entity_system::EntityManager::new();
    let mut cm = entity_system::ComponentManager::new();

    let entity = em.create();
    cm.insert(entity, TestComponent{name: "test"});
    cm.insert(entity, OtherComponent{name: "other"});

    for _ in range(0u32, 5000) {
        let entity = em.create();
        cm.insert(entity, TestComponent{name: "test"});
        cm.insert(entity, OtherComponent{name: "other"});
    } 

    b.iter(|| {
        let c = cm.get_mut::<OtherComponent>(entity);
        c.name;
    })
}
