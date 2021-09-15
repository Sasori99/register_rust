use rocksdb::DB;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub age: i32,
    pub email: String,
    pub active: bool
}

pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &str, v: &User) -> bool;
    fn find(&self, k: &str) -> Option<User>;
    fn delete(&self, k: &str) -> bool;
    fn find_by_username(&self, k: &str) -> Option<User>;
    fn find_by_email(&self, k: &str) -> Option<User>;
}

#[derive(Clone)]
pub struct RocksDB {
    db: Arc<DB>
}

impl KVStore for RocksDB {
    fn init(file_path: &str) -> Self {
        RocksDB { db: Arc::new(DB::open_default(file_path).unwrap()) }
    }

    fn save(&self, k: &str, v: &User) -> bool {
        let bytes = bincode::serialize(&v).unwrap();
        self.db.put(k.as_bytes(), bytes).is_ok()
    }

    fn find(&self, k: &str) -> Option<User> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result: User = bincode::deserialize(&v).unwrap();
                println!("Finding '{}' return '{:?}'", k, result);
                Some(result)
            }
            Ok(None) => {
                println!("Finding '{}' return None", k);
                None
            }
            Err(e) => {
                println!("Error retrieving value for {}: {}", k, e);
                None
            }
        }
    }

    fn delete(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }

    fn find_by_username(&self, k: &str) -> Option<User> {
        let mut iter = self.db.raw_iterator();
        iter.seek_to_first();
        let mut result: Option<User> = None;
        while iter.valid() {
            let user: User = bincode::deserialize(iter.value().unwrap()).unwrap();
            if user.username == k {
                result = Some(user);
                break;
            }
            iter.next();
        };
        result
    }

    fn find_by_email(&self, k: &str) -> Option<User> {
        let mut iter = self.db.raw_iterator();
        iter.seek_to_first();
        let mut result: Option<User> = None;
        while iter.valid() {
            let user: User = bincode::deserialize(iter.value().unwrap()).unwrap();
            if user.email == k {
                result = Some(user);
                break;
            }
            iter.next();
        };
        result
    }
}