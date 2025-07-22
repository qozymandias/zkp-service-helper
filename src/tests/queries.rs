use serde::Serialize;

use super::*;
use crate::run_test;

fn check_paginated_res<T: Serialize>(res: &crate::interface::PaginationResult<Vec<T>>) {
    assert!(!res.data.is_empty());
    assert!(usize::try_from(res.total).expect("Should convert") >= res.data.len());
}

#[tokio::test]
async fn test_query_image() {
    let res = run_test!(ZkWasmServiceHelper::query_image, CONFIG.query.md5.clone()).expect("Image should exist in DB");
    assert_eq!(res.md5, CONFIG.query.md5);
}

#[tokio::test]
async fn test_query_image_binary() {
    let res = run_test!(ZkWasmServiceHelper::query_image_binary, CONFIG.query.md5.clone());
    assert!(!res.is_empty());
}

#[tokio::test]
async fn test_query_user() {
    let addr = CONFIG.user_address();
    let res = run_test!(ZkWasmServiceHelper::query_user, addr.clone()).expect("Should exist in db");
    assert_eq!(addr, res.user_address);
}

#[tokio::test]
async fn test_query_user_subscription() {
    let addr = CONFIG.user_address();
    let _res = run_test!(ZkWasmServiceHelper::query_user_subscription, addr.clone());
    #[cfg(feature = "pedantic-tests")]
    {
        let res = _res.expect("Should exist in db");
        assert_eq!(addr, res.subscriber_address);
    }
}

#[tokio::test]
async fn test_query_tx_history() {
    let _res = run_test!(ZkWasmServiceHelper::query_tx_history, CONFIG.user_address());
    #[cfg(feature = "pedantic-tests")]
    check_paginated_res(&_res);
}

#[tokio::test]
async fn test_query_deposit_history() {
    let _res = run_test!(ZkWasmServiceHelper::query_deposit_history, CONFIG.user_address());
    #[cfg(feature = "pedantic-tests")]
    check_paginated_res(&_res);
}

#[tokio::test]
async fn test_query_config() {
    run_test!(ZkWasmServiceHelper::query_config);
}

#[tokio::test]
async fn test_query_statistics() {
    run_test!(ZkWasmServiceHelper::query_statistics);
}

#[tokio::test]
async fn test_query_node_statistics() {
    let res = run_test!(ZkWasmServiceHelper::query_node_statistics, None, Some(0), Some(5));
    check_paginated_res(&res);
}

#[tokio::test]
async fn test_query_node_statistics_by_address() {
    let addr = CONFIG.query.node_address.clone();
    let res = run_test!(ZkWasmServiceHelper::query_node_statistics, Some(addr.clone()), None, None);
    check_paginated_res(&res);
    assert_eq!(addr, res.data[0].address);
}

#[tokio::test]
async fn test_query_prover_node_summary() {
    run_test!(ZkWasmServiceHelper::query_prover_node_summary);
}

#[tokio::test]
async fn test_query_online_node_summary() {
    run_test!(ZkWasmServiceHelper::query_online_node_summary);
}

#[tokio::test]
async fn test_query_logs() {
    let res = run_test!(
        ZkWasmServiceHelper::query_logs,
        CONFIG.query.task_id.clone(),
        CONFIG.user_address(),
        CONFIG.details.private_key.clone()
    );
    assert!(!res.is_empty());
}

#[tokio::test]
async fn test_query_estimated_proof_fee() {
    let res = run_test!(
        ZkWasmServiceHelper::query_estimated_proof_fee,
        CONFIG.user_address().clone(),
        CONFIG.query.md5.clone(),
        crate::interface::ProofSubmitMode::Auto,
    );
    assert!(res.min.is_some());
    assert!(res.max.is_some());
}

#[tokio::test]
async fn test_query_prover_node_timerange_stats() {
    let now = std::time::SystemTime::now();
    let then = now - std::time::Duration::from_secs(4 * 7 * 24 * 60 * 60);

    let res = run_test!(
        ZkWasmServiceHelper::query_prover_node_timerange_stats,
        CONFIG.query.node_address.clone(),
        then,
        now,
    );
    let fst_dt: chrono::DateTime<chrono::Utc> = res.fst_ts.and_then(|s| s.parse().ok()).expect("Should convert");
    let lst_dt: chrono::DateTime<chrono::Utc> = res.lst_ts.and_then(|s| s.parse().ok()).expect("Should convert");
    let fst_st: std::time::SystemTime = fst_dt.into();
    let lst_st: std::time::SystemTime = lst_dt.into();
    assert!(now > fst_st);
    assert!(then < lst_st);
}

#[cfg(test)]
mod task {
    use crate::interface::TaskStatus;
    use crate::interface::TaskType;

    use super::*;

