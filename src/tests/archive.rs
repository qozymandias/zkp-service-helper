use super::*;
use crate::run_archive_test;

#[tokio::test]
async fn test_query_archive_summary() {
    run_archive_test!(ZkWasmServiceHelper::query_archive_summary);
}

#[tokio::test]
async fn test_query_archive_task_volume_list() {
    run_archive_test!(ZkWasmServiceHelper::query_archive_task_volume_list, None, None);
}

#[tokio::test]
async fn test_query_archive_auto_submit_task_volume_list() {
    run_archive_test!(ZkWasmServiceHelper::query_archive_auto_submit_task_volume_list, None, None);
}

#[tokio::test]
async fn test_query_archived_task() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archived_task,
        CONFIG.archive.archived_task_id.clone()
    );
}

#[tokio::test]
async fn test_query_archived_auto_submit_networks_by_task_id() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archived_auto_submit_networks_by_task_id,
        CONFIG.archive.archived_task_id.clone()
    );
}

#[tokio::test]
async fn test_query_archived_auto_submit_info_by_task_id() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archived_auto_submit_info_by_task_id,
        CONFIG.archive.archived_task_id.clone(),
        CONFIG.details.chain_id
    );
}

#[tokio::test]
async fn test_query_archived_auto_submit_info_by_archive_id() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archived_auto_submit_info_by_archive_id,
        CONFIG.archive.id.clone(),
        CONFIG.details.chain_id
    );
}

#[tokio::test]
async fn test_query_archive_server_config() {
    run_archive_test!(ZkWasmServiceHelper::query_archive_server_config);
}

#[tokio::test]
async fn test_query_archive_task_volume() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archive_task_volume,
        CONFIG.archive.archive_volume_name.clone(),
        None,
        None
    );
}

#[tokio::test]
async fn test_query_archive_auto_submit_volume() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archive_auto_submit_volume,
        CONFIG.archive.archive_auto_submit_volume_name.clone(),
        None,
        None
    );
}

#[tokio::test]
async fn test_query_archive() {
    run_archive_test!(
        ZkWasmServiceHelper::query_archive,
        Some(CONFIG.archive.archived_task_id.clone()),
        None,
        None,
        None,
        None,
        None
    );
}
