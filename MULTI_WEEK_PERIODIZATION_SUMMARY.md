# Multi-Week Periodization Feature - Implementation Summary

## Overview

This document summarizes the multi-week periodization feature implementation for PWF v2.0. This feature enables sophisticated training programs that vary systematically across multiple weeks, supporting all major periodization models used in strength and conditioning.

## What Has Been Completed

### 1. Feature Design

**File**: `/home/nitro/Projects/pwf/PERIODIZATION_IMPLEMENTATION.md`

Complete technical specification including:
- Data structures (PlanWeek, ExerciseOverride)
- Override resolution algorithm
- Validation rules and error codes (PWF-P050 through PWF-P064)
- JSON schema changes
- Best practices and usage guidelines
- Future enhancement roadmap

### 2. Example Plans

Three comprehensive example files demonstrating different periodization models:

#### a) Linear Periodization (12 weeks)
**File**: `/home/nitro/Projects/pwf/examples/periodization-linear.yaml`

- Classic linear progression model
- 3 phases: Accumulation (weeks 1-4), Transition (weeks 5-8), Intensification (weeks 9-12)
- Volume decreases while intensity increases
- Includes deload weeks
- Demonstrates percentage-based programming

#### b) Block Periodization (9 weeks)
**File**: `/home/nitro/Projects/pwf/examples/periodization-block.yaml`

- Three distinct 3-week mesocycles
- Block 1: Hypertrophy (high volume, moderate intensity)
- Block 2: Strength (moderate volume, high intensity)
- Block 3: Power/Peaking (low volume, very high intensity)
- Shows transition between training emphases

#### c) Daily Undulating Periodization - DUP (8 weeks)
**File**: `/home/nitro/Projects/pwf/examples/periodization-undulating.yaml`

- Varies intensity within each week
- Each main lift trained 3x/week at different rep ranges
- Power Day: 1-3 reps (85-97% 1RM)
- Strength Day: 3-6 reps (75-85% 1RM)
- Hypertrophy Day: 8-12 reps (65-75% 1RM)
- Progressive intensity increases every 2 weeks

## Key Features

### 1. Base Template + Weekly Overrides

The system uses a two-layer approach:

**Base Template (cycle.days)**: Defines the exercise selection and default parameters
```yaml
days:
  - id: "day-1"
    exercises:
      - id: "squat"
        name: "Back Squat"
        modality: strength
        target_sets: 4
        target_reps: 8
```

**Weekly Overrides (cycle.weeks)**: Specifies parameter changes per week
```yaml
weeks:
  - week_number: 1
    overrides:
      - exercise_id: "squat"
        target_sets: 5
        target_weight_percent: 75
        percent_of: "1rm"
```

### 2. Flexible Matching System

Overrides can identify exercises using:
- `exercise_id` (recommended - most reliable)
- `exercise_name` (fallback option)

And specify the day using:
- `day_id` (for named days)
- `day_order` (for ordered days)

### 3. Backward Compatibility

- Single-week plans (v1 and v2) continue to work unchanged
- The `weeks` array is optional
- Existing validation and parsing logic remains intact

## Implementation Requirements

### Code Changes Needed

#### 1. Update Type Definitions
**File**: `/home/nitro/Projects/pwf/crates/pwf-core/src/plan/types.rs`

Add three new structures:
- Modify `PlanCycle` to include `pub weeks: Option<Vec<PlanWeek>>`
- Add `PlanWeek` struct
- Add `ExerciseOverride` struct

See the complete definitions in `PERIODIZATION_IMPLEMENTATION.md`.

#### 2. Update Validator
**File**: `/home/nitro/Projects/pwf/crates/pwf-core/src/plan/validator.rs`

Add validation for:
- Weeks only allowed in v2 (PWF-P050)
- Unique week numbers (PWF-P051)
- Sequential week numbers (PWF-P052)
- Exercise identifier requirements (PWF-P053)
- Day identifier requirements (PWF-P054)
- Exercise existence validation (PWF-P055)
- Day existence validation (PWF-P056)
- Percentage loading rules (PWF-P057)
- Modality immutability (PWF-P058)

Plus warnings for best practices (PWF-P060-P064).

#### 3. Update JSON Schema
**File**: `/home/nitro/Projects/pwf/schema/pwf-v2.json`

Add schemas for:
- `Week` object
- `ExerciseOverride` object
- Update `Cycle` to include optional `weeks` array

See the complete schema in `PERIODIZATION_IMPLEMENTATION.md`.

#### 4. Write Tests
**File**: `/home/nitro/Projects/pwf/crates/pwf-core/tests/periodization_tests.rs` (new file)

Required test categories:
- Serialization/deserialization roundtrips
- Validation rule enforcement
- Integration tests with example files
- Backward compatibility tests
- Override resolution tests

### Testing Commands

```bash
# Build the project
cargo build --release

# Run all tests
cargo test --all-features --workspace

# Format code
cargo fmt --all

# Check with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Validate example plans
./target/release/pwf validate examples/periodization-linear.yaml
./target/release/pwf validate examples/periodization-block.yaml
./target/release/pwf validate examples/periodization-undulating.yaml
```

