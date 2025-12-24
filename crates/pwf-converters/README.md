# PWF Converters

Format conversion library for the Portable Workout Format (PWF). This library enables importing workout data from various fitness file formats into PWF.

## Features

### Supported Conversions

#### Import (to PWF)
- **FIT → PWF** ✅
  - Garmin, COROS, Wahoo, and other FIT-compatible devices
  - Full support for PWF v2.1 advanced features

- **TCX → PWF** ✅
  - Training Center XML format (Garmin Connect, Strava exports)
  - GPS routes, heart rate, cadence, and multi-lap workouts

#### Export (from PWF)
- **PWF → TCX** ✅
  - Export to Training Center XML for Garmin Connect, Strava, TrainingPeaks
  - Full GPS, heart rate, power, and cadence support

### FIT Format Support

The FIT (Flexible and Interoperable Data Transfer) converter extracts:

#### Basic Workout Data
- Session timestamps and duration
- Sport type detection (running, cycling, swimming, etc.)
- Lap/set structure
- Distance and pace metrics
- Heart rate data (average, max)

#### Advanced Metrics (PWF v2.1)
- **Power Metrics**
  - Normalized Power (NP)
  - Training Stress Score (TSS)
  - Intensity Factor (IF)
  - Variability Index (VI)
  - Functional Threshold Power (FTP)
  - Total work (kJ)

- **Physiological Metrics**
  - Training Effect (Firstbeat/Garmin)
  - Anaerobic Training Effect
  - Recovery Time (hours)

- **GPS Route Tracking**
  - Complete position history with timestamps
  - Elevation data (ascent, descent, extremes)
  - Speed and heading per position
  - Heart rate, power, cadence, temperature per point
  - Bounding box calculation for map display
  - Smart recording mode detection

- **Swimming Data**
  - Pool configuration (length and unit detection)
  - Individual length tracking with:
    - Stroke type (freestyle, backstroke, breaststroke, butterfly, drill, IM)
    - Stroke count
    - SWOLF score calculation
    - Active vs. rest length detection
    - Duration per length
  - Aggregate metrics (total/active lengths, average SWOLF)

- **Multi-Sport Activities**
  - Automatic triathlon/duathlon detection
  - Sport segment creation per discipline
  - Transition tracking between segments
  - Per-segment telemetry

- **Device Information**
  - Manufacturer detection (Garmin, Polar, Wahoo, Suunto, COROS)
  - Device type (watch, bike computer, HR monitor, power meter)
  - Product model and serial number
  - Software/hardware versions
  - Battery and operating time tracking

## Usage

### Command Line

```bash
# Import from FIT
pwf convert --from fit --to pwf activity.fit workout.yaml

# Import from TCX
pwf convert --from tcx --to pwf activity.tcx workout.yaml

# Export to TCX
pwf convert --from pwf --to tcx workout.yaml activity.tcx

# Summary only (skip time-series GPS data for imports)
pwf convert --from tcx --to pwf --summary-only activity.tcx workout.yaml

# Verbose output (show conversion warnings and progress)
pwf convert --from fit --to pwf --verbose activity.fit workout.yaml
```

### TCX Format Support

The TCX (Training Center XML) converter extracts:

#### Supported Data
- **Workout Basics**
  - Activity timestamps and duration
  - Sport type (running, cycling, swimming, rowing, etc.)
  - Multi-lap structure
  - Distance and pace metrics

- **Telemetry**
  - Heart rate (average and maximum per lap)
  - Cadence (average)
  - Calories burned

- **GPS Route Tracking**
  - Position history with timestamps
  - Elevation data
  - Bounding box calculation
  - Per-point heart rate and cadence

- **Multi-Lap Activities**
  - Each lap becomes a PWF exercise/set
  - Lap-level telemetry aggregation
  - Support for interval workouts

#### Limitations
- No power metrics (not in standard TCX schema)
- No device information (vendor extensions only)
- Swimming stroke data not typically included

### Library Usage

