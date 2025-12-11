# Equipment Tags

Equipment tags identify what gear is needed to complete a workout plan.

## Standard Tags

Use these standardized tags in the `meta.equipment` array for consistent display across applications.

| Tag | Display Label | Description |
|-----|---------------|-------------|
| `barbell` | Barbell | Olympic or standard barbell |
| `dumbbells` | Dumbbells | Adjustable or fixed dumbbells |
| `kettlebell` | Kettlebell | Any size kettlebell |
| `pullup_bar` | Pull-up Bar | Pull-up bar or doorway bar |
| `bench` | Bench | Flat or adjustable weight bench |
| `cables` | Cables | Cable machine or pulley system |
| `bands` | Resistance Bands | Loop or tube resistance bands |
| `bodyweight` | Bodyweight Only | No equipment required |
| `machine` | Machine | Generic gym machine |

## Usage

```yaml
meta:
  title: "Full Body Strength"
  equipment: [barbell, dumbbells, bench, pullup_bar]
```

## Display Behavior

Applications should:

1. Display equipment tags as user-friendly labels
2. Filter plans by available equipment
3. Show equipment icons where appropriate

### Example Rendering

```
Equipment: Barbell • Dumbbells • Bench • Pull-up Bar
```

## Custom Tags

Custom equipment tags are allowed but may not be recognized by all applications:

```yaml
meta:
  equipment: [barbell, squat_rack, leg_press_machine]
```

> **Note:** Prefer standard tags when possible. Custom tags like `squat_rack` or `leg_press_machine` may display as plain text without icons.

## No Equipment

For bodyweight-only plans, either:

1. Use the `bodyweight` tag:
   ```yaml
   meta:
     equipment: [bodyweight]
   ```

2. Or leave equipment empty:
   ```yaml
   meta:
     equipment: []
   ```

Both are valid. Using `bodyweight` makes the intent explicit.

## Equipment Combinations

Common equipment combinations:

### Home Gym (Minimal)

```yaml
equipment: [dumbbells, bands]
```

### Home Gym (Full)

```yaml
equipment: [barbell, dumbbells, bench, pullup_bar, bands]
```

### Commercial Gym

```yaml
equipment: [barbell, dumbbells, bench, pullup_bar, cables, machine]
```

### Kettlebell Only

```yaml
equipment: [kettlebell]
```

### No Equipment

```yaml
equipment: [bodyweight]
# or
equipment: []
```

## Future Tags (v1.1)

Under consideration for standardization:

| Proposed Tag | Description |
|--------------|-------------|
| `squat_rack` | Squat rack or power cage |
| `rings` | Gymnastic rings |
| `box` | Plyo box |
| `medicine_ball` | Medicine ball or slam ball |
| `foam_roller` | Foam roller |
| `trx` | Suspension trainer |
| `rower` | Rowing machine |
| `bike` | Stationary bike or assault bike |
| `treadmill` | Treadmill |
