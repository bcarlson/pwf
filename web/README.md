# PWF Web Tools

Browser-based tools for validating, converting, visualizing, and building PWF (Portable Workout Format) files.

**Live Demo**: https://bcarlson.github.io/pwf/

## Features

- **Validator** - YAML validation with Monaco editor and syntax highlighting
- **Converter** - Format conversion (FIT/TCX/GPX/CSV ↔ PWF) in the browser
- **Visualizer** - GPS maps (Leaflet.js) and telemetry charts (Chart.js)
- **Plan Builder** - Visual workout plan creator with templates and sharing

All tools run client-side using WebAssembly. **No data is uploaded to servers.**

## Technology Stack

- **Framework**: Svelte 5 + TypeScript
- **Validation**: WASM (compiled from Rust `pwf-core`)
- **Editor**: Monaco Editor (VS Code editor component)
- **Maps**: Leaflet.js with OpenStreetMap tiles
- **Charts**: Chart.js
- **Build**: Vite
- **Testing**: Vitest + Testing Library
- **Styling**: CSS Variables with dark mode support

## Development Setup

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ (for WASM compilation)
- wasm-pack (`cargo install wasm-pack`)

### Installation

```bash
# From the web directory
cd web

# Install dependencies
npm install

# Build WASM module
npm run build:wasm

# Start development server
npm run dev
```

The dev server will run at http://localhost:5173

### Project Structure

```
web/
├── src/
│   ├── components/
│   │   ├── builder/          # Visual plan builder
│   │   │   ├── steps/        # Wizard steps
│   │   │   ├── forms/        # Exercise/meta forms
│   │   │   ├── shared/       # Shared builder components
│   │   │   └── utils/        # YAML generation, imports
│   │   ├── converter/        # Format converter UI
│   │   ├── validator/        # YAML validator UI
│   │   └── visualizer/       # GPS/chart visualizations
│   ├── lib/
│   │   ├── builderState.ts   # Plan builder state management
│   │   ├── customTemplates.ts # Template CRUD (localStorage)
│   │   ├── shareUtils.ts     # Plan sharing/compression
│   │   ├── stores.ts         # Global app state
│   │   ├── wasm.ts           # WASM module loader
│   │   └── workoutTemplates.ts # Pre-built templates
│   ├── App.svelte            # Main app with routing
│   ├── main.ts               # Entry point
│   └── styles.css            # Global styles
├── public/                   # Static assets
├── package.json
├── vite.config.ts            # Vite configuration
└── vitest.config.ts          # Test configuration
```

## Build Commands

```bash
# Development server with hot reload
npm run dev

# Build WASM module from Rust
npm run build:wasm

# Build for production
npm run build:web

# Build everything (WASM + web)
npm run build

# Preview production build
npm run preview

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Type check
npm run check
```

## Testing

```bash
# Run all tests
npm test

# Watch mode
npm test -- --watch

# Coverage report
npm run test:coverage

# Test specific file
npm test -- src/lib/__tests__/shareUtils.test.ts
```

**Test Coverage**: 96% (37 tests)

### Test Files

- `src/lib/__tests__/shareUtils.test.ts` - URL sharing and compression
- `src/lib/__tests__/customTemplates.test.ts` - Template CRUD operations

## Architecture

### State Management

**Global State** (`src/lib/stores.ts`):
- `wasmReady` - WASM module load status
- `currentTab` - Active tab (validate/convert/visualize/builder)
- `darkMode` - Theme preference
- `validationResult` - Current validation state

**Builder State** (`src/lib/builderState.ts`):
- Svelte store for plan builder
- Immutable updates for reactivity
- Methods: `updateMeta()`, `addDay()`, `updateExercise()`, `reorderDays()`, etc.

### WASM Integration

The `pwf-core` Rust library is compiled to WebAssembly and provides:
- Plan validation (`validatePlan()`)
- History validation (`validateHistory()`)
- Format conversion (exposed via converter panel)

```typescript
import { loadWasm, validatePlan } from './lib/wasm';

await loadWasm(); // Load WASM module
const result = validatePlan(yamlString); // Validate plan
```

### Routing

Hash-based routing for GitHub Pages compatibility:
- `#/` - Validator (default)
- `#/builder` - Plan builder
- `#/builder?plan=<compressed>` - Shared plan

### Plan Sharing

Plans are shared via URL using LZ-String compression:
1. Serialize PlanDraft to JSON
2. Compress with `compressToEncodedURIComponent()`
3. Append to URL hash: `#/builder?plan=N4Ig...`
4. On load, decompress and validate

### Custom Templates

Stored in browser localStorage (`pwf_custom_templates`):
- CRUD operations in `src/lib/customTemplates.ts`
- Displayed alongside pre-built templates
- Exportable as YAML

## Deployment

### GitHub Pages

Deployment is automated via `.github/workflows/deploy-web.yaml`:

```bash
# Triggered on push to main
git push origin main

# Manual deployment
npm run build
# Upload dist/ to GitHub Pages
```

**Base Path**: `/pwf/` (configured in `vite.config.ts`)

### Environment

- **Production URL**: https://bcarlson.github.io/pwf/
- **CDN**: None (self-hosted on GitHub Pages)
- **Analytics**: None (privacy-first)

## Contributing

### Code Style

- Use TypeScript for type safety
- Follow existing component patterns
- Use Svelte stores for state management
- Keep components small and focused
- Write tests for new utilities

### Adding a New Template

Edit `src/lib/workoutTemplates.ts`:

```typescript
export const templates: WorkoutTemplate[] = [
  {
    id: 'my-template',
    name: 'My Template',
    category: 'strength',
    difficulty: 'beginner',
    description: 'Description here',
    plan: {
      plan_version: 1,
      meta: { name: 'My Plan' },
      cycle: { days: [...] }
    }
  }
];
```

### Adding a New Modality

1. Update `src/lib/builderState.ts` - Add to `Modality` type
2. Update `src/components/builder/forms/ExerciseForm.svelte` - Add form fields
3. Update `src/components/builder/utils/yamlGenerator.ts` - Handle serialization
4. Add tests

## Troubleshooting

### WASM Build Fails

```bash
# Install wasm-pack
cargo install wasm-pack

# Clean and rebuild
rm -rf ../target/wasm32-unknown-unknown
npm run build:wasm
```

### Dev Server Errors

```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Tests Fail

```bash
# Update snapshots
npm test -- -u

# Check coverage
npm run test:coverage
```

## Performance

- **Initial Load**: ~2.5MB (includes WASM)
- **Subsequent Loads**: <100KB (cached)
- **WASM Load Time**: ~200ms
- **Service Worker**: Caches all assets for offline use

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari 14+, Chrome Android)

**Requirements**: WebAssembly support

## License

MIT License - See [LICENSE](../LICENSE)

---

**Live Demo**: https://bcarlson.github.io/pwf/
**Main Project**: https://github.com/bcarlson/pwf