```rust
use pwf_converters::{fit_to_pwf, tcx_to_pwf, pwf_to_tcx};
use std::fs::File;

// Import: Convert FIT file to PWF YAML
let fit_file = File::open("activity.fit")?;
let result = fit_to_pwf(fit_file, false)?;

// Import: Convert TCX file to PWF YAML
let tcx_file = File::open("activity.tcx")?;
let result = tcx_to_pwf(tcx_file, false)?;

// Export: Convert PWF history to TCX XML
let history: pwf_core::history::WpsHistory = pwf_core::history::parse(&pwf_yaml)?;
let result = pwf_to_tcx(&history)?;

// Check for warnings (both import and export)
if result.has_warnings() {
    for warning in &result.warnings {
        println!("Warning: {}", warning);
    }
}

// Save PWF YAML (import)
std::fs::write("workout.yaml", &result.pwf_yaml)?;

// Save TCX XML (export)
std::fs::write("activity.tcx", &result.tcx_xml)?;
```

## Conversion Quality

### Data Preservation

The FIT converter aims to preserve >95% of relevant FIT data fields:

- ✅ **Fully Supported**: Sessions, laps, records, device info, GPS, swimming lengths
- ✅ **Power Metrics**: NP, TSS, IF, VI, FTP (cycling/running with power)
- ✅ **Swimming**: Pool config, stroke types, SWOLF, length tracking
- ✅ **Multi-Sport**: Triathlon/duathlon segment detection
- ⚠️  **Partially Supported**: Some FIT-specific fields may not have PWF equivalents
- ❌ **Not Supported**: FIT file writing (read-only), proprietary metrics

### Warnings and Data Loss

The converter provides detailed warnings for:

- **Missing Fields**: FIT fields without PWF equivalents
- **Value Clamping**: Values adjusted to fit PWF constraints
- **Unsupported Features**: FIT-specific functionality not in PWF
- **Data Quality Issues**: Inconsistent or missing data

Example warnings:
```
⚠ Missing field 'fractional_cadence': PWF only supports integer cadence
⚠ Unsupported feature: lap_trigger
⚠ Data quality issue: No lap data found, creating single exercise
```

## Technical Details

### Coordinate Systems

- **FIT GPS**: Uses semicircles (2³¹ semicircles = 180°)
- **PWF GPS**: Uses decimal degrees
- Conversion: `degrees = semicircles × (180.0 / 2,147,483,648.0)`

### Timestamp Handling

- **FIT Epoch**: December 31, 1989, 00:00:00 UTC
- **PWF Format**: ISO 8601 (e.g., `2024-01-15T14:30:00Z`)
- Automatic timezone conversion

### Pool Length Detection

The converter automatically detects pool length:
- **50m pool**: 45-55 meters → `pool_length: 50.0, pool_length_unit: meters`
- **33yd pool**: 30-40 meters → `pool_length: 33.0, pool_length_unit: yards`
- **25m pool**: Default fallback

### Sport Type Mapping

| FIT Code | PWF Sport |
|----------|-----------|
| 0 | Running |
| 1 | Cycling |
| 2 | Transition (skipped) |
| 5 | Swimming |
| ... | Other |

### Multi-Sport Detection

Activities are detected as multi-sport when:
1. Multiple sessions exist in the FIT file
2. Sessions have different sport types (excluding transitions)
3. Each sport creates a `SportSegment` with telemetry

## Error Handling

The converter uses a comprehensive error system:

```rust
pub enum ConversionError {
    FitReadError(fitparser::Error),      // Failed to parse FIT file
    InvalidFitData(String),              // Corrupt or inconsistent data
    PwfValidationError(String),          // Converted data fails PWF validation
    IoError(std::io::Error),             // File I/O errors
    YamlError(serde_yaml::Error),        // YAML serialization errors
    UnsupportedFormat(String),           // Format not yet implemented
    MissingRequiredField(String),        // Critical FIT field missing
}
```

## Performance

