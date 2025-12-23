//! PWF to TCX conversion logic

use crate::error::{ConversionError, ConversionWarning, TcxExportResult};
use crate::tcx::mappings::map_pwf_sport_to_tcx;
use chrono::{DateTime, Utc};
use pwf_core::history::{GpsPosition, Workout, WpsHistory};
use pwf_core::Modality;

/// Convert PWF history to TCX XML format
///
/// # Arguments
/// * `history` - PWF history structure to export
///
/// # Returns
/// TcxExportResult with TCX XML and any warnings
pub fn pwf_to_tcx(history: &WpsHistory) -> Result<TcxExportResult, ConversionError> {
    let mut result = TcxExportResult::new(String::new());

    // Create TCX structure
    let mut tcx_db = tcx::TrainingCenterDatabase {
        activities: None,
        folders: None,
        courses: None,
        extensions: None,
    };

    // Convert workouts to TCX activities
    let mut activities = Vec::new();
    for workout in &history.workouts {
        match convert_workout_to_activity(workout, &mut result) {
            Ok(activity) => activities.push(activity),
            Err(e) => {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: format!("Failed to convert workout: {}", e),
                });
            }
        }
    }

    // Wrap activities in the Activities structure
    if !activities.is_empty() {
        tcx_db.activities = Some(tcx::Activities { activities });
    } else {
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: "No workouts to export".to_string(),
        });
    }

    // Serialize to XML
    result.tcx_xml = serialize_tcx(&tcx_db)?;

    Ok(result)
}

/// Convert a PWF Workout to a TCX Activity
fn convert_workout_to_activity(
    workout: &Workout,
    result: &mut TcxExportResult,
) -> Result<tcx::Activity, ConversionError> {
    // Determine sport
    let sport = if let Some(sport_enum) = &workout.sport {
        map_pwf_sport_to_tcx(sport_enum)
    } else {
        // Default to "Other" if no sport specified
        result.add_warning(ConversionWarning::MissingField {
            source_field: "sport".to_string(),
            reason: "Workout has no sport, defaulting to 'Other'".to_string(),
        });
        "Other".to_string()
    };

    // Parse workout start time
    let id = if let Some(started_at) = &workout.started_at {
        started_at.clone()
    } else {
        // Fallback to date if no started_at
        format!("{}T00:00:00Z", workout.date)
    };

    // Create laps from exercises
    let mut laps = Vec::new();

    // Group exercises into a single lap or create multiple laps
    // For simplicity, we'll create one lap per exercise
    for (exercise_idx, exercise) in workout.exercises.iter().enumerate() {
        match convert_exercise_to_lap(exercise, workout, exercise_idx, result) {
            Ok(lap) => laps.push(lap),
            Err(e) => {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: format!("Failed to convert exercise '{}': {}", exercise.name, e),
                });
            }
        }
    }

    // If we have no laps, create a minimal lap
    if laps.is_empty() {
        laps.push(create_minimal_lap(workout, result)?);
    }

    let activity = tcx::Activity {
        sport,
        id,
        laps,
        notes: workout.notes.clone(),
        extensions: None,
    };

    Ok(activity)
}

/// Convert a PWF Exercise to a TCX Lap
fn convert_exercise_to_lap(
    exercise: &pwf_core::history::CompletedExercise,
    workout: &Workout,
    _exercise_idx: usize,
    result: &mut TcxExportResult,
) -> Result<tcx::ActivityLap, ConversionError> {
    // Calculate lap start time
    let _lap_start = if let Some(started_at) = &workout.started_at {
        started_at.clone()
    } else {
        format!("{}T00:00:00Z", workout.date)
    };

    // Calculate total time and distance for this exercise
    let mut total_time_seconds = 0.0;
    let mut total_distance_meters = 0.0;
    let mut total_calories = 0;
    let mut max_hr = None;
    let mut avg_hr_sum = 0.0;
    let mut hr_count = 0;

    // Process sets to calculate aggregates
    for set in &exercise.sets {
        // Add set duration
        if let Some(duration) = set.duration_sec {
            total_time_seconds += duration as f64;
        }

        // Add distance
        if let Some(distance) = set.distance_meters {
            total_distance_meters += distance;
        }

        // Track heart rate from telemetry
        if let Some(ref telemetry) = set.telemetry {
            if let Some(hr) = telemetry.heart_rate_avg {
                avg_hr_sum += hr as f64;
                hr_count += 1;
            }
            if let Some(hr) = telemetry.heart_rate_max {
                max_hr = Some(max_hr.unwrap_or(0).max(hr));
            }
        }
    }

    // Create trackpoints if we have GPS data
    let mut tracks = Vec::new();
    if let Some(telemetry) = &workout.telemetry {
        if let Some(gps_route) = &telemetry.gps_route {
            let track = convert_gps_route_to_track(&gps_route.positions, result)?;
            tracks.push(track);
        }
    }

    // Calculate average heart rate
    let average_heart_rate = if hr_count > 0 {
        Some(avg_hr_sum / hr_count as f64)
    } else {
        None
    };

    // Convert max_hr to Option<f64>
    let maximum_heart_rate = max_hr.map(|hr| hr as f64);

    // Default to 0 calories if not tracked
    if total_calories == 0 && workout.telemetry.is_some() {
        if let Some(ref telemetry) = workout.telemetry {
            if let Some(cal) = telemetry.total_calories {
                total_calories = cal as u16;
            }
        }
    }

    // Warn about strength training not being well-represented in TCX
    if exercise.modality == Some(Modality::Strength) {
        result.add_warning(ConversionWarning::UnsupportedFeature {
            feature: format!(
                "Strength exercise '{}' does not map well to TCX format (TCX is primarily for cardio)",
                exercise.name
            ),
        });
    }

    let lap = tcx::ActivityLap {
        total_time_seconds,
        distance_meters: total_distance_meters,
        maximum_speed: None, // We don't track instantaneous speed per set
        calories: total_calories,
        average_heart_rate,
        maximum_heart_rate,
        intensity: Some(tcx::Intensity::Active),
        cadence: None, // Could extract from sets if available
        trigger_method: Some(tcx::TriggerMethod::Manual),
        tracks,
        notes: exercise.notes.clone(),
        extensions: None,
    };

    Ok(lap)
}

