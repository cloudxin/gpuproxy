use gpuproxy::config::*;
use gpuproxy::proof_rpc::*;
use gpuproxy::models::*;
use gpuproxy::models::migrations::*;
use crate::worker::Worker;
use crate::db_ops::*;
use log::*;
use simplelog::*;
use clap::{App, AppSettings, Arg};
use std::sync::Arc;
use std::sync::{Mutex};

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let app_m = App::new("gpuproxy-worker")
        .version("0.0.1")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("run")
                .setting(AppSettings::ArgRequiredElseHelp)
                .about("worker for execute task")
                .args(&[
                    Arg::new("gpuproxy-url")
                        .long("gpuproxy-url")
                        .env("C2PROXY_LISTEN_URL")
                        .default_value("http://127.0.0.1:8888")
                        .help("specify url for connect gpuproxy for get task to excute"),
                    Arg::new("db-dsn")
                        .long("db-dsn")
                        .env("C2PROXY_DSN")
                        .default_value("gpuproxy-worker.db")
                        .help("specify sqlite path to store task"),
                    Arg::new("max-c2")
                        .long("max-c2")
                        .env("C2PROXY_MAX_C2")
                        .default_value("1")
                        .help("number of c2 task to run parallelly"),
                ]),
        )
        .get_matches();

    match app_m.subcommand() {
        Some(("run", ref sub_m)) => {
            let url: String = sub_m.value_of_t("gpuproxy-url").unwrap_or_else(|e| e.exit());
            let max_c2: usize = sub_m.value_of_t("max-c2").unwrap_or_else(|e| e.exit());
            let db_dsn: String = sub_m.value_of_t("db-dsn").unwrap_or_else(|e| e.exit());
            let cfg = ClientConfig::new(url, db_dsn, max_c2,"db".to_string(),"".to_string());

            let db_conn = establish_connection(cfg.db_dsn.as_str());
            run_db_migrations(&db_conn).expect("migrations error");
            let task_pool = db_ops::TaskpoolImpl::new(Mutex::new(db_conn));
            let worker_id = task_pool.get_worker_id().unwrap();
            
            let worker_api =  Arc::new(proof::get_proxy_api(cfg.url).unwrap());
            let worker = worker::LocalWorker::new(cfg.max_c2, worker_id.to_string(), worker_api.clone(), worker_api);
            let join_handle = worker.process_tasks();
            info!("ready for local worker address worker_id {}", worker_id);
            join_handle.join().unwrap();
        } // run was used
        _ => {} // Either no subcommand or one not tested for...
    }
}