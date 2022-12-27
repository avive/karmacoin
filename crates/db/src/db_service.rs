// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate rocksdb;

// use self::rocksdb::IteratorMode;
use crate::db_utils::{filter_data, parse_value};
use anyhow::{anyhow, Result};

use self::rocksdb::{Direction, IteratorMode};
use bytes::{BufMut, Bytes, BytesMut};
use datetime::Instant;
use rocksdb::DB as rocks;
use rocksdb::{ColumnFamilyDescriptor, Options};
use std::{env, fs};
use xactor::*;

// todo: use DbValue instead of all (Bytes, u64) tuples used below
/// DbValue is binary data and ttl stored in the db by key.
pub struct DbValue {
    pub value: Bytes,
    pub ttl: u64,
}

/// Public db api
impl DatabaseService {
    /// Configure the db
    pub async fn config_db(config: Configure) -> Result<()> {
        let db_service = DatabaseService::from_registry().await?;
        db_service.call(config).await?
    }

    /// Write data by key in a column family
    pub async fn write(item: WriteItem) -> Result<()> {
        let db_service = DatabaseService::from_registry().await?;
        db_service.call(item).await?
    }

    /// Read data by key in a column family
    pub async fn read(item: ReadItem) -> Result<Option<(Bytes, u64)>> {
        let db_service = DatabaseService::from_registry().await?;
        db_service.call(item).await?
    }

    /// Read all items from a column family
    pub async fn read_all_items(item: ReadAllItems) -> Result<ReadAllItemsData> {
        let db_service = DatabaseService::from_registry().await?;
        db_service.call(item).await?
    }

    /// Delete a value by key from a column family
    pub async fn delete(item: DeleteItem) -> Result<()> {
        let db_service = DatabaseService::from_registry().await?;
        db_service.call(item).await?
    }

    /// Drop the db and delete its data folder
    pub async fn drop_db(&mut self) -> Result<()> {
        if self.db.is_none() {
            debug!("db already destroyed");
            return Ok(());
        }

        let curr_dir = env::current_dir()?;
        let path = self.db.as_ref().unwrap().path().to_str().unwrap();
        let os_path = curr_dir.join(path);

        // this should close the db as we are dropping it
        self.db = None;

        info!("deleting db at: {}", os_path.to_str().unwrap());

        // destroy the db
        // let db_opts = get_db_options();
        // rocks::destroy(&db_opts, path).map_err(|e| anyhow!("failed to destroy db: {}", e))?;

        // remove the db directory
        if os_path.exists() {
            let _ = fs::remove_dir_all(os_path);
        }

        Ok(())
    }

    /// private helper method - get the db options
    fn default_options() -> Options {
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_compaction_filter("ttlfilter", filter_data);
        db_opts
    }
}

/// DatabaseService
pub struct DatabaseService {
    db: Option<rocks>,
    drop_on_exit: bool,
    db_name: Option<String>,
}

#[message(result = "Result<()>")]
pub struct Configure {
    pub drop_on_exit: bool,
    pub db_name: String,
    pub col_descriptors: Vec<ColumnFamilyDescriptor>,
}

/// Configure the db
/// todo: pass column families in arguments and not hard-code it
#[async_trait::async_trait]
impl Handler<Configure> for DatabaseService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: Configure) -> Result<()> {
        self.db_name = Some(msg.db_name.clone());
        self.drop_on_exit = msg.drop_on_exit;

        let db_opts = DatabaseService::default_options();
        let db = rocks::open_cf_descriptors(
            &db_opts,
            self.db_name.as_ref().unwrap(),
            msg.col_descriptors,
        )
        .expect(&*format!("failed to open the db at: {}", msg.db_name)); // if db failed to open then just panic
        info!("DatabaseService started with db: {}", msg.db_name);
        self.db = Some(db);

        Ok(())
    }
}

#[async_trait::async_trait]
impl Actor for DatabaseService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("DatabaseService starting...");
        Ok(())
    }

    async fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("db service stopped");
        if !self.drop_on_exit {
            return;
        }

        match self.drop_db().await {
            Ok(()) => debug!("dropped db on service stop"),
            Err(_e) => error!("failed to drop db on service stop"),
        }
    }
}
impl Service for DatabaseService {}
impl Default for DatabaseService {
    fn default() -> Self {
        DatabaseService {
            db: None,
            drop_on_exit: false,
            db_name: None,
        }
    }
}

/////////////  Writing data

// TODO: add support for writing values with a string index - convert string to bytes internally
// as this is going to be very common

/// A db data item - key value pair
/// todo: change everything to use Bytes!
pub struct DataItem {
    pub key: Bytes,
    pub value: Bytes,
}

#[message(result = "Result<()>")]
pub struct WriteItem {
    pub data: DataItem,
    pub cf: &'static str, // column-family
    pub ttl: u64,         // seconds to keep data in the db
}

/// Write a BinaryWriteItem to the store
#[async_trait::async_trait]
impl Handler<WriteItem> for DatabaseService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: WriteItem) -> Result<()> {
        let time_stamp = Instant::now().seconds() as u64 + msg.ttl;
        debug!(
            "PutItem: time_stamp for key {:?} is {:?}",
            msg.data.key, time_stamp
        );