- **Small files** (<1 MB): ~10-50ms
- **Large files** (>10 MB): ~100-500ms
- **GPS-heavy files**: Time scales with position count
- **Memory usage**: ~2-3x file size during conversion

Use `--summary-only` to skip GPS time-series data for faster conversion and smaller output files.

## Testing

The library includes comprehensive tests:

```bash
# Run all converter tests
cargo test -p pwf-converters

# Run integration tests
cargo test -p pwf-converters --test fit_conversion_tests

# Run with verbose output
cargo test -p pwf-converters -- --nocapture
```

## Export Formats

### PWF → TCX (Recommended Export Format) ✅

**Status:** Fully implemented

Export PWF workouts to TCX (Training Center XML) format for upload to Garmin Connect, Strava, TrainingPeaks, and other platforms:

```bash
# Export PWF history to TCX
pwf convert --from pwf --to tcx workout.yaml output.tcx

# Verbose output with warnings
pwf convert --from pwf --to tcx --verbose workout.yaml output.tcx
```

**TCX Benefits:**
- ✅ Universal platform support (Garmin, Strava, TrainingPeaks, etc.)
- ✅ Manual XML generation for full control
- ✅ Supports GPS routes, heart rate, power, and cadence
- ✅ Human-readable XML format
- ✅ Well-documented and stable schema

**TCX Conversion Coverage:**
- ✅ Full workout metadata (activity timestamps, duration)
- ✅ GPS routes with position, altitude
- ✅ Heart rate data (average and maximum per lap)
- ✅ Power and cadence via TrainingPeaks extensions (ns3:TPX)
- ✅ Multi-lap structure (one lap per exercise)
- ✅ Workout notes
- ⚠️ Strength exercises not well-represented (TCX is cardio-focused)
- ⚠️ Swimming data simplified to basic laps
- ⚠️ PWF-specific features like RPE and RIR not included

### PWF → FIT Export (Not Available)

**Status:** ❌ Not feasible with current Rust ecosystem

Direct FIT file writing is not currently supported due to lack of production-ready Rust libraries for FIT file creation.

**Technical Reasons:**
- `fitparser v0.5`: Read-only library (excellent for FIT → PWF import)
- `fit-rust v0.1`: Experimental, undocumented, pre-release (not production-ready)
- No official Garmin FIT SDK for Rust (only C#, Java, Python, JavaScript)
- Implementing FIT writer from scratch requires 12-16 weeks of development

**Recommended Alternative:**
Use TCX export instead. TCX files are accepted by all platforms that support FIT and provide ~95% data coverage.

**Workaround (if FIT is absolutely required):**
1. Export PWF to TCX: `pwf export --to tcx workout.yaml output.tcx`
2. Use [Garmin FitCSVTool](https://developer.garmin.com/fit/fitcsvtool/) to convert TCX → FIT

For detailed analysis, see: [FIT_EXPORT_ANALYSIS.md](../../FIT_EXPORT_ANALYSIS.md)

**Future Consideration:**
We monitor the Rust ecosystem for FIT writing libraries. If a production-ready library emerges (version 1.0+, well-documented, actively maintained), we will implement FIT export.

## Future Formats

Planned converter support:
- **GPX → PWF** (GPS Exchange Format import for basic GPS tracks)
- **GPX ← PWF** (Export routes for navigation apps)
- **PWF → CSV** (Export time-series telemetry data for spreadsheet analysis)
- **PWF → FIT** (If production-ready Rust library emerges - see `FIT_EXPORT_ANALYSIS.md`)

## Contributing

When adding new format converters:

1. Create a new module in `src/` (e.g., `src/tcx/`)
2. Implement the `to_pwf()` function
3. Add comprehensive tests in `tests/`
4. Update this README with format details
5. Add CLI integration in `pwf-cli/src/main.rs`

## Dependencies

- **fitparser** (v0.5): FIT file parsing (read-only)
- **pwf-core**: PWF type system and validation
- **chrono**: Timestamp conversion
- **thiserror**: Error handling
- **serde_yaml**: PWF YAML serialization

## License

Same as the parent PWF project.
