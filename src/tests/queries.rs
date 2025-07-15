use super::*;
use crate::run_test;

#[tokio::test]
async fn test_query_image() {
    let res = run_test!(ZkWasmServiceHelper::query_image, CONFIG.query.md5.clone());
    assert_eq!(res.md5, CONFIG.query.md5);
}

#[tokio::test]
async fn test_query_image_binary() {
    let res = run_test!(ZkWasmServiceHelper::query_image_binary, CONFIG.query.md5.clone());
    assert!(!res.is_empty())
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
    let res = run_test!(ZkWasmServiceHelper::query_user_subscription, addr.clone()).expect("Should exist in db");
    assert_eq!(addr, res.subscriber_address);
}

#[tokio::test]
async fn test_query_tx_history() {
    let res = run_test!(ZkWasmServiceHelper::query_tx_history, CONFIG.user_address());
    assert!(!res.data.is_empty());
    assert_eq!(res.total as usize, res.data.len());
}

#[tokio::test]
async fn test_query_deposit_history() {
    let res = run_test!(ZkWasmServiceHelper::query_deposit_history, CONFIG.user_address());
    assert!(!res.data.is_empty());
    assert_eq!(res.total as usize, res.data.len());
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
    let addr = CONFIG.query.node_address.clone();
    let res = run_test!(ZkWasmServiceHelper::query_node_statistics, Some(addr.clone()), None, None);
    assert!(!res.data.is_empty());
    assert_eq!(res.total as usize, res.data.len());
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
        now,
        then,
    );
    let fst_dt: chrono::DateTime<chrono::Utc> = res.fst_ts.and_then(|s| s.parse().ok()).unwrap();
    let lst_dt: chrono::DateTime<chrono::Utc> = res.lst_ts.and_then(|s| s.parse().ok()).unwrap();
    let fst_st: std::time::SystemTime = fst_dt.into();
    let lst_st: std::time::SystemTime = lst_dt.into();
    assert!(now > fst_st);
    assert!(then < lst_st);
}

#[cfg(test)]
mod task {
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(id, res.data[0]._id.oid);
    }

    #[tokio::test]
    async fn test_query_tasks_by_tasktype() {
        let task_type = serde_json::to_string(&crate::interface::TaskType::Prove).expect("Should convert to string");
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            None,
            None,
            Some(task_type.clone()),
            None,
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(task_type, res.data[0].task_type);
    }

    #[tokio::test]
    async fn test_query_tasks_by_status() {
        let status = serde_json::to_string(&crate::interface::TaskStatus::Done).expect("Should convert to string");
        let res = run_test!(
            ZkWasmServiceHelper::query_tasks,
            None,
            None,
            None,
            None,
            Some(status.clone()),
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(
            status,
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(id, res.data[0]._id.oid);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_tasktype() {
        let task_type = serde_json::to_string(&crate::interface::TaskType::Prove).expect("Should convert to string");
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            None,
            None,
            Some(task_type.clone()),
            None,
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(task_type, res.data[0].task_type);
    }

    #[tokio::test]
    async fn test_query_concise_tasks_by_status() {
        let status = serde_json::to_string(&crate::interface::TaskStatus::Done).expect("Should convert to string");
        let res = run_test!(
            ZkWasmServiceHelper::query_concise_tasks,
            None,
            None,
            None,
            None,
            Some(status.clone()),
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(
            status,
            serde_json::to_string(&res.data[0].status).expect("Should convert to string")
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
    async fn test_query_auto_submit_proofs_by_id() {
        let id = CONFIG.auto_submit.id.clone();
        let res = run_test!(
            ZkWasmServiceHelper::query_auto_submit_proofs,
            Some(id.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(id, res.data[0]._id.as_ref().expect("Should have id").oid);
    }

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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(
            serde_json::to_string(&crate::interface::AutoSubmitProofStatus::Pending).unwrap(),
            serde_json::to_string(&res.data[0].status).unwrap()
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert!(res.data[0].task_ids.contains(&id));
    }

    #[tokio::test]
    async fn test_query_round1_info_by_status() {
        let res = run_test!(
            ZkWasmServiceHelper::query_round1_info,
            None,
            None,
            Some(crate::interface::Round1Status::Pending),
            None,
            None,
            None,
            None,
        );
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(
            serde_json::to_string(&crate::interface::Round1Status::Pending).unwrap(),
            serde_json::to_string(&res.data[0].status).unwrap()
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(
            serde_json::to_string(&crate::interface::Round2Status::ProofRegistered).unwrap(),
            serde_json::to_string(&res.data[0].status).unwrap()
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
        assert!(!res.data.is_empty());
        assert_eq!(res.total as usize, res.data.len());
        assert_eq!(chain_id, res.data[0].auto_submit_network_chain_id);
    }
}

#[cfg(test)]
mod archive {
    use super::*;

    #[tokio::test]
    async fn test_query_archive_summary() {
        run_test!(ZkWasmServiceHelper::query_archive_summary);
    }

    #[tokio::test]
    async fn test_query_archive_task_volume_list() {
        run_test!(ZkWasmServiceHelper::query_archive_task_volume_list, None, None);
    }

    #[tokio::test]
    async fn test_query_archive_auto_submit_task_volume_list() {
        run_test!(ZkWasmServiceHelper::query_archive_auto_submit_task_volume_list, None, None);
    }

    #[tokio::test]
    async fn test_query_archived_task() {
        run_test!(
            ZkWasmServiceHelper::query_archived_task,
            CONFIG.archive.archived_task_id.clone()
        );
    }

    #[tokio::test]
    async fn test_query_archived_auto_submit_networks_by_task_id() {
        run_test!(
            ZkWasmServiceHelper::query_archived_auto_submit_networks_by_task_id,
            CONFIG.archive.archived_task_id.clone()
        );
    }

    #[tokio::test]
    async fn test_query_archived_auto_submit_info_by_task_id() {
        run_test!(
            ZkWasmServiceHelper::query_archived_auto_submit_info_by_task_id,
            CONFIG.archive.archived_task_id.clone(),
            CONFIG.details.chain_id
        );
    }

    #[tokio::test]
    async fn test_query_archived_auto_submit_info_by_archive_id() {
        run_test!(
            ZkWasmServiceHelper::query_archived_auto_submit_info_by_archive_id,
            CONFIG.archive.id.clone(),
            CONFIG.details.chain_id
        );
    }

    #[tokio::test]
    async fn test_query_archive_server_config() {
        run_test!(ZkWasmServiceHelper::query_archive_server_config);
    }

    #[tokio::test]
    async fn test_query_archive_task_volume() {
        run_test!(
            ZkWasmServiceHelper::query_archive_task_volume,
            CONFIG.archive.archive_volume_name.clone(),
            None,
            None
        );
    }

    #[tokio::test]
    async fn test_query_archive_auto_submit_volume() {
        run_test!(
            ZkWasmServiceHelper::query_archive_auto_submit_volume,
            CONFIG.archive.archive_volume_name.clone(),
            None,
            None
        );
    }

    #[tokio::test]
    async fn test_query_archive() {
        run_test!(
            ZkWasmServiceHelper::query_archive,
            Some(CONFIG.archive.archived_task_id.clone()),
            None,
            None,
            None,
            None,
            None
        );
    }
}
