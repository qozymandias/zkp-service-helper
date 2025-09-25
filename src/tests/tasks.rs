use super::*;
use crate::run_test;

fn read_wasm(image: String) -> anyhow::Result<(Vec<u8>, String)> {
    use std::io::Read;

    let mut file = std::fs::File::open(image)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let md5 = format!("{:X}", md5::compute(&buffer)).to_uppercase();
    Ok((buffer, md5))
}

#[cfg(test)]
mod payments {
    use super::*;
    use crate::interface::SubscriptionDuration;
    use crate::interface::SubscriptionType;

    #[tokio::test]
    #[ignore]
    async fn test_add_payment() {
        // TODO
        let txhash = String::new();
        run_test!(ZkWasmServiceHelper::add_payment, txhash);
    }

    #[tokio::test]
    #[ignore]
    async fn test_add_subscription() {
        // TODO
        let subscriber_address = String::new();
        let subscription_type = SubscriptionType::Basic;
        let duration = SubscriptionDuration {
            base_duration: crate::interface::BaseSubscriptionDuration::Month,
            multiplier: 1,
        };
        let payment_hash = String::new();
        run_test!(
            ZkWasmServiceHelper::add_subscription,
            subscriber_address,
            subscription_type,
            duration,
            payment_hash,
        );
    }
}

async fn wait_for_done_task(id: &str) {
    loop {
        let res = ZKH
            .query_task_from_id(id.to_string())
            .await
            .ok()
            .flatten()
            .expect("Should be able to query task");
        if let crate::interface::TaskStatus::Done = res.status {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run_setup_image() -> anyhow::Result<(String, String)> {
    let (buffer, md5) = read_wasm(CONFIG.tasks.image.clone())?;

    let res = run_test!(
        ZkWasmServiceHelper::setup_image,
        md5.clone(),
        buffer,
        md5.clone(),
        CONFIG.user_address().clone(),
        format!("ZKP CLI test image {md5}"),
        String::new(),
        22,
        crate::interface::ProvePaymentSrc::Default,
        vec![CONFIG.details.chain_id],
        None,
        None,
        crate::interface::InitialContext::Without,
        CONFIG.details.private_key.clone(),
    );
    Ok((res.id, res.md5))
}

async fn run_prove_image(md5: String) -> anyhow::Result<String> {
    let res = run_test!(
        ZkWasmServiceHelper::add_prove,
        CONFIG.user_address().clone(),
        md5,
        vec![],
        vec![],
        crate::interface::ProofSubmitMode::Manual,
        crate::interface::CustomContext::Without,
        CONFIG.details.private_key.clone(),
    );
    Ok(res.id)
}

async fn run_reset_image(md5: String) -> anyhow::Result<String> {
    let res = run_test!(
        ZkWasmServiceHelper::add_reset,
        md5,
        22,
        CONFIG.user_address().clone(),
        crate::interface::ProvePaymentSrc::Default,
        vec![CONFIG.details.chain_id],
        None,
        crate::interface::ResetContext::Without,
        CONFIG.details.private_key.clone(),
    );
    Ok(res.id)
}

async fn run_modify_image(md5: String) -> anyhow::Result<()> {
    run_test!(
        ZkWasmServiceHelper::modify_image,
        md5.clone(),
        CONFIG.user_address().clone(),
        format!("ZKP CLI test image {md5} -- modified"),
        String::new(),
        CONFIG.details.private_key.clone(),
    );
    Ok(())
}

#[tokio::test]
async fn test_basic_image_operations_sequentially() {
    println!("Running Setup ...");
    let (id, md5) = run_setup_image().await.expect("Should be able to setup image");
    wait_for_done_task(&id).await;

    println!("Running Prove ...");
    let id = run_prove_image(md5.clone()).await.expect("Should be able to prove image");
    wait_for_done_task(&id).await;

    println!("Running Reset ...");
    let id = run_reset_image(md5.clone()).await.expect("Should be able to reset image");
    wait_for_done_task(&id).await;

    println!("Running Modify ...");
    run_modify_image(md5.clone()).await.expect("Should be able to modify image");
}

#[cfg(test)]
mod reprocess {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_force_unprovable_to_reprocess() {
        let ids = std::env::var("IDS_TO_REPROCESS")
            .expect("Failed to get IDS_TO_REPROCESS")
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let n_ids = ids.len();

        let res = run_test!(
            ZkWasmServiceHelper::force_unprovable_to_reprocess,
            ids,
            CONFIG.user_address().clone(),
            CONFIG.details.private_key.clone(),
        );
        assert_eq!(n_ids, res.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_force_dryrun_fails_to_reprocess() {
        let ids = std::env::var("IDS_TO_REPROCESS")
            .expect("Failed to get IDS_TO_REPROCESS")
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let n_ids = ids.len();

        let res = run_test!(
            ZkWasmServiceHelper::force_dryrun_fails_to_reprocess,
            ids,
            CONFIG.user_address().clone(),
            CONFIG.details.private_key.clone(),
        );
        assert_eq!(n_ids, res.len());
    }
}
