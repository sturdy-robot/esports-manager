use esm_models::esport_type::EsportType;

// ---------------------------------------------------------------------------
// Construction & display
// ---------------------------------------------------------------------------

#[test]
fn esport_type_moba_display() {
    assert_eq!(EsportType::Moba.as_str(), "Moba");
}

#[test]
fn esport_type_rts_display() {
    assert_eq!(EsportType::Rts.as_str(), "Rts");
}

#[test]
fn esport_type_fps_display() {
    assert_eq!(EsportType::Fps.as_str(), "Fps");
}

// ---------------------------------------------------------------------------
// From string (for DB deserialization)
// ---------------------------------------------------------------------------

#[test]
fn esport_type_from_str_moba() {
    let t: EsportType = "Moba".parse().unwrap();
    assert_eq!(t, EsportType::Moba);
}

#[test]
fn esport_type_from_str_rts() {
    let t: EsportType = "Rts".parse().unwrap();
    assert_eq!(t, EsportType::Rts);
}

#[test]
fn esport_type_from_str_fps() {
    let t: EsportType = "Fps".parse().unwrap();
    assert_eq!(t, EsportType::Fps);
}

#[test]
fn esport_type_from_str_invalid_returns_error() {
    let result: Result<EsportType, _> = "Racing".parse();
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Serialization roundtrip
// ---------------------------------------------------------------------------

#[test]
fn esport_type_serde_roundtrip() {
    let original = EsportType::Moba;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: EsportType = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn esport_type_serde_all_variants() {
    for variant in &[EsportType::Moba, EsportType::Rts, EsportType::Fps] {
        let json = serde_json::to_string(variant).unwrap();
        let back: EsportType = serde_json::from_str(&json).unwrap();
        assert_eq!(*variant, back);
    }
}