/// Convert GPS route positions to TCX Track
fn convert_gps_route_to_track(
    positions: &[GpsPosition],
    _result: &mut TcxExportResult,
) -> Result<tcx::Track, ConversionError> {
    let mut trackpoints = Vec::new();

    for pos in positions {
        // Parse timestamp
        let time: DateTime<Utc> = pos
            .timestamp
            .parse()
            .map_err(|e| ConversionError::TcxWriteError(format!("Invalid timestamp: {}", e)))?;

        let position = Some(tcx::Position {
            latitude: pos.latitude_deg,
            longitude: pos.longitude_deg,
        });

        let heart_rate = pos
            .heart_rate_bpm
            .map(|hr| tcx::HeartRate { value: hr as f64 });

        let extensions = if pos.power_watts.is_some() || pos.cadence.is_some() {
            Some(tcx::Extensions {
                tpx: Some(tcx::Ns3Tpx {
                    speed: pos.speed_mps,
                    watts: pos.power_watts.map(|p| p as u16),
                }),
            })
        } else {
            None
        };

        let trackpoint = tcx::Trackpoint {
            time,
            position,
            altitude_meters: pos.elevation_m,
            distance_meters: None, // TCX trackpoint distance is cumulative, PWF doesn't store this per-point
            heart_rate,
            cadence: pos.cadence.map(|c| c as u8),
            extensions,
        };

        trackpoints.push(trackpoint);
    }

    Ok(tcx::Track { trackpoints })
}

/// Create a minimal lap when no exercises are present
fn create_minimal_lap(
    workout: &Workout,
    _result: &mut TcxExportResult,
) -> Result<tcx::ActivityLap, ConversionError> {
    let total_time_seconds = workout.duration_sec.unwrap_or(0) as f64;

    let lap = tcx::ActivityLap {
        total_time_seconds,
        distance_meters: 0.0,
        maximum_speed: None,
        calories: 0,
        average_heart_rate: None,
        maximum_heart_rate: None,
        intensity: Some(tcx::Intensity::Active),
        cadence: None,
        trigger_method: Some(tcx::TriggerMethod::Manual),
        tracks: Vec::new(),
        notes: None,
        extensions: None,
    };

    Ok(lap)
}

