mod db_get;
pub use db_get::*;

mod db_set;
pub use db_set::*;

mod http_fetch;
use std::{marker::PhantomData, collections::HashMap};

use actix::{prelude::*, SystemRegistry};
pub use db_get::DatabaseGet;
pub use db_set::DatabaseSet;
use futures::{executor, lock::Mutex};
// use once_cell::sync::Lazy;
pub use http_fetch::HttpFetch;
use rusqlite::params;
use seda_chain_adapters::MainChainAdapterTrait;
use tokio_rusqlite::Connection;

use crate::RuntimeAdapterError;

mod chain_change;
pub use chain_change::ChainChange;
mod chain_view;
pub use chain_view::ChainView;

pub struct Host<MC: MainChainAdapterTrait> {
    db_conn: Connection,
    _pd: PhantomData<MC>
}

impl<MC: MainChainAdapterTrait> Default for Host<MC> {
    fn default() -> Self {
        println!("HELLO FROM HOST DEFAULT");
        executor::block_on(async move {
            let db_conn = Connection::open("./seda_db.db3").await.expect("Couldn't open db conn");

            db_conn
                .call(|db_conn| {
                    db_conn
                        .execute(
                            "CREATE TABLE IF NOT EXISTS data (
                                key TEXT,
                                value TEXT NOT NULL
                            )",
                            params![],
                        )
                        .expect("couldn't create db table");

                    Ok::<_, RuntimeAdapterError>(())
                })
                .await
                .expect("Couldn't execute db call");

            Host { db_conn, _pd: PhantomData}
        })
    }
}

impl<MC: MainChainAdapterTrait>  Actor for Host<MC> {
    type Context = Context<Self>;
    // type Context = NodeContext<Self, MC>;
    
}

impl<MC: MainChainAdapterTrait>  actix::Supervised for Host<MC> {}

impl<MC>  SystemService for Host<MC> 
    where
    MC: MainChainAdapterTrait, 

{}



// impl<MC>  SystemService for Host<MC> 
//     where
//     MC: MainChainAdapterTrait, 
// {
//     // fn start_service(wrk: &ArbiterHandle) -> Addr<Self> {
//     //     Supervisor::start_in_arbiter(wrk, |ctx| {
//     //         let mut act = Self::default();
//     //         act.service_started(ctx);
//     //         act
//     //     })
//     // }

//     // fn service_started(&mut self, ctx: &mut Context<Self>) {}

//     // fn from_registry() -> Addr<Self> {
//     //     let sys = System::current();

//     //     let mut sreg = SREG.lock();
//     //     let reg = sreg
//     //         .entry(sys.id())
//     //         .or_insert_with(|| SystemRegistry::new(sys.arbiter().clone()));

//     //     if let Some(addr) = reg.registry.get(&std::any::TypeId::of::<Self>()) {
//     //         if let Some(addr) = addr.downcast_ref::<Addr<Self>>() {
//     //             return addr.clone();
//     //         }
//     //     }

//     //     let addr = Self::start_service(System::current().arbiter());
//     //     reg.registry
//     //         .insert(std::any::TypeId::of::<Self>(), Box::new(addr.clone()));
//     //     addr
//     // }


//     // fn from_registry() -> Addr<Self> {
//     //     let sys = System::current();

//     //     let mut sreg = SREG.lock();
//     //     let reg = sreg
//     //         .entry(sys.id())
//     //         .or_insert_with(|| actix::SystemRegistry::new(sys.arbiter().clone()));

//     //     if let Some(addr) = reg.registry.get(&std::any::TypeId::of::<Self>()) {
//     //         if let Some(addr) = addr.downcast_ref::<Addr<Self>>() {
//     //             return addr.clone();
//     //         }
//     //     }

//     //     let addr = Self::start_service(System::current().arbiter());
//     //     reg.registry
//     //         .insert(std::any::TypeId::of::<Self>(), Box::new(addr.clone()));
//     //     addr
//     // }

//     // fn service_started(&mut self, ctx: &mut Context<Self>) {}

//     // fn from_registry() -> Addr<Self> {
//     //     let sys = System::current();

//     //     let mut sreg = SREG.lock();
//     //     let reg = sreg
//     //         .entry(sys.id())
//     //         .or_insert_with(|| actix::SystemRegistry::new(sys.arbiter().clone()));

//     //     if let Some(addr) = reg.registry.get(&std::any::TypeId::of::<Self>()) {
//     //         if let Some(addr) = addr.downcast_ref::<Addr<Self>>() {
//     //             return addr.clone();
//     //         }
//     //     }

//     //     let addr = Self::start_service(System::current().arbiter());
//     //     reg.registry
//     //         .insert(std::any::TypeId::of::<Self>(), Box::new(addr.clone()));
//     //     addr
//     // }
// }


// static SREG: Lazy<Mutex<HashMap<usize, SystemRegistry>>> =
//     Lazy::new(|| Mutex::new(HashMap::new()));




// impl<MC> Host<MC>
// where MC: MainChainAdapterTrait {
    
//     pub async fn new() -> Self {
//         Self::default()
//     }
// }