# PWF + OwnLift Integration Analysis

This document analyzes what changes are needed in both PWF and OwnLift for full bidirectional compatibility.

## Current State Summary

| Capability | PWF Spec | OwnLift Implementation |
|------------|----------|------------------------|
| Plan Import | v1.0 | Full support |
| Plan Export | v1.0 | Full support |
| History Export | v1.0 | **Not implemented** |
| History Import | v1.0 | **Not implemented** |
| PR Detection | Basic | 6 types (richer) |
| RPE Tracking | Planned v1.1 | Already implemented |
| Equipment Tags | Supported | **Not stored** |

**Bottom line:** Plan exchange works. History exchange does not.

---

## Part 1: What PWF Needs (to support OwnLift's features)

### 1.1 RPE/RIR Support (v1.1 - Already Planned)

OwnLift tracks RPE (1-10) on every set. PWF should add:

```yaml
# In CompletedSet
sets:
  - reps: 5
    weight_kg: 100
    rpe: 8.5        # Rate of Perceived Exertion (0-10)
    rir: 2          # Reps in Reserve (optional alternative)
```

**Recommendation:** Add both `rpe` and `rir` fields. Apps can use either.

### 1.2 Warmup Set Type

OwnLift distinguishes warmup vs working sets. PWF has `set_type` but it's optional.

**Current PWF:**
```yaml
set_type: working  # working, warmup, dropset, failure, amrap
```

**Recommendation:** Keep current spec. OwnLift's `isWarmup: boolean` maps to `set_type: warmup`.

### 1.3 Extended PR Types

OwnLift tracks 6 PR types:
- `1rm` - Estimated one-rep max (Epley formula)
- `volume` - Session volume per exercise
- `rep_3`, `rep_5`, `rep_8`, `rep_10` - Max weight at rep count

**Current PWF** `record_type`:
- `1rm`, `max_weight`, `max_reps`, `max_volume`, `max_duration`, `max_distance`, `fastest_time`

**Gap:** PWF has `max_weight` but not rep-specific maxes.

**Recommendation:** Add rep-specific record types:
```yaml
record_type:
  - 1rm
  - max_weight        # Absolute max (any reps)
  - max_weight_3rm    # Max at 3 reps
  - max_weight_5rm    # Max at 5 reps
  - max_weight_8rm    # Max at 8 reps
  - max_weight_10rm   # Max at 10 reps
  - max_reps
  - max_volume
  - max_duration
  - max_distance
  - fastest_time
```

### 1.4 Set Timestamp Tracking

OwnLift records when each set was completed.

**Current PWF:** `completed_at` field exists but is optional.

**Recommendation:** Keep optional. Apps that track this can export it.

### 1.5 Session Duration Tracking

OwnLift stores `started_at` and `ended_at` for sessions.

**Current PWF:** Already supports these fields.

**Status:** Aligned

### 1.6 Plan Activation/Status Tracking

OwnLift tracks plan status: `draft`, `active`, `completed`, `archived`.

**Gap:** PWF doesn't include plan status in exports.

**Recommendation:** Add optional `status` to plan meta for exports:
```yaml
meta:
  title: "My Plan"
  status: active  # draft, active, completed, archived
  activated_at: "2025-01-01T00:00:00Z"
  completed_at: "2025-01-15T00:00:00Z"
```

### 1.7 Weight Unit Preference

OwnLift stores both value and unit for weights.

**Current PWF:** Has both `weight_kg` and `weight_lb` fields.

**Recommendation:** Keep both. Add `preferred_unit` to export source:
```yaml
export_source:
  app_name: "OwnLift"
  preferred_units:
    weight: kg  # or lb
    distance: meters
```

---

## Part 2: What OwnLift Needs (to fully support PWF)

### 2.1 PWF History Export (Critical)

**Gap:** OwnLift cannot export workout history in PWF format.

**Required Implementation:**

