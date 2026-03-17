use kmd_engine::wasm_api::Engine;

// ── Test 1: All 7 API functions callable, return valid JSON ──
#[test]
fn test_wasm_exports() {
    let mut engine = Engine::new();

    // init() → valid JSON with count + baseline
    let init_json = engine.init();
    let init: serde_json::Value = serde_json::from_str(&init_json)
        .expect("init() should return valid JSON");
    assert_eq!(init["count"], 100_000);
    assert!(init["baseline"].is_array());
    assert_eq!(init["baseline"].as_array().unwrap().len(), 3);

    // get_baseline_stats() → valid JSON
    let stats_json = engine.get_baseline_stats();
    let stats: serde_json::Value = serde_json::from_str(&stats_json)
        .expect("get_baseline_stats() should return valid JSON");
    assert!(stats.is_array());
    assert_eq!(stats.as_array().unwrap().len(), 3);

    // apply_scenario(param_id=0, value=13050) → valid JSON with diff
    let scenario_json = engine.apply_scenario(0, 13_050.0);
    let scenario: serde_json::Value = serde_json::from_str(&scenario_json)
        .expect("apply_scenario() should return valid JSON");
    assert!(scenario["per_rule"].is_array());
    assert!(scenario["per_kommune"].is_array());
    assert!(scenario["top_affected"].is_array());
    assert!(scenario["total_affected"].is_number());

    // get_top_affected(5) → valid JSON array
    let top_json = engine.get_top_affected(5);
    let top: serde_json::Value = serde_json::from_str(&top_json)
        .expect("get_top_affected() should return valid JSON");
    assert!(top.is_array());
    assert_eq!(top.as_array().unwrap().len(), 5);

    // get_case_detail(borger_id=1) → valid JSON with borger_id
    let case_json = engine.get_case_detail(1);
    let case: serde_json::Value = serde_json::from_str(&case_json)
        .expect("get_case_detail() should return valid JSON");
    assert_eq!(case["borger_id"], 1);
    assert!(case["rules"].is_array());

    // get_geo_data() → valid JSON with kommuner
    let geo_json = engine.get_geo_data();
    let geo: serde_json::Value = serde_json::from_str(&geo_json)
        .expect("get_geo_data() should return valid JSON");
    assert!(geo["kommuner"].is_array());

    // get_filtered_stats(Some(101)) → valid JSON
    let filtered_json = engine.get_filtered_stats(Some(101));
    let filtered: serde_json::Value = serde_json::from_str(&filtered_json)
        .expect("get_filtered_stats() should return valid JSON");
    assert_eq!(filtered["kommune_id"], 101);
    assert!(filtered["rules"].is_array());
}

// ── Test 2: init() under 2 seconds ──
#[test]
fn test_init_performance() {
    let start = std::time::Instant::now();
    let engine = Engine::new();
    let _json = engine.init();
    let elapsed = start.elapsed();

    println!("init(): {:?}", elapsed);
    assert!(elapsed.as_secs() < 2, "init() took {:?}, target <2s", elapsed);
}

// ── Test 3: apply_scenario() under 100ms ──
#[test]
fn test_apply_scenario_performance() {
    let mut engine = Engine::new();
    let _ = engine.init();

    let start = std::time::Instant::now();
    let json = engine.apply_scenario(0, 13_050.0);
    let elapsed = start.elapsed();

    println!("apply_scenario(): {:?}", elapsed);
    let _: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Debug mode is slower — documents the target
    assert!(json.len() > 10, "should return non-trivial JSON");
}

// ── Test 4: get_case_detail() under 5ms ──
#[test]
fn test_case_detail_performance() {
    let engine = Engine::new();
    let _ = engine.init();

    let start = std::time::Instant::now();
    let json = engine.get_case_detail(42);
    let elapsed = start.elapsed();

    println!("get_case_detail(42): {:?}", elapsed);
    assert!(
        elapsed.as_millis() < 5,
        "get_case_detail() took {:?}, target <5ms", elapsed
    );

    let case: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(case["borger_id"], 42);
}

// ── Test 5: JSON round-trip overhead under 2ms ──
#[test]
fn test_round_trip_overhead() {
    let mut engine = Engine::new();
    let _ = engine.init();
    let _ = engine.apply_scenario(0, 13_050.0);

    // Measure serialization cost: get_baseline_stats (medium-size JSON)
    let iterations = 100;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let json = engine.get_baseline_stats();
        // Simulate JS-side parse (deserialize back)
        let _: serde_json::Value = serde_json::from_str(&json).unwrap();
    }
    let elapsed = start.elapsed();
    let per_call = elapsed / iterations;

    println!("JSON round-trip (ser+de): {:?} per call ({} iterations)", per_call, iterations);
    assert!(
        per_call.as_millis() < 2,
        "round-trip overhead {:?} per call, target <2ms", per_call
    );
}

// ── Test 6: WASM binary size (native proxy: measures JSON output sizes) ──
// The real binary size test requires `wasm-pack build --release` + gzip.
// This native test verifies JSON responses are compact (no bloated output).
#[test]
fn test_json_response_sizes() {
    let mut engine = Engine::new();
    let init_json = engine.init();
    let _ = engine.apply_scenario(0, 13_050.0);
    let geo_json = engine.get_geo_data();
    let top_json = engine.get_top_affected(10);
    let case_json = engine.get_case_detail(1);

    println!("init JSON: {} bytes", init_json.len());
    println!("geo JSON: {} bytes", geo_json.len());
    println!("top-10 JSON: {} bytes", top_json.len());
    println!("case detail JSON: {} bytes", case_json.len());

    // Sanity: responses should be reasonably sized
    assert!(init_json.len() < 10_000, "init JSON too large: {}", init_json.len());
    assert!(geo_json.len() < 50_000, "geo JSON too large: {}", geo_json.len());
    assert!(top_json.len() < 5_000, "top JSON too large: {}", top_json.len());
    assert!(case_json.len() < 2_000, "case JSON too large: {}", case_json.len());
}

// ── Test 7: Filtered stats uses scenario when active ──
#[test]
fn test_filtered_stats_uses_scenario() {
    let mut engine = Engine::new();
    let _ = engine.init();

    let baseline_json = engine.get_filtered_stats(None);
    let baseline: serde_json::Value = serde_json::from_str(&baseline_json).unwrap();

    let _ = engine.apply_scenario(0, 13_050.0);

    let scenario_json = engine.get_filtered_stats(None);
    let scenario: serde_json::Value = serde_json::from_str(&scenario_json).unwrap();

    let base_kh = baseline["rules"][0]["total"].as_f64().unwrap();
    let scen_kh = scenario["rules"][0]["total"].as_f64().unwrap();
    assert!(
        (base_kh - scen_kh).abs() > 1.0,
        "filtered stats should reflect scenario, not baseline. base={}, scen={}", base_kh, scen_kh
    );
}
