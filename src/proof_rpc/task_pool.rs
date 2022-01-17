use std::ops::Deref;
use crate::models::{NewTask, Task, WorkerInfo, NewWorkerInfo};
use crate::models::schema::tasks::dsl::*;
use std::sync::{Mutex};
use diesel::insert_into;
use diesel::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use filecoin_proofs_api::seal::{SealCommitPhase1Output};

use anyhow::{anyhow, Result};
use chrono::Utc;
use log::info;
use uuid::Uuid;
use crate::models::schema::worker_infos::dsl::worker_infos;

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TaskStatus {
    Undefined,
    Init,
    Running,
    Error,
    Completed,
}

pub trait Taskpool {
    fn add(&self, miner_arg: forest_address::Address, worker_id: String, prove_id_arg: String, sector_id_arg: i64,  phase1_output_arg: SealCommitPhase1Output) -> Result<i64>;
    fn fetch(&self, tid: i64) -> Result<Task>;
    fn fetch_undo(&self) -> Result<Vec<Task>>;
    fn fetch_one_todo(&self) -> Result<Task>;
    fn get_status(&self, tid: i64) -> Result<TaskStatus>;
    fn record_error(&self, tid: i64, err_msg: String) -> Option<anyhow::Error>;
    fn record_proof(&self, tid: i64, proof: String) -> Option<anyhow::Error>;
    fn get_worker_id(&self) -> Result<uuid::Uuid>;
}


pub struct TaskpoolImpl {
    conn: Mutex<SqliteConnection>,
}

impl TaskpoolImpl {
    pub fn new(conn: Mutex<SqliteConnection>) -> Self {
        TaskpoolImpl { conn }
    }
}

unsafe impl Send for TaskpoolImpl {}
unsafe impl Sync for TaskpoolImpl {}

impl Taskpool for TaskpoolImpl {
    fn add(&self, miner_arg: forest_address::Address, worker_id_arg: String, prove_id_arg: String, sector_id_arg: i64,  phase1_output_arg: SealCommitPhase1Output,) -> Result<i64> {
        let miner_noprefix = &miner_arg.to_string()[1..];
        let new_task = NewTask{
            miner: miner_noprefix.to_string(),
            worker_id: worker_id_arg,
            prove_id: prove_id_arg,
            sector_id: sector_id_arg,
            phase1_output: serde_json::to_string(&phase1_output_arg).unwrap(),
            task_type:0,
            status:TaskStatus::Init.into(),
            create_at: Utc::now().timestamp(),
        };

        let lock = self.conn.lock().unwrap();
        let result = insert_into(tasks).values(&new_task).execute(lock.deref());

        match result {
            Ok(val) => Ok(val as i64),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn fetch(&self, tid: i64) -> Result<Task> {
        let lock = self.conn.lock().unwrap();
        let result = tasks.find(tid).first(lock.deref());
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn get_status(&self, tid: i64) -> Result<TaskStatus> {
        let lock = self.conn.lock().unwrap();
        let result: QueryResult::<i32> = tasks.select(status).filter(id.eq(tid)).get_result(lock.deref());
        match result {
            Ok(val) => Ok(TaskStatus::try_from(val).unwrap()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn fetch_undo(&self) -> Result<Vec<Task>> {
        let lock = self.conn.lock().unwrap();
        let result = tasks.filter(status.eq::<i32>(TaskStatus::Init.into()))
            .load(lock.deref());
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn fetch_one_todo(&self) -> Result<Task> {
        let lock = self.conn.lock().unwrap();
        let predicate = status.eq::<i32>(TaskStatus::Init.into());
        let result: Task = tasks.filter(&predicate).first(lock.deref()).unwrap();
        let update_result = diesel::update(tasks.filter(id.eq(result.id))).set((
            status.eq::<i32>(TaskStatus::Init.into()),
            start_at.eq(Utc::now().timestamp()),
        )).execute(lock.deref());
        match update_result {
            Ok(_) => Ok(result),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn record_error(&self, _tid: i64, err_msg_str: String) -> Option<anyhow::Error> {
        let lock = self.conn.lock().unwrap();
        let update_result = diesel::update(tasks.filter(id.eq(_tid))).set((
                                                           status.eq::<i32>(TaskStatus::Error.into()),
                                                           error_msg.eq(err_msg_str),
                                                           )).execute(lock.deref());
        match update_result {
            Ok(_) => Option::None,
            Err(e) => Some(anyhow!(e.to_string())),
        }
    }

    fn record_proof(&self, _tid: i64, proof_str: String) -> Option<anyhow::Error> {
        let lock = self.conn.lock().unwrap();
        let update_result = diesel::update(tasks.filter(id.eq(_tid))).set((
            status.eq::<i32>(TaskStatus::Error.into()),
            proof.eq(proof_str),
        )).execute(lock.deref());
        match update_result {
            Ok(_) => Option::None,
            Err(e) => Some(anyhow!(e.to_string())),
        }
    }

    fn get_worker_id(&self) -> Result<uuid::Uuid> {
        let lock = self.conn.lock().unwrap();
        let row_count: i64 =  worker_infos.count().get_result(lock.deref()).unwrap();
        if row_count > 0 {
           let uid =  uuid::Uuid::new_v4();
            let new_worker_info = NewWorkerInfo{
                worker_id: uid.to_string(),
            };

            let lock = self.conn.lock().unwrap();
            let result = insert_into(worker_infos).values(&new_worker_info).execute(lock.deref()).unwrap();
            info!("create worker id {}", result);
           Ok(uid)
        } else {
           let worker_info: WorkerInfo =  worker_infos.first(lock.deref()).unwrap();
            let load_worker_id = Uuid::parse_str(worker_info.worker_id.as_str()).unwrap();
            info!("load worker id {}", load_worker_id.to_string());
            Ok(load_worker_id)
        }
    }
}




#[cfg(test)]
mod tests{

    #[test]
    pub fn test_status() {

    }
}