1. Create `src/utils/exportGenerators/pwfHistoryExport.ts`
2. Serialize completed workouts with:
   - Full set details (weight, reps, RPE, duration, distance)
   - Session metadata (date, duration, notes)
   - Plan references (if from plan)
3. Include personal records
4. Include body measurements
5. Add "Export as PWF" option to export screen

**Example Output:**
```yaml
history_version: 1
exported_at: "2025-01-15T10:30:00Z"

export_source:
  app_name: "OwnLift"
  app_version: "1.2.0"
  platform: "ios"

workouts:
  - date: "2025-01-15"
    started_at: "2025-01-15T09:00:00Z"
    ended_at: "2025-01-15T10:15:00Z"
    title: "Push Day"
    plan_id: "beginner-strength-v1"
    exercises:
      - name: "Bench Press"
        modality: strength
        sets:
          - reps: 5
            weight_kg: 100
            set_type: working
            rpe: 8
            completed_at: "2025-01-15T09:15:00Z"

personal_records:
  - exercise_name: "Bench Press"
    record_type: 1rm
    value: 120.5
    achieved_at: "2025-01-15"
```

### 2.2 Equipment Tag Storage

**Gap:** OwnLift doesn't store `equipment` from plan imports.

**Required Changes:**

1. Add `equipment` column to `plans` table (JSON array or separate table)
2. Parse and store equipment from PWF imports
3. Export equipment in plan serialization
4. Optional: Show equipment requirements in plan browser

**Schema Change:**
```typescript
// src/db/schema.ts
equipment: text('equipment'), // JSON array: '["barbell","dumbbells"]'
```

### 2.3 daysPerWeek Metadata

**Gap:** OwnLift doesn't store or calculate this.

**Required Changes:**

1. Add `daysPerWeek` column to `plans` table
2. Parse from PWF imports (or calculate from day count)
3. Export in plan serialization

### 2.4 recommendedFirst Flag

**Gap:** Used for plan library ordering but not stored.

**Required Changes:**

1. Add `recommendedFirst` boolean to `plans` table
2. Parse from PWF imports
3. Use for sorting in plan browser

### 2.5 target_load as Separate Field

**Gap:** OwnLift stores loading guidance in `targetNotes`.

**Current PWF:**
```yaml
target_load: "225 lbs"   # Separate field
target_notes: "Pause on chest"
cues: "Form notes"       # Alias for target_notes
```

**Options:**

A) **Add dedicated column** (breaking change):
```typescript
targetLoad: text('target_load'),
targetNotes: text('target_notes'),
```

B) **Parse from targetNotes** (non-breaking):
- On import, concatenate `target_load` + `cues` into `targetNotes`
- On export, use heuristics to extract load patterns

**Recommendation:** Option B for backwards compatibility.

### 2.6 PWF History Import

**Gap:** Cannot import PWF history exports from other apps.

**Required Implementation:**

1. Create `src/utils/plan-contract/historyParser.ts`
2. Parse PWF history format
3. Map to OwnLift schema:
   - `workouts[]` → `workout_sessions` + `setEntries`
   - `personal_records[]` → `personalRecords`
   - `body_measurements[]` → `bodyMeasurements`
4. Handle conflicts (duplicate dates, existing PRs)
5. Add "Import History" option

---

## Part 3: Implementation Priority

### Phase 1: Core History Export (High Priority)

Enable OwnLift users to export their data in PWF format.

| Task | Effort | Files |
|------|--------|-------|
| Create PWF history serializer | Medium | `pwfHistoryExport.ts` |
| Add export option to UI | Low | Export screen |
| Include PRs in export | Low | Query + serialize |
| Include measurements in export | Low | Query + serialize |

### Phase 2: Plan Metadata (Medium Priority)

Store additional PWF metadata for richer plan management.

