

mod simplekv;
mod btree;

pub type KvStore<V> = simplekv::KvStore<V>;
pub type Btree = btree::BTree;