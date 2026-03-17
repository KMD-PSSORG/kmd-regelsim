use kmd_engine::wasm_api::Engine;

// ── Test 1: get_geo_data returns per-kommune diff with per-capita ──
#[test]
fn test_geo_data_returns() {
    let mut engine = Engine::new();
    let _ = engine.init();
    let _ = engine.apply_scenario(0, 13_050.0);

    let geo_json = engine.get_geo_data();
    let geo: serde_json::Value = serde_json::from_str(&geo_json).unwrap();

    let kommuner = geo["kommuner"].as_array().unwrap();

    // Should cover all 98 kommuner (including those with zero delta)
    assert_eq!(kommuner.len(), 98, "should have all 98 kommuner, got {}", kommuner.len());

    let mut found_positive = false;
    let mut found_negative = false;
    let mut found_zero = false;

    for k in kommuner {
        assert!(k["kommune_id"].is_number(), "missing kommune_id");
        assert!(k["population"].is_number(), "missing population");
        assert!(k["total_delta"].is_number(), "missing total_delta");
        assert!(k["per_capita_delta"].is_number(), "missing per_capita_delta");
        assert!(k["affected_count"].is_number(), "missing affected_count");

        let pop = k["population"].as_u64().unwrap();
        assert!(pop > 0, "every kommune should have population");

        let total = k["total_delta"].as_f64().unwrap();
        let per_cap = k["per_capita_delta"].as_f64().unwrap();
        let expected = total / pop as f64;
        assert!(
            (per_cap - expected).abs() < 0.01,
            "per_capita_delta should be total_delta/population: {} vs {}", per_cap, expected
        );

        if total > 1.0 { found_positive = true; }
        if total < -1.0 { found_negative = true; }
        if total.abs() < 0.01 { found_zero = true; }
    }

    assert!(found_positive || found_negative, "should have non-zero deltas");
    let _ = found_zero; // zero-delta kommuner are expected but not required
}
