# Vitalis

macOS menu bar app (Tauri v2) that shows the Mac's vital signs: CPU, GPU, SSD and battery temperatures, RAM breakdown (donut), disk usage, the apps using the most memory, and a "Limpar RAM" screen that gracefully quits selected apps. The UI is in Portuguese (pt-BR). The target machine is an Apple Silicon MacBook Air (fanless, so there is no fan data).

## Commands

```bash
bun install
bun tauri dev                 # run the app (Vite on :1420 + Rust debug build); tray icon appears in the menu bar
bun tauri build               # release .app + .dmg in src-tauri/target/release/bundle/{macos,dmg}
bun run typecheck             # tsc --noEmit
cd src-tauri && cargo test    # Rust unit tests (services tested with fake repositories)
```

- `bun run build` runs `vite build` before `tsc`. The TanStack router plugin generates `src/routeTree.gen.ts` (gitignored) during the Vite step, so `tsc` fails on a clean checkout if the order is reversed.
- Stop `tauri dev` before opening the installed app. Otherwise two tray icons appear.

## Architecture

### Backend: `src-tauri/src/` (the user's `/backend` skill, adapted to Rust)

Every feature in `app/<feature>/` (`temperature`, `memory`, `disk`, `processes`, `cleaner`, `settings`) has three layers:

- `presentation/`: `*_controller.rs` holds the `#[tauri::command]` functions. `*_dto.rs` holds serde structs (camelCase) built with `From<&Entity>`, plus input DTOs with `validate()`. Controllers contain no logic.
- `domain/`: `*_entity.rs`, `*_service.rs` (dependencies injected through the constructor as `Arc<dyn Trait>`) and `*_interfaces.rs` (repository traits). There are no FFI types in this layer.
- `infrastructure/`: `*_repository.rs` implements the traits on top of macOS APIs.

Shared pieces:

- `lib/ffi/`: all `unsafe` code and raw bindings: `smc`, `iohid`, `mach`, `libproc`, `appkit` (objc2 `msg_send!`), `cgwindow`, `spaces`, and `cf`, which provides the `CfOwned` RAII wrapper.
- `lib/config.rs`: a single `CONFIG` constant holding intervals, thresholds, limits and the always-protected bundle ids.
- `shared/`:
  - `container.rs` wires services to their repositories.
  - `errors.rs`: `AppError` serializes to `{ error, code }`.
  - `tray.rs` and `popover_state.rs` handle the tray and the popover.
  - `bootstrap.rs` runs the setup.
- `jobs/`: `vitals_worker.rs` is a thread that samples every 2s while the popover is visible and every 5s while it is hidden. `vitals_publisher.rs` emits the `vitals` event and updates the tray title (`CPU° · RAM%`).
- The crate root is `root.rs`, not `lib.rs`, because the module is named `lib` to mirror the `/backend` skill layout. Hence `#![allow(special_module_name)]`.
- File names use snake_case (`memory_service.rs`) because Rust module names can't contain dots.
- Commands that block (process scans, waiting for apps to quit) wrap their work in `tauri::async_runtime::spawn_blocking`. Synchronous commands would run on the main thread.

### Frontend: `src/` (the user's `/frontend` skill)

- Stack: React 19, TanStack Router (file-based, `createMemoryHistory`), React Query, Tailwind v4 with OKLCH tokens in `index.css`, shadcn/ui (`components/ui/`, new-york style), Motion, Recharts, Zod, Sonner.
- `domain/<entity>/{types,api,query}.ts`:
  - `api.ts` calls `api.call(CommandEnum.X, { body })`, which wraps Tauri `invoke`, inside `createWrapQuery`, and parses the result with Zod.
  - `ResponseProps` is a discriminated union, so `if (error) throw error; return data;` narrows the type.
- `hooks/use-vitals-subscription.ts` listens to the `vitals` event and pushes the data into the query cache with `setQueryData`. Queries fed this way use `staleTime: Infinity`.
- Route-local components live in `routes/**/-components/`. Use one component per file, default exports, and `classNames` for slot overrides.
- Constants, enums, memory segment colors and hatching all live in `lib/constants.ts`.

## Domain knowledge (non-obvious; verified on this Mac)

- **Temperatures:**
  - CPU = max of SMC keys `Tp*` (P-cores) and `Te*` (E-cores). GPU = `Tg*`.
  - SSD (`NAND CH0 temp`) and battery (`gas gauge battery`) come from IOHID.
  - The HID sensors `PMU tdie*`/`tdev*` belong to the power-management chip, not the CPU. They lag far behind under load (56° vs. 92° for the cores), so they are ignored.
  - SMC `TVmS` sits around 41° regardless of load and is ignored. The "Hot" app shows it at idle.
- **Memory** (matches Activity Monitor):
  - app = (internal − purgeable) × page
  - cache = (external + purgeable) × page
  - compressed = compressor pages
  - used = app + wired + compressed
  - "Disponível" = free + cache
  - Cache and free are drawn hatched and muted because they are effectively available memory.
- **Per-app memory:**
  - `ri_phys_footprint`, grouped by `responsibility_get_pid_responsible_for_pid`. This means CLIs launched from Terminal count toward Terminal.
  - `process_breakdown.rs` splits a group into own processes (inside the `.app` bundle or `.xpc` services) and launched ones.
  - It labels processes from argv. For example `node · vite`, and `claude` for the version-named binary `…/versions/2.1.311`.
- **Cleaner:**
  - Only regular apps qualify. The frontmost app, Vitalis itself, `CONFIG.cleaner.always_protected` and user-protected apps are excluded.
  - Apps are pre-selected when they are hidden or have no windows on any Space, and use more than 100 MB.
  - Apps quit via `NSRunningApplication.terminate()`. `forceTerminate()` exists only behind a second confirmation.
  - There is no `purge` or cache flushing, by design.
- **Popover:**
  - It hides on blur after `CONFIG.popover_blur_settle`, unless the active Space changed (`CGSGetActiveSpace`). In that case it stays open and takes focus back.
  - `PopoverState::was_just_hidden` stops the tray click that caused the blur from reopening it.

## Gotchas

- `CfOwned::from_create`: use `.then(|| …)`, never `.then_some(Self(ptr))`. The eager form builds a value around a null pointer and then drops it, which calls `CFRelease(NULL)` and crashes.
- shadcn `Button asChild` must pass exactly one child to `Slot`. The loader icon is only rendered when the button is not `asChild`.
- After changing a DTO shape, the Vite HMR can render new components with stale cached data. Do a full reload (`touch index.html`) before assuming a bug.
- The app identifier is `dev.joao.vitalis`. Settings live in `~/Library/Application Support/dev.joao.vitalis/settings.json` (`protectedApps`).
- Bundles are signed ad-hoc (`signingIdentity: "-"`) and not notarized. They open normally on this Mac, but other Macs need right-click → Abrir on first launch.
- When testing the cleaner, never actually quit the user's apps without asking.