| Task | Effort | Files |
|------|--------|-------|
| Add equipment column | Low | Schema migration |
| Add daysPerWeek column | Low | Schema migration |
| Add recommendedFirst column | Low | Schema migration |
| Update parser | Low | `parser.ts` |
| Update serializer | Low | `serializer.ts` |

### Phase 3: History Import (Lower Priority)

Allow importing history from other PWF-compatible apps.

| Task | Effort | Files |
|------|--------|-------|
| Create history parser | Medium | `historyParser.ts` |
| Conflict resolution UI | Medium | New screen |
| Map to OwnLift schema | Medium | Transformation logic |

### Phase 4: PWF Spec Updates

Propose changes to PWF spec based on OwnLift learnings.

| Proposal | PWF Version |
|----------|-------------|
| RPE/RIR fields | v1.1 |
| Rep-specific PR types | v1.1 |
| Plan status in exports | v1.1 |
| Preferred units in export_source | v1.1 |

---

## Part 4: Data Mapping Reference

### Plan Import: PWF → OwnLift

| PWF Field | OwnLift Table.Column |
|-----------|---------------------|
| `meta.id` | `plans.id` |
| `meta.title` | `plans.title` |
| `meta.description` | `plans.description` |
| `meta.author` | `plans.author` |
| `meta.equipment` | **Not stored** |
| `meta.daysPerWeek` | **Not stored** |
| `meta.recommendedFirst` | **Not stored** |
| `meta.tags` | **Not stored** |
| `cycle.days[].focus` | `planDays.focus` |
| `cycle.days[].order` | `planDays.orderIndex` |
| `cycle.days[].exercises[].modality` | `planDayExercises.modality` |
| `cycle.days[].exercises[].target_load` | `planDayExercises.targetNotes` |
| `cycle.days[].exercises[].link` | `planDayExercises.link` |
| `cycle.days[].exercises[].image` | `planDayExercises.localImagePath` (downloaded) |

### History Export: OwnLift → PWF

| OwnLift Table.Column | PWF Field |
|---------------------|-----------|
| `workout_sessions.date` | `workouts[].date` |
| `workout_sessions.notes` | `workouts[].notes` |
| `setEntries.exerciseName` | `workouts[].exercises[].name` |
| `setEntries.weight` + `weightUnit` | `sets[].weight_kg` or `weight_lb` |
| `setEntries.reps` | `sets[].reps` |
| `setEntries.rpe` | `sets[].rpe` |
| `setEntries.isWarmup` | `sets[].set_type: warmup` |
| `setEntries.durationSec` | `sets[].duration_sec` |
| `setEntries.distanceMeters` | `sets[].distance_meters` |
| `personalRecords.exerciseName` | `personal_records[].exercise_name` |
| `personalRecords.prType` | `personal_records[].record_type` |
| `personalRecords.value` | `personal_records[].value` |
| `personalRecords.achievedAt` | `personal_records[].achieved_at` |
| `bodyMeasurements.weight` | `body_measurements[].weight_kg` |
| `bodyMeasurements.bodyFatPercent` | `body_measurements[].body_fat_percent` |

---

## Part 5: Validation Considerations

### Pre-Export Validation

Before exporting PWF history, validate:

1. All exercises have names (not empty strings)
2. Dates are valid ISO 8601
3. Weights are positive numbers
4. RPE values are 0-10 range

### Post-Import Validation

After importing PWF data, validate:

1. No duplicate sessions on same date (or merge)
2. PR values don't conflict with existing records
3. Exercise names are normalized (trim whitespace)

---

## Conclusion

**Immediate wins:**
1. OwnLift adds PWF history export → Users can take their data anywhere
2. PWF adds RPE field → Matches OwnLift's existing capability

**Medium-term:**
1. OwnLift stores full PWF metadata → Better plan library features
2. PWF adds rep-specific PR types → Matches OwnLift's tracking

**Long-term:**
1. Full bidirectional history sync between PWF-compatible apps
2. PWF becomes the "OpenAPI of fitness data"
