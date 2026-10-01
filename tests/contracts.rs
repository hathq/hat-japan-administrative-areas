use hat_japan_administrative_areas::{
    ADMINISTRATIVE_AREAS, PACKAGE_ID, PACKAGE_JSON, REPOSITORY_ID, list_for_country, select_exact,
};

const SCHEMAS: &[(&str, &str)] = &[
    (
        "hathq://hat-japan-administrative-areas/administrative-area/v1",
        include_str!("../schemas/administrative-area-v1.schema.json"),
    ),
    (
        "hathq://hat-japan-administrative-areas/administrative-area-selection-event/v1",
        include_str!("../schemas/administrative-area-selection-event-v1.schema.json"),
    ),
    (
        "hathq://hat-japan-administrative-areas/administrative-area-selection-projection/v1",
        include_str!("../schemas/administrative-area-selection-projection-v1.schema.json"),
    ),
    (
        "hathq://hat-japan-administrative-areas/list-administrative-areas-input/v1",
        include_str!("../schemas/list-administrative-areas-input-v1.schema.json"),
    ),
    (
        "hathq://hat-japan-administrative-areas/list-administrative-areas-output/v1",
        include_str!("../schemas/list-administrative-areas-output-v1.schema.json"),
    ),
    (
        "hathq://hat-japan-administrative-areas/residence-country/v1",
        include_str!("../schemas/residence-country-v1.schema.json"),
    ),
];

#[test]
fn package_identity_and_fail_closed_contract_are_present() {
    let value: serde_json::Value = serde_json::from_str(PACKAGE_JSON).expect("package JSON");
    assert_eq!(value["schema"], "hathq://hat/package/v2");
    assert_eq!(value["package_id"], PACKAGE_ID);
    assert_eq!(value["repository_id"], REPOSITORY_ID);
    assert_eq!(
        value["operations"][0]["context_plan"]["unresolved_policy"],
        "record-unresolved"
    );
    assert_eq!(
        value["operations"][0]["reducer"]["conflict_policy"],
        "reject"
    );
    assert_eq!(value["operations"][0]["handler"]["kind"], "declarative-hat");
}

#[test]
fn release_contains_all_prefectures_in_exact_code_order() {
    assert_eq!(ADMINISTRATIVE_AREAS.len(), 47);
    assert_eq!(ADMINISTRATIVE_AREAS.first().expect("first").code, "JP-01");
    assert_eq!(ADMINISTRATIVE_AREAS.last().expect("last").code, "JP-47");
    assert!(
        ADMINISTRATIVE_AREAS
            .windows(2)
            .all(|pair| pair[0].code < pair[1].code)
    );
    assert_eq!(
        select_exact("JP", "JP-13").expect("Tokyo").name_ja,
        "東京都"
    );
}

#[test]
fn unsupported_or_ambiguous_input_is_never_completed() {
    assert_eq!(list_for_country("US"), None);
    assert_eq!(
        select_exact("jp", "JP-13"),
        Err("residence-country-unsupported")
    );
    assert_eq!(select_exact("JP", "13"), Err("administrative-area-unknown"));
    assert_eq!(
        select_exact("JP", "東京"),
        Err("administrative-area-unknown")
    );
}

#[test]
fn every_released_schema_is_closed_and_has_its_exact_identity() {
    for (identity, bytes) in SCHEMAS {
        let value: serde_json::Value = serde_json::from_str(bytes).expect("schema JSON");
        assert_eq!(value["$id"], *identity);
        assert_eq!(value["additionalProperties"], false);
    }
}
