use hat_specifications::{
    ACTION_RESULT_SCHEMA, ActionReference, HatActionResult, HatInvocationOutcome,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::worker::Lease;
use crate::worker_io::{canonical_directory, message, path, required, run_hatter, write_document};

const INPUT_SCHEMA: &str =
    "hathq://hat-japan-administrative-areas/list-administrative-areas-input/v1";
const OUTPUT_SCHEMA: &str =
    "hathq://hat-japan-administrative-areas/list-administrative-areas-output/v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AreaInput {
    schema: String,
    residence_country_region: String,
}

#[derive(Serialize)]
struct AreaOutput<'a> {
    schema: &'static str,
    residence_country_region: &'static str,
    areas: Vec<AreaValue<'a>>,
}

#[derive(Serialize)]
struct AreaValue<'a> {
    code: &'a str,
    name_ja: &'a str,
    name_en: &'a str,
}

pub(super) fn process(
    args: &BTreeMap<String, String>,
    common: &[String],
    control: &Path,
    lease: &Lease,
) -> Result<(), String> {
    let invocation = &lease.record.invocation;
    if invocation.operation_id != hat_japan_administrative_areas::OPERATION_ID
        || invocation.input.owner_id != "zixcel-graph"
    {
        return Err("invocation is outside the administrative-area worker contract".into());
    }
    let input_store = canonical_directory(required(args, "input-store")?)?;
    let input_bytes = read_digest_document(&input_store, &invocation.input)?;
    let input: AreaInput = serde_json::from_slice(&input_bytes).map_err(message)?;
    if input.schema != INPUT_SCHEMA
        || input.residence_country_region != hat_japan_administrative_areas::COUNTRY_CODE
    {
        return Err("input is outside the administrative-area operation schema".into());
    }
    let areas = hat_japan_administrative_areas::list_for_country(&input.residence_country_region)
        .ok_or("residence-country-unsupported")?
        .iter()
        .map(|value| AreaValue {
            code: value.code,
            name_ja: value.name_ja,
            name_en: value.name_en,
        })
        .collect();
    let output = AreaOutput {
        schema: OUTPUT_SCHEMA,
        residence_country_region: hat_japan_administrative_areas::COUNTRY_CODE,
        areas,
    };
    let bytes = serde_json::to_vec(&output).map_err(message)?;
    let digest = hex::encode(Sha256::digest(&bytes));
    let output_store = canonical_directory(required(args, "output-store")?)?;
    write_document(&output_store.join(format!("{digest}.json")), &bytes)?;
    complete(args, common, control, lease, &digest)?;
    println!("{{\"processed\":true,\"outputDigestSha256\":\"{digest}\"}}");
    Ok(())
}

fn complete(
    args: &BTreeMap<String, String>,
    common: &[String],
    control: &Path,
    lease: &Lease,
    digest: &str,
) -> Result<(), String> {
    let invocation = &lease.record.invocation;
    let result = HatActionResult {
        schema: ACTION_RESULT_SCHEMA.into(),
        invocation_id: invocation.invocation_id.clone(),
        operation_id: hat_japan_administrative_areas::OPERATION_ID.into(),
        state_revision: lease.record.status.state_revision.saturating_add(1),
        projection_revision: invocation.expected_projection_revision.saturating_add(1),
        outcome: HatInvocationOutcome::Completed,
        output: Some(ActionReference {
            owner_id: hat_japan_administrative_areas::REPOSITORY_ID.into(),
            reference: digest.into(),
            schema_id: OUTPUT_SCHEMA.into(),
            digest_sha256: digest.into(),
        }),
        reason_id: None,
        evidence_refs: Vec::new(),
    };
    let result_path = control.join(format!("result-{}.json", result.invocation_id));
    write_document(&result_path, &serde_json::to_vec(&result).map_err(message)?)?;
    run_hatter(
        args,
        "complete",
        common,
        &[
            "--worker-id",
            required(args, "worker-id")?,
            "--result-json",
            path(&result_path)?,
        ],
    )?;
    Ok(())
}

fn read_digest_document(root: &Path, reference: &ActionReference) -> Result<Vec<u8>, String> {
    if reference.schema_id != INPUT_SCHEMA || reference.reference != reference.digest_sha256 {
        return Err("input reference is not content-addressed".into());
    }
    let bytes = fs::read(root.join(format!("{}.json", reference.reference))).map_err(message)?;
    if bytes.len() > 1_048_576 || hex::encode(Sha256::digest(&bytes)) != reference.digest_sha256 {
        return Err("input digest differs".into());
    }
    Ok(bytes)
}
