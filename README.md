# Walli

A small macOS menu-bar app that rotates your desktop wallpaper from Unsplash and Bing.

## Install

Grab the latest `.dmg` from [Releases](https://github.com/SaraVieira/walli/releases/latest).

Because the build is ad-hoc signed (no Apple Developer ID yet), macOS Gatekeeper will block it on first launch. To open it:

1. Drag `Walli.app` from the DMG into `/Applications`.
2. Right-click `Walli.app` → **Open**
3. Open System Settings and go to Privacy & Security.
4. Scroll down to the Security section.
5. You should see a message stating, "“Walli” was blocked..."Click Open Anyway.
6. Enter your administrator password to confirm.

You only need to do this once; afterwards it launches normally.

## Stack

Tauri 2 · React 18 · TypeScript · Tailwind · SQLite (rusqlite)

## Develop

```sh
pnpm install
pnpm tauri dev
```

## Quality gate

```sh
pnpm check                 # typecheck + lint + prettier + vitest
cd src-tauri && cargo clippy -- -D warnings && cargo test
```

## Build

```sh
pnpm tauri build           # produces an unsigned .app and .dmg under src-tauri/target/release/bundle/
```

## Release

Push to `main` → CI bumps the patch version, tags `vX.Y.Z`, and publishes a GitHub release with the `.dmg` attached.
