extern crate concurrent_fixed_hashmap;

use std::sync::Arc;
use concurrent_fixed_hashmap::ConcurrentFixedHashMap;

#[test]
fn test_insert_and_get() {
    let mut handlers = Vec::new();

    let map = Arc::new(ConcurrentFixedHashMap::with_size(1 << 27));

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert((100 * i, 1000 * i, 10000 * i), 999 * i);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for (i,h) in (1..11).zip(handlers.into_iter()) {
        h.join().unwrap();

        if let Some(e) = map.get(&(100 * i, 1000 * i, 10000 * i)) {
            assert_eq!(999 * i, *e);
        } else{
            assert!(false);
        }
    }
}
#[test]
fn test_insert_and_get_mut() {
    let mut handlers = Vec::new();

    let map = Arc::new(ConcurrentFixedHashMap::with_size(1 << 27));

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert((100 * i, 1000 * i, 10000 * i), 999 * i);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }

    let mut handlers = Vec::new();

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            if let Some(mut e) = map.get_mut(&(100 * i, 1000 * i, 10000 * i)) {
                *e = *e * 100;
            }
        }));
    }

    for (i,h) in (1..11).zip(handlers.into_iter()) {
        h.join().unwrap();

        if let Some(e) = map.get(&(100 * i, 1000 * i, 10000 * i)) {
            assert_eq!(999 * i * 100, *e);
        } else{
            assert!(false);
        }
    }
}
#[test]
fn test_insert_new_and_get() {
    let mut handlers = Vec::new();

    let map = Arc::new(ConcurrentFixedHashMap::with_size(1 << 27));

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert_new((100 * i, 1000 * i, 10000 * i), 999 * i);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for (i,h) in (1..11).zip(handlers.into_iter()) {
        h.join().unwrap();

        if let Some(e) = map.get(&(100 * i, 1000 * i, 10000 * i)) {
            assert_eq!(999 * i, *e);
        } else{
            assert!(false);
        }
    }
}
#[test]
fn test_insert_and_insert_new_and_get() {
    let mut handlers = Vec::new();

    let map = Arc::new(ConcurrentFixedHashMap::with_size(1 << 27));

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert((100 * i, 1000 * i, 10000 * i), 999 * i);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }

    let mut handlers = Vec::new();

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert_new((100 * i, 1000 * i, 10000 * i), 999 * i * 1000);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for (i,h) in (1..11).zip(handlers.into_iter()) {
        h.join().unwrap();

        if let Some(e) = map.get(&(100 * i, 1000 * i, 10000 * i)) {
            assert_eq!(999 * i, *e);
        } else{
            assert!(false);
        }
    }
}
#[test]
fn test_insert_and_get_mut_and_conatins() {
    let mut handlers = Vec::new();

    let map = Arc::new(ConcurrentFixedHashMap::with_size(1 << 27));

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            map.insert((100 * i, 1000 * i, 10000 * i), 999 * i);

            if let None = map.get(&(100 * i, 1000 * i, 10000 * i)) {
                panic!("item not get.");
            }
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }

    let mut handlers = Vec::new();

    for i in 1..11 {
        let map = Arc::clone(&map);

        handlers.push(std::thread::spawn(move || {
            if let Some(mut e) = map.get_mut(&(100 * i, 1000 * i, 10000 * i)) {
                *e = *e * 100;
            }
        }));
    }

    for (i,h) in (1..11).zip(handlers.into_iter()) {
        h.join().unwrap();

        assert!(map.contains_key(&(100 * i, 1000 * i, 10000 * i)));
    }
}