    #[tokio::test]
    async fn test_query_tasks_by_address() {
        let addr = CONFIG.user_address();
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            Some(addr.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(addr, res.data[0].user_address);
    }

    #[tokio::test]
    async fn test_query_tasks_by_md5() {
        let md5 = CONFIG.query.md5.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            Some(md5.clone()),
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(md5, res.data[0].md5);
    }

    #[tokio::test]
    async fn test_query_tasks_by_task_id() {
        let id = CONFIG.query.task_id.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            None,
            Some(id.clone()),
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(id, res.data[0]._id.oid);
    }

    #[tokio::test]
    async fn test_query_tasks_by_tasktype() {
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            None,
            None,
            Some(TaskType::Prove),
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&TaskType::Prove).expect("Should convert"),
            serde_json::to_string(&res.data[0].task_type).expect("Should convert")
        );
    }

    #[tokio::test]
    async fn test_query_tasks_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            None,
            None,
            None,
            Some(TaskStatus::Done),
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&TaskStatus::Done).expect("Should convert"),
            serde_json::to_string(&res.data[0].status).expect("Should convert to string")
        );
    }

    #[tokio::test]
    async fn test_query_tasks_from_ids() {
        let ids = vec![CONFIG.query.task_id.clone(); 10];
        let res = run_test!(ZkWasmServiceHelper::query_tasks_from_ids, ids.clone());
        assert!(!res.is_empty());
        assert_eq!(res.len(), ids.len());
        assert_eq!(ids[0], res[0]._id.oid);
    }

    #[tokio::test]
    async fn test_query_tasks_from_id() {
        let id = CONFIG.query.task_id.clone();
        let res = run_test!(ZkWasmServiceHelper::query_tasks_from_id, id.clone()).expect("Task should exist");
        assert_eq!(id, res._id.oid);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_address() {
        let addr = CONFIG.user_address();
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            Some(addr.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(addr, res.data[0].user_address);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_md5() {
        let md5 = CONFIG.query.md5.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            Some(md5.clone()),
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(md5, res.data[0].md5);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_task_id() {
        let id = CONFIG.query.task_id.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            None,
            Some(id.clone()),
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(id, res.data[0]._id.oid);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_tasktype() {
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            None,
            None,
            Some(TaskType::Prove),
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&TaskType::Prove).expect("Should convert"),
            serde_json::to_string(&res.data[0].task_type).expect("Should convert")
        );
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            None,
            None,
            None,
            Some(TaskStatus::Done),
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&TaskStatus::Done).expect("Should convert"),
            serde_json::to_string(&res.data[0].status).expect("Should convert")
        );
    }

    #[tokio::test]
    async fn test_get_task_external_host_table() {
        let res = run_test!(ZkWasmServiceHelper::get_task_external_host_table, CONFIG.query.task_id.clone(),);
        assert!(!res.external_host_table.is_empty());
    }
}

#[cfg(test)]
mod auto_submit {
    use super::*;

    #[tokio::test]
    async fn test_query_auto_submit_proofs_by_task_id() {
        let id = CONFIG.auto_submit.task_id_in_auto_submit_batch.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_auto_submit_proofs,
            None,
            Some(id.clone()),
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(id, res.data[0].task_id);
    }

    #[tokio::test]
    async fn test_query_auto_submit_proofs_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_auto_submit_proofs,
            None,
            None,
            Some(crate::interface::AutoSubmitProofStatus::Batched),
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&crate::interface::AutoSubmitProofStatus::Batched).expect("Should convert"),
            serde_json::to_string(&res.data[0].status).expect("Should convert")
        );
    }

    #[tokio::test]
    async fn test_query_auto_submit_proofs_by_chain_id() {
        let chain_id = CONFIG.details.chain_id;
        let res = run_test!(
            ZkWasmServiceHelper::query_auto_submit_proofs,
            None,
            None,
            None,
            None,
            Some(chain_id),
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
    }

    #[tokio::test]
    async fn test_query_round1_info_by_id() {
        let id = CONFIG.auto_submit.round1_id.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            Some(id.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(id, res.data[0]._id.as_ref().expect("Should have id").oid);
    }

    #[tokio::test]
    async fn test_query_round1_info_by_task_id() {
        let id = CONFIG.auto_submit.task_id_in_auto_submit_batch.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            None,
            Some(id.clone()),
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert!(res.data[0].task_ids.contains(&id));
    }

    #[tokio::test]
    async fn test_query_round1_info_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            None,
            None,
            Some(crate::interface::Round1Status::Batched),
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&crate::interface::Round1Status::Batched).expect("Should convert"),
            serde_json::to_string(&res.data[0].status).expect("Should convert"),
        );
    }

    #[tokio::test]
    async fn test_query_round1_info_by_chain_id() {
        let chain_id = CONFIG.details.chain_id;
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            None,
            None,
            None,
            None,
            Some(chain_id),
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
    }

    #[tokio::test]
    async fn test_query_round2_info_by_id() {
        let id = CONFIG.auto_submit.round2_id.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_round2_info,
            Some(id.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(id, res.data[0]._id.as_ref().expect("Should have id").oid);
    }

    #[tokio::test]
    async fn test_query_round2_info_by_task_id() {
        let id = CONFIG.auto_submit.task_id_in_auto_submit_batch.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_round2_info,
            None,
            None,
            Some(id.clone()),
            None,
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert!(res.data[0].task_ids.contains(&id));
    }

    #[tokio::test]
    async fn test_query_round2_info_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_round2_info,
            None,
            None,
            None,
            Some(crate::interface::Round2Status::ProofRegistered),
            None,
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(
            serde_json::to_string(&crate::interface::Round2Status::ProofRegistered).expect("Should convert"),
            serde_json::to_string(&res.data[0].status).expect("Should convert"),
        );
    }

    #[tokio::test]
    async fn test_query_round2_info_by_chain_id() {
        let chain_id = CONFIG.details.chain_id;
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            None,
            None,
            None,
            None,
            Some(chain_id),
            None,
            None,
        );
        check_paginated_res(&res);
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
    }
}