## Use Cases

### 1. Strength Sports (Powerlifting, Olympic Lifting)

- Linear progression for beginners
- Block periodization for intermediate/advanced
- Peak for competitions using taper weeks

### 2. General Fitness

- Undulating periodization to prevent plateaus
- Varied stimulus for continued adaptation
- Deload weeks for recovery

### 3. Team Sports

- Off-season strength building (high volume)
- Pre-season power development (low volume, high intensity)
- In-season maintenance (minimal volume)

### 4. Hybrid Athletes

- Combine strength blocks with conditioning blocks
- Transition between training emphases
- Manage fatigue across multiple qualities

## Benefits Over Single-Week Plans

1. **Complete Program Vision**: See the entire training plan at once
2. **Systematic Progression**: Built-in progressive overload
3. **Recovery Management**: Scheduled deload weeks
4. **Phase Transitions**: Smooth transitions between training blocks
5. **Competition Prep**: Taper and peak for specific dates
6. **Training History**: Document what was planned vs. what was completed

## Documentation Needed

### User Documentation

Create `/home/nitro/Projects/pwf/docs/periodization.md`:
- Explanation of periodization concepts
- How to create multi-week plans
- Common periodization models with code examples
- Best practices for override structure
- Troubleshooting common issues

### Specification Updates

Update `/home/nitro/Projects/pwf/docs/SPECIFICATION.md`:
- Add multi-week periodization section
- Document new fields and structures
- Provide schema references
- Include example snippets

### Block Documentation

Create `/home/nitro/Projects/pwf/docs/blocks/week.md`:
- Week block specification
- Required and optional fields
- Override matching rules
- Examples and anti-patterns

## Next Steps

### Immediate (Required for Feature Completion)

1. **Code Implementation**
   - [ ] Add new types to `types.rs`
   - [ ] Implement validation logic in `validator.rs`
   - [ ] Update JSON schema in `pwf-v2.json`

2. **Testing**
   - [ ] Write unit tests for new types
   - [ ] Write validation tests
   - [ ] Write integration tests with examples
   - [ ] Verify backward compatibility

3. **Quality Checks**
   - [ ] Run `cargo test` - all tests must pass
   - [ ] Run `cargo fmt --all` - format code
   - [ ] Run `cargo clippy` - fix all warnings
   - [ ] Validate all example files

### Future Enhancements (v2.1+)

1. **Auto-Progression Rules**: Define progression once, auto-generate weeks
2. **Week Ranges**: Apply overrides to multiple weeks (e.g., "weeks 1-3")
3. **Conditional Logic**: If-then rules for adaptive programming
4. **Load Calculations**: TSS, volume load, intensity metrics
5. **Template Library**: Pre-built periodization templates

## Files Created

All files are located in `/home/nitro/Projects/pwf/`:

1. `PERIODIZATION_IMPLEMENTATION.md` - Complete technical specification
2. `MULTI_WEEK_PERIODIZATION_SUMMARY.md` - This document
3. `examples/periodization-linear.yaml` - 12-week linear periodization example
4. `examples/periodization-block.yaml` - 9-week block periodization example
5. `examples/periodization-undulating.yaml` - 8-week DUP example

## References

### Scientific Literature

- Bompa, T. & Haff, G. (2009). *Periodization: Theory and Methodology of Training*
- Issurin, V. (2010). "New Horizons for the Methodology and Physiology of Training Periodization"
- Rhea, M. et al. (2002). "A comparison of linear and daily undulating periodized programs"
- Zourdos, M. et al. (2016). "Modified Daily Undulating Periodization Model"

### Project Documentation

- PWF Specification: `docs/SPECIFICATION.md`
- Exercise Modalities: `docs/modalities.md`
- Block Documentation: `docs/blocks/*.md`
- CLAUDE.md: Project guidelines for development

## Questions & Support

For questions about this implementation:

1. Review `PERIODIZATION_IMPLEMENTATION.md` for technical details
2. Check example files for usage patterns
3. Review validation error codes (PWF-P050-P064)
4. Consult scientific literature for periodization theory

## Design Philosophy

This implementation follows PWF's core principles:

1. **Human-Readable**: YAML structure is clear and intuitive
2. **Portable**: Plans can be shared across platforms and apps
3. **Flexible**: Supports multiple periodization models
4. **Extensible**: Foundation for future auto-progression features
5. **Validated**: Comprehensive error checking and warnings

## Conclusion

The multi-week periodization feature represents a significant enhancement to PWF v2.0, enabling coaches and athletes to create sophisticated, research-backed training programs. The implementation balances flexibility with simplicity, maintains backward compatibility, and provides a solid foundation for future enhancements.

The three example files demonstrate real-world usage patterns and serve as templates for common periodization models. Combined with comprehensive validation and documentation, this feature makes PWF a complete solution for evidence-based program design.
