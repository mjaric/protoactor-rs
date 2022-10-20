use criterion::{criterion_group, criterion_main, Criterion};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use protoactor::message::Pid;
use std::sync::mpsc::{channel, Sender};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub trait Registry
where
    Self: Sync + 'static,
{
    fn len() -> usize;

    fn get(&self, key: String) -> Pid;

    fn create_next(&self, key: String) -> Pid;
}

pub struct Process {
    pub id: String,
}

struct MutexRegistry {
    // todo: split into sharded maps
    local_processes: Mutex<HashMap<String, Process>>,
}

fn run_mutex_registry(concurrency: i32) {
    let registry = Arc::new(MutexRegistry {
        local_processes: Mutex::new(
            [
                ("0".into(), Process { id: "0".into() }),
                ("1".into(), Process { id: "1".into() }),
                ("2".into(), Process { id: "2".into() }),
                ("3".into(), Process { id: "3".into() }),
                ("4".into(), Process { id: "4".into() }),
                ("5".into(), Process { id: "5".into() }),
                ("6".into(), Process { id: "6".into() }),
                ("7".into(), Process { id: "7".into() }),
                ("8".into(), Process { id: "8".into() }),
                ("9".into(), Process { id: "9".into() }),
                ("10".into(), Process { id: "10".into() }),
                ("11".into(), Process { id: "11".into() }),
                ("12".into(), Process { id: "12".into() }),
                ("13".into(), Process { id: "13".into() }),
                ("14".into(), Process { id: "14".into() }),
                ("15".into(), Process { id: "15".into() }),
                ("16".into(), Process { id: "16".into() }),
            ]
            .into(),
        ),
    });

    let threads: Vec<_> = (1..concurrency)
        .map(|i| {
            let data = Arc::clone(&registry);
            thread::spawn(move || worker_thread(data, i))
        })
        .collect();

    for t in threads {
        t.join().expect("Thread panicked");
    }
    // println!("TOTAL {}", registry.local_processes.lock().unwrap().len())
}

fn worker_thread(registry: Arc<MutexRegistry>, i: i32) {
    let key = i % 16;
    if key != 0 {
        registry
            .local_processes
            .lock()
            .unwrap()
            .entry(key.to_string())
            .or_insert_with(|| Process {
                id: key.to_string(),
            });
    } else {
        let key = uuid::Uuid::new_v4();
        registry
            .local_processes
            .lock()
            .unwrap()
            .entry(key.to_string())
            .or_insert(Process {
                id: key.to_string(),
            });
    }
}

fn task_loop_registry() -> Sender<(String, Sender<Arc<Process>>)> {
    let mut local_processes: HashMap<String, Arc<Process>> = [
        ("0".into(), Arc::new(Process { id: "0".into() })),
        ("1".into(), Arc::new(Process { id: "1".into() })),
        ("2".into(), Arc::new(Process { id: "2".into() })),
        ("3".into(), Arc::new(Process { id: "3".into() })),
        ("4".into(), Arc::new(Process { id: "4".into() })),
        ("5".into(), Arc::new(Process { id: "5".into() })),
        ("6".into(), Arc::new(Process { id: "6".into() })),
        ("7".into(), Arc::new(Process { id: "7".into() })),
        ("8".into(), Arc::new(Process { id: "8".into() })),
        ("9".into(), Arc::new(Process { id: "9".into() })),
        ("10".into(), Arc::new(Process { id: "10".into() })),
        ("11".into(), Arc::new(Process { id: "11".into() })),
        ("12".into(), Arc::new(Process { id: "12".into() })),
        ("13".into(), Arc::new(Process { id: "13".into() })),
        ("14".into(), Arc::new(Process { id: "14".into() })),
        ("15".into(), Arc::new(Process { id: "15".into() })),
        ("16".into(), Arc::new(Process { id: "16".into() })),
    ]
    .into();
    let (tx, rx) = channel::<(String, Sender<Arc<Process>>)>();

    let _ = thread::spawn(move || {
        while let Ok((id, sender)) = rx.recv() {
            let process = local_processes
                .entry(id.to_string())
                .or_insert_with(|| Arc::new(Process { id }));
            let _ = sender.send(process.clone());
        }
    });
    tx
}

fn run_registry_actor(concurrency: i32) {
    let tx = task_loop_registry();
    let threads: Vec<_> = (1..concurrency)
        .map(|i| {
            let registry_tx = tx.clone();
            thread::spawn(move || {
                let (tx, rx) = channel::<Arc<Process>>();
                let _ = registry_tx.send((i.to_string(), tx));
                let process = rx.recv().unwrap();
                drop(registry_tx);
                assert_eq!(i.to_string(), process.id);
            })
        })
        .collect();
    drop(tx);

    for t in threads {
        t.join().expect("Thread panicked");
    }
}