        let mut buf = BytesMut::with_capacity(msg.data.value.len() + 8);
        buf.put_u64(time_stamp);
        buf.put_slice(msg.data.value.to_vec().as_slice());
        let data = buf.freeze();
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow!("db is nil"))?;
        let cf = db_ref
            .cf_handle(&msg.cf)
            .ok_or_else(|| anyhow!("missing db column family: {}", msg.cf))?;

        Ok(db_ref
            .put_cf(cf, msg.data.key, data)
            .map_err(|e| anyhow!("failed to write item: {:?}", e))?)
    }
}

///// Compact db

#[message(result = "Result<()>")]
pub struct Compact;

/// Compact the db
#[async_trait::async_trait]
impl Handler<Compact> for DatabaseService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Compact) -> Result<()> {
        debug!("Initiating db compaction...");
        // ref: https://github.com/facebook/rocksdb/wiki/Manual-Compaction
        // ref: compaction_filter_test in rocksdb lib rust tests
        self.db
            .as_ref()
            .ok_or_else(|| anyhow!("db is nil"))?
            .compact_range(None::<&[u8]>, None::<&[u8]>);

        Ok(())
    }
}

/// Read all (k,v) for a column family
#[message(result = "Result<ReadAllItemsData>")]
#[derive(Clone)]
pub struct ReadAllItems {
    pub from: Option<String>, // when non-empty - return from key (excluding it)
    pub max_results: u32,     // 0 for no limit, otherwise, return up to max_results
    pub cf: &'static str,
}

pub struct ReadAllItemsData {
    pub items: Vec<(Bytes, DbValue)>,
    pub total_keys: u64,
}

/// Read all items stored in a column family
#[async_trait::async_trait]
impl Handler<ReadAllItems> for DatabaseService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ReadAllItems,
    ) -> Result<ReadAllItemsData> {
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow!("db is nil"))?;
        let cf = db_ref
            .cf_handle(&msg.cf)
            .ok_or_else(|| anyhow!("no matching cf"))?;

        let mut iter = match msg.from.as_ref() {
            Some(name) => {
                db_ref.iterator_cf(cf, IteratorMode::From(name.as_bytes(), Direction::Forward))
            }
            None => db_ref.iterator_cf(cf, IteratorMode::Start),
        };

        let mut res: Vec<(Bytes, DbValue)> = vec![];

        if msg.from.is_some() {
            // we skip the first result if from was provided
            let _ = iter.next();
        }

        for item in iter {
            let kv_bytes = item.unwrap();
            let key = Bytes::copy_from_slice(kv_bytes.0.as_ref());
            let (value, ttl) = parse_value(kv_bytes.1.as_ref())?;
            res.push((key, DbValue { value, ttl }));

            if msg.max_results != 0 && res.len() >= msg.max_results as usize {
                break;
            }
        }

        let total_keys = db_ref.property_int_value_cf(cf, "rocksdb.estimate-num-keys")?;

        Ok(ReadAllItemsData {
            items: res,
            total_keys: total_keys.unwrap(),
        })
    }
}

///// Reading data

#[message(result = "Result<Option<(Bytes, u64)>>")]
#[derive(Clone)]
pub struct ReadItem {
    pub key: Bytes,
    pub cf: &'static str,
}

/// Read an item identified by a binary key from the db
#[async_trait::async_trait]
impl Handler<ReadItem> for DatabaseService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: ReadItem,
    ) -> Result<Option<(Bytes, u64)>> {
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow!("db is nil"))?;

        let cf = db_ref
            .cf_handle(msg.cf)
            .ok_or_else(|| anyhow!("no matching cf"))?;

        let data = db_ref
            .get_cf(cf, msg.key.to_vec().as_slice())
            .map_err(|e| anyhow!("error reading from db: {:?}", e))?;

        match data {
            Some(data) => {
                let (data, ttl) = parse_value(data.as_ref())?;
                Ok(Some((data, ttl)))
            }
            None => {
                debug!("Got none for data :-( for key: {:?}", msg.key);
                Ok(None)
            }
        }
    }
}

#[message(result = "Result<()>")]
#[derive(Clone)]
pub struct DeleteItem {
    pub key: Bytes,
    pub cf: &'static str,
}

#[async_trait::async_trait]
/// Read an item identified by a binary key from the db
impl Handler<DeleteItem> for DatabaseService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DeleteItem) -> Result<()> {
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow!("db is nil"))?;
        let cf = db_ref
            .cf_handle(msg.cf)
            .ok_or_else(|| anyhow!("no matching cf: {}", msg.cf))?;

        db_ref
            .delete_cf(cf, msg.key.to_vec().as_slice())
            .map_err(|e| anyhow!(format!("db error: {}", e)))
    }
}

////////////////

//// Destroy the db should close it and delete its files

#[message(result = "Result<()>")]
pub struct Destroy;

/// Delete the db from store. Clients must call Init to work again with the db.
#[async_trait::async_trait]
impl Handler<Destroy> for DatabaseService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Destroy) -> Result<()> {
        self.drop_db().await
    }
}

/////////////////
