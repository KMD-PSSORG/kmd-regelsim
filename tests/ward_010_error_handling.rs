use kmd_engine::wasm_api::Engine;

// ── Test: Invalid API calls return JSON errors, never panic ──
#[test]
fn test_error_responses_no_panic() {
    let mut engine = Engine::new();
    let _ = engine.init();

    // get_geo_data before any scenario → should return JSON error, not panic
    let geo = engine.get_geo_data();
    let geo_val: serde_json::Value = serde_json::from_str(&geo).unwrap();
    assert!(
        geo_val.get("error").is_some(),
        "get_geo_data without scenario should return JSON error, got: {}", geo
    );

    // get_top_affected before any scenario → should return JSON error
    let top = engine.get_top_affected(10);
    let top_val: serde_json::Value = serde_json::from_str(&top).unwrap();
    assert!(
        top_val.get("error").is_some(),
        "get_top_affected without scenario should return JSON error, got: {}", top
    );

    // apply_scenario with invalid param_id → should return JSON error
    let bad_scenario = engine.apply_scenario(255, 1000.0);
    let bad_val: serde_json::Value = serde_json::from_str(&bad_scenario).unwrap();
    assert!(
        bad_val.get("error").is_some(),
        "apply_scenario with invalid param_id should return JSON error, got: {}", bad_scenario
    );

    // get_case_detail with non-existent borger_id → should return JSON error
    let bad_case = engine.get_case_detail(999_999);
    let bad_case_val: serde_json::Value = serde_json::from_str(&bad_case).unwrap();
    assert!(
        bad_case_val.get("error").is_some(),
        "get_case_detail with bad id should return JSON error, got: {}", bad_case
    );

    // Valid calls should still work
    let _ = engine.apply_scenario(0, 13_050.0);
    let geo_ok = engine.get_geo_data();
    let geo_ok_val: serde_json::Value = serde_json::from_str(&geo_ok).unwrap();
    assert!(
        geo_ok_val.get("kommuner").is_some(),
        "valid get_geo_data should return kommuner"
    );
}
