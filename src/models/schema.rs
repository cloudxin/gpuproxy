table! {
    tasks (id) {
        id -> Text,
        miner -> Text,
        prove_id -> Text,
        sector_id -> BigInt,
        phase1_output -> Text,
        proof -> Text,
        worker_id -> Text,
        task_type -> Integer,
        error_msg -> Text,
        status -> Integer,
        create_at -> BigInt,
        start_at  -> BigInt,
        complete_at -> BigInt,
    }
}


table! {
    worker_infos (id) {
        id -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    worker_infos,
);