/// Serialize TCX structure to XML string
/// Since serde-xml-rs doesn't support sequences well, we manually construct XML
fn serialize_tcx(tcx_db: &tcx::TrainingCenterDatabase) -> Result<String, ConversionError> {
    let mut xml = String::new();

    // XML declaration
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');

    // Root element with namespaces
    xml.push_str(r#"<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2" "#);
    xml.push_str(r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#);
    xml.push_str(r#"xsi:schemaLocation="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2 http://www.garmin.com/xmlschemas/TrainingCenterDatabasev2.xsd" "#);
    xml.push_str(r#"xmlns:ns3="http://www.garmin.com/xmlschemas/ActivityExtension/v2">"#);
    xml.push('\n');

    // Activities section
    if let Some(ref activities) = tcx_db.activities {
        xml.push_str("  <Activities>\n");
        for activity in &activities.activities {
            serialize_activity(&mut xml, activity)?;
        }
        xml.push_str("  </Activities>\n");
    }

    // Close root element
    xml.push_str("</TrainingCenterDatabase>\n");

    Ok(xml)
}

/// Serialize a single Activity to XML
fn serialize_activity(xml: &mut String, activity: &tcx::Activity) -> Result<(), ConversionError> {
    xml.push_str(&format!(
        "    <Activity Sport=\"{}\">\n",
        xml_escape(&activity.sport)
    ));
    xml.push_str(&format!("      <Id>{}</Id>\n", xml_escape(&activity.id)));

    // Laps
    for lap in &activity.laps {
        serialize_lap(xml, lap)?;
    }

    // Notes
    if let Some(ref notes) = activity.notes {
        xml.push_str(&format!("      <Notes>{}</Notes>\n", xml_escape(notes)));
    }

    xml.push_str("    </Activity>\n");
    Ok(())
}

/// Serialize a single Lap to XML
fn serialize_lap(xml: &mut String, lap: &tcx::ActivityLap) -> Result<(), ConversionError> {
    // For now, use the activity ID as start time (simplified)
    xml.push_str("      <Lap StartTime=\"2025-01-01T00:00:00Z\">\n");

    xml.push_str(&format!(
        "        <TotalTimeSeconds>{}</TotalTimeSeconds>\n",
        lap.total_time_seconds
    ));
    xml.push_str(&format!(
        "        <DistanceMeters>{}</DistanceMeters>\n",
        lap.distance_meters
    ));

    if let Some(max_speed) = lap.maximum_speed {
        xml.push_str(&format!(
            "        <MaximumSpeed>{}</MaximumSpeed>\n",
            max_speed
        ));
    }

    xml.push_str(&format!("        <Calories>{}</Calories>\n", lap.calories));

    if let Some(avg_hr) = lap.average_heart_rate {
        xml.push_str(&format!("        <AverageHeartRateBpm>\n          <Value>{}</Value>\n        </AverageHeartRateBpm>\n", avg_hr as u32));
    }

    if let Some(max_hr) = lap.maximum_heart_rate {
        xml.push_str(&format!("        <MaximumHeartRateBpm>\n          <Value>{}</Value>\n        </MaximumHeartRateBpm>\n", max_hr as u32));
    }

    xml.push_str("        <Intensity>Active</Intensity>\n");
    xml.push_str("        <TriggerMethod>Manual</TriggerMethod>\n");

    // Tracks
    for track in &lap.tracks {
        serialize_track(xml, track)?;
    }

    if let Some(ref notes) = lap.notes {
        xml.push_str(&format!("        <Notes>{}</Notes>\n", xml_escape(notes)));
    }

    xml.push_str("      </Lap>\n");
    Ok(())
}

/// Serialize a single Track to XML
fn serialize_track(xml: &mut String, track: &tcx::Track) -> Result<(), ConversionError> {
    xml.push_str("        <Track>\n");

    for trackpoint in &track.trackpoints {
        serialize_trackpoint(xml, trackpoint)?;
    }

    xml.push_str("        </Track>\n");
    Ok(())
}

/// Serialize a single Trackpoint to XML
fn serialize_trackpoint(xml: &mut String, tp: &tcx::Trackpoint) -> Result<(), ConversionError> {
    xml.push_str("          <Trackpoint>\n");

    xml.push_str(&format!(
        "            <Time>{}</Time>\n",
        tp.time.to_rfc3339()
    ));

    if let Some(ref pos) = tp.position {
        xml.push_str("            <Position>\n");
        xml.push_str(&format!(
            "              <LatitudeDegrees>{}</LatitudeDegrees>\n",
            pos.latitude
        ));
        xml.push_str(&format!(
            "              <LongitudeDegrees>{}</LongitudeDegrees>\n",
            pos.longitude
        ));
        xml.push_str("            </Position>\n");
    }

    if let Some(altitude) = tp.altitude_meters {
        xml.push_str(&format!(
            "            <AltitudeMeters>{}</AltitudeMeters>\n",
            altitude
        ));
    }

    if let Some(distance) = tp.distance_meters {
        xml.push_str(&format!(
            "            <DistanceMeters>{}</DistanceMeters>\n",
            distance
        ));
    }

    if let Some(ref hr) = tp.heart_rate {
        xml.push_str(&format!("            <HeartRateBpm>\n              <Value>{}</Value>\n            </HeartRateBpm>\n", hr.value as u32));
    }

    if let Some(cadence) = tp.cadence {
        xml.push_str(&format!("            <Cadence>{}</Cadence>\n", cadence));
    }

    // Extensions for power/speed
    if let Some(ref ext) = tp.extensions {
        if let Some(ref tpx) = ext.tpx {
            xml.push_str("            <Extensions>\n");
            xml.push_str("              <ns3:TPX>\n");

            if let Some(speed) = tpx.speed {
                xml.push_str(&format!(
                    "                <ns3:Speed>{}</ns3:Speed>\n",
                    speed
                ));
            }

            if let Some(watts) = tpx.watts {
                xml.push_str(&format!(
                    "                <ns3:Watts>{}</ns3:Watts>\n",
                    watts
                ));
            }

            xml.push_str("              </ns3:TPX>\n");
            xml.push_str("            </Extensions>\n");
        }
    }

    xml.push_str("          </Trackpoint>\n");
    Ok(())
}

/// Escape XML special characters
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pwf_core::history::Units;
    use pwf_core::Sport;

    // ===== Unit Tests for xml_escape =====

    #[test]
    fn test_xml_escape_ampersand() {
        assert_eq!(xml_escape("A & B"), "A &amp; B");
    }

    #[test]
    fn test_xml_escape_less_than() {
        assert_eq!(xml_escape("A < B"), "A &lt; B");
    }

    #[test]
    fn test_xml_escape_greater_than() {
        assert_eq!(xml_escape("A > B"), "A &gt; B");
    }

    #[test]
    fn test_xml_escape_quote() {
        assert_eq!(xml_escape("A \"B\" C"), "A &quot;B&quot; C");
    }

    #[test]
    fn test_xml_escape_apostrophe() {
        assert_eq!(xml_escape("A 'B' C"), "A &apos;B&apos; C");
    }

    #[test]
    fn test_xml_escape_all_special_chars() {
        assert_eq!(xml_escape("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
    }

    #[test]
    fn test_xml_escape_no_special_chars() {
        assert_eq!(xml_escape("Hello World"), "Hello World");
    }

    #[test]
    fn test_xml_escape_mixed_content() {
        assert_eq!(
            xml_escape("Run & <recovery> with \"quotes\" and 'apostrophes' > 5km"),
            "Run &amp; &lt;recovery&gt; with &quot;quotes&quot; and &apos;apostrophes&apos; &gt; 5km"
        );
    }

    #[test]
    fn test_xml_escape_empty_string() {
        assert_eq!(xml_escape(""), "");
    }

    // ===== Unit Tests for serialize_tcx =====

    #[test]
    fn test_serialize_tcx_empty_database() {
        let tcx_db = tcx::TrainingCenterDatabase {
            activities: None,
            folders: None,
            courses: None,
            extensions: None,
        };

        let xml = serialize_tcx(&tcx_db).unwrap();

        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<TrainingCenterDatabase"));
        assert!(xml.contains("</TrainingCenterDatabase>"));
        assert!(
            xml.contains("xmlns=\"http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2\"")
        );
    }

    #[test]
    fn test_serialize_tcx_with_namespaces() {
        let tcx_db = tcx::TrainingCenterDatabase {
            activities: None,
            folders: None,
            courses: None,
            extensions: None,
        };

        let xml = serialize_tcx(&tcx_db).unwrap();

        assert!(xml.contains("xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\""));
        assert!(xml.contains("xmlns:ns3=\"http://www.garmin.com/xmlschemas/ActivityExtension/v2\""));
    }

    // ===== Integration Tests =====

    #[test]
    fn test_pwf_to_tcx_empty_history() {
        let history = WpsHistory {
            history_version: 2,
            exported_at: "2025-01-20T12:00:00Z".to_string(),
            export_source: None,
            units: Units::default(),
            workouts: Vec::new(),
            personal_records: Vec::new(),
            body_measurements: Vec::new(),
        };

        let result = pwf_to_tcx(&history).unwrap();
        assert!(result.has_warnings());
        assert!(result.tcx_xml.contains("<TrainingCenterDatabase"));
    }

    #[test]
    fn test_pwf_to_tcx_simple_workout() {
        let workout = Workout {
            id: Some("w1".to_string()),
            date: "2025-01-20".to_string(),
            started_at: Some("2025-01-20T10:00:00Z".to_string()),
            ended_at: Some("2025-01-20T11:00:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Morning Run".to_string()),
            notes: Some("Great run!".to_string()),
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
            telemetry: None,
            devices: Vec::new(),
            sport: Some(Sport::Running),
            sport_segments: None,
        };

        let history = WpsHistory {
            history_version: 2,
            exported_at: "2025-01-20T12:00:00Z".to_string(),
            export_source: None,
            units: Units::default(),
            workouts: vec![workout],
            personal_records: Vec::new(),
            body_measurements: Vec::new(),
        };

        let result = pwf_to_tcx(&history).unwrap();
        assert!(result.tcx_xml.contains("<TrainingCenterDatabase"));
        assert!(result.tcx_xml.contains("Running"));
        assert!(result.tcx_xml.contains("Great run!"));
    }
}
