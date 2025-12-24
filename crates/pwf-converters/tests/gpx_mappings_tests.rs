//! Tests for GPX sport type mappings and metadata inference

use pwf_converters::gpx::mappings::{infer_sport_from_metadata, map_gpx_type_to_sport};
use pwf_core::Sport;
use gpx;

#[test]
fn test_map_gpx_type_basic_sports() {
    assert_eq!(map_gpx_type_to_sport(Some("run")), Sport::Running);
    assert_eq!(map_gpx_type_to_sport(Some("running")), Sport::Running);
    assert_eq!(map_gpx_type_to_sport(Some("bike")), Sport::Cycling);
    assert_eq!(map_gpx_type_to_sport(Some("biking")), Sport::Cycling);
    assert_eq!(map_gpx_type_to_sport(Some("cycling")), Sport::Cycling);
}

#[test]
fn test_map_gpx_type_outdoor_sports() {
    assert_eq!(map_gpx_type_to_sport(Some("hike")), Sport::Hiking);
    assert_eq!(map_gpx_type_to_sport(Some("hiking")), Sport::Hiking);
    assert_eq!(map_gpx_type_to_sport(Some("walk")), Sport::Walking);
    assert_eq!(map_gpx_type_to_sport(Some("walking")), Sport::Walking);
}

#[test]
fn test_map_gpx_type_water_sports() {
    assert_eq!(map_gpx_type_to_sport(Some("swim")), Sport::Swimming);
    assert_eq!(map_gpx_type_to_sport(Some("swimming")), Sport::Swimming);
    assert_eq!(map_gpx_type_to_sport(Some("row")), Sport::Rowing);
    assert_eq!(map_gpx_type_to_sport(Some("rowing")), Sport::Rowing);
    assert_eq!(map_gpx_type_to_sport(Some("kayak")), Sport::Kayaking);
    assert_eq!(map_gpx_type_to_sport(Some("kayaking")), Sport::Kayaking);
    assert_eq!(map_gpx_type_to_sport(Some("canoe")), Sport::Kayaking);
    assert_eq!(map_gpx_type_to_sport(Some("paddle")), Sport::StandUpPaddling);
    assert_eq!(map_gpx_type_to_sport(Some("paddling")), Sport::StandUpPaddling);
    assert_eq!(map_gpx_type_to_sport(Some("sup")), Sport::StandUpPaddling);
}

#[test]
fn test_map_gpx_type_winter_sports() {
    assert_eq!(map_gpx_type_to_sport(Some("ski")), Sport::CrossCountrySkiing);
    assert_eq!(map_gpx_type_to_sport(Some("skiing")), Sport::CrossCountrySkiing);
    assert_eq!(map_gpx_type_to_sport(Some("xc-ski")), Sport::CrossCountrySkiing);
    assert_eq!(map_gpx_type_to_sport(Some("downhill")), Sport::DownhillSkiing);
    assert_eq!(map_gpx_type_to_sport(Some("alpine")), Sport::DownhillSkiing);
    assert_eq!(map_gpx_type_to_sport(Some("snowboard")), Sport::Snowboarding);
    assert_eq!(map_gpx_type_to_sport(Some("snowboarding")), Sport::Snowboarding);
}

#[test]
fn test_map_gpx_type_unknown() {
    assert_eq!(map_gpx_type_to_sport(Some("unknown")), Sport::Other);
    assert_eq!(map_gpx_type_to_sport(Some("random")), Sport::Other);
    assert_eq!(map_gpx_type_to_sport(None), Sport::Other);
}

#[test]
fn test_infer_sport_from_keywords_running() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("running morning workout".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Running);
}

#[test]
fn test_infer_sport_from_keywords_cycling() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("bike ride cycling".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Cycling);
}

#[test]
fn test_infer_sport_from_keywords_hiking() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("hike trail mountain".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Hiking);
}

#[test]
fn test_infer_sport_from_keywords_walking() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("walk walking stroll".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Walking);
}

#[test]
fn test_infer_sport_from_keywords_swimming() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("swim swimming pool".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Swimming);
}

#[test]
fn test_infer_sport_from_description_cycling() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        description: Some("Cycling in the park".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Cycling);
}

#[test]
fn test_infer_sport_from_description_walking() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: None,
        description: Some("Walking around the neighborhood".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Walking);
}

#[test]
fn test_infer_sport_default_no_metadata() {
    let gpx = gpx::Gpx::default();
    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Other);
}

#[test]
fn test_infer_sport_no_matching_keywords() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("exercise fitness training".to_string()),
        description: Some("Generic workout session".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Other);
}

#[test]
fn test_infer_sport_keywords_override_description() {
    let mut gpx = gpx::Gpx::default();
    let metadata = gpx::Metadata {
        keywords: Some("running".to_string()),
        description: Some("Cycling in the park".to_string()),
        ..Default::default()
    };
    gpx.metadata = Some(metadata);

    // Keywords should take precedence
    assert_eq!(infer_sport_from_metadata(&gpx), Sport::Running);
}