pub struct ConcurrentRegistry {
    concurrency: u64,
    // system: ActorSystem,
    // host_resolvers: Vec<fn(Pid) -> Option<AnyProcess>>,
    tables: Vec<Mutex<HashMap<String, Process>>>,
    // sequence_id: AtomicU64,
}

impl ConcurrentRegistry
where
    Self: 'static,
{
    pub fn new(concurrency: u64) -> Self {
        let temp: Vec<HashMap<String, Process>> = (0..concurrency)
            .into_iter()
            .map(|_| HashMap::new())
            .collect();
        let tables = [
            ("0".into(), Process { id: "0".into() }),
            ("1".into(), Process { id: "1".into() }),
            ("2".into(), Process { id: "2".into() }),
            ("3".into(), Process { id: "3".into() }),
            ("4".into(), Process { id: "4".into() }),
            ("5".into(), Process { id: "5".into() }),
            ("6".into(), Process { id: "6".into() }),
            ("7".into(), Process { id: "7".into() }),
            ("8".into(), Process { id: "8".into() }),
            ("9".into(), Process { id: "9".into() }),
            ("10".into(), Process { id: "10".into() }),
            ("11".into(), Process { id: "11".into() }),
            ("12".into(), Process { id: "12".into() }),
            ("13".into(), Process { id: "13".into() }),
            ("14".into(), Process { id: "14".into() }),
            ("15".into(), Process { id: "15".into() }),
            ("16".into(), Process { id: "16".into() }),
        ]
        .into_iter()
        .fold(temp, |mut acc, (k, v)| {
            let part = Self::get_partition(&k, concurrency);
            acc[part].insert(k, v);
            acc
        })
        .into_iter()
        .map(Mutex::new)
        .collect();
        Self {
            concurrency,
            tables,
        }
    }

    pub fn get_or_insert(&self, key: String) {
        let table = Self::get_partition(&key, self.concurrency);
        self.tables[table]
            .lock()
            .unwrap()
            .entry(key.to_string())
            .or_insert_with(|| Process {
                id: key.to_string(),
            });
    }

    fn get_partition(key: &String, part_count: u64) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash = s.finish();
        ((hash & 0x7fffffff) % part_count) as usize
    }
}

fn worker_thread_concurrent(registry: Arc<ConcurrentRegistry>, i: u64) {
    let key = i % 16;
    if key != 0 {
        registry.get_or_insert(key.to_string())
    } else {
        let key = uuid::Uuid::new_v4();
        registry.get_or_insert(key.to_string())
    }
}

fn run_concurrent_registry(concurrency: u64) {
    let registry = Arc::new(ConcurrentRegistry::new(concurrency));

    let threads: Vec<_> = (1..concurrency)
        .map(|i| {
            let data = Arc::clone(&registry);
            thread::spawn(move || worker_thread_concurrent(data, i))
        })
        .collect();

    for t in threads {
        t.join().expect("Thread panicked");
    }
    // println!("TOTAL {}", registry.local_processes.lock().unwrap().len())
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("mutex c-1", |b| b.iter(|| run_mutex_registry(1)));
    // c.bench_function("mutex c-4", |b| b.iter(|| run_mutex_registry(4)));
    // c.bench_function("mutex c-8", |b| b.iter(|| run_mutex_registry(8)));
    // c.bench_function("mutex c-16", |b| b.iter(|| run_mutex_registry(16)));
    // c.bench_function("mutex c-32", |b| b.iter(|| run_mutex_registry(32)));
    // c.bench_function("chashmap c-64", |b| b.iter(|| run_chashmap(8)));
    c.bench_function("mutex c-64", |b| b.iter(|| run_mutex_registry(8)));

    // c.bench_function("process loop c-1", |b| b.iter(|| run_registry_actor(1)));
    // c.bench_function("process loop c-4", |b| b.iter(|| run_registry_actor(4)));
    // c.bench_function("process loop c-8", |b| b.iter(|| run_registry_actor(8)));
    // c.bench_function("process loop c-16", |b| b.iter(|| run_registry_actor(16)));
    // c.bench_function("process loop c-32", |b| b.iter(|| run_registry_actor(32)));
    c.bench_function("process loop c-64", |b| b.iter(|| run_registry_actor(8)));

    // c.bench_function("concurrent c-64", |b| b.iter(|| run_concurrent_registry(8)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
