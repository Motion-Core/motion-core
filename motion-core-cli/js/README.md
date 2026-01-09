# @motion-core/cli

The official command-line interface for [Motion Core](https://motion-core.dev/), a Svelte library for high-quality animations and motion components.

This package serves as a Node.js wrapper around the native Rust binary, ensuring optimal performance while remaining easily installable via your favorite package manager.

## Installation

You can run the CLI directly using `npx`, `bunx`, or `pnpm dlx` without installation:

```bash
npx @motion-core/cli init
```

Or install it globally:

```bash
# using npm
npm install -g @motion-core/cli

# using pnpm
pnpm add -g @motion-core/cli

# using bun
bun add -g @motion-core/cli

# using yarn
yarn global add @motion-core/cli
```

## Usage

Once installed, you can use the `motion-core` command to scaffold projects, add components, and manage the local cache.

### `init`

Initialize a new Motion Core configuration in your current project. This command detects your framework and sets up the necessary file structure.

```bash
motion-core init [options]
```

**Options:**

- `--dry-run`: Preview the initialization process without writing any files or changes.

### `add`

Add a component (and its dependencies) to your project.

```bash
motion-core add <component-names> [options]
```

**Examples:**

```bash
motion-core add glass-pane
motion-core add glass-pane image-trail
```

**Options:**

- `--dry-run`: Preview the installation plan (files to create/update, dependencies to install) without applying changes.
- `-y, --yes`: Skip confirmation prompts. Useful for CI/CD environments.
- When a component file already exists locally, the CLI now shows a colored diff and asks whether to overwrite or keep your local changes. Passing `-y` (or setting `MOTION_CORE_CLI_ASSUME_YES=1`) will automatically overwrite without prompting, while dry-run mode reports all conflicts without touching disk.

### `list`

List all available components in the registry.

```bash
motion-core list [options]
```

**Options:**

- `--json`: Output the registry data in JSON format instead of a human-readable table.

### `cache`

Manage the local cache used to store registry data and component assets.

```bash
motion-core cache [options]
```

**Options:**

- `--clear`: Prepare to clear cached registry data and assets. **Must be used with `--force` to perform the deletion.**
- `--force`: Confirm the deletion of cached files.

## How it Works

This package identifies your operating system and CPU architecture (Windows, macOS, Linux / x64, arm64) and delegates execution to the appropriate pre-compiled Rust binary. This approach combines the raw performance of native code with the convenience of Node.js package distribution.

## Development

If you are contributing to the CLI or building it locally:

1.  **Prerequisites**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) and Node.js installed.

2.  **Build the Rust binary**:
    From the `motion-core-cli` root directory, build the binary for your current platform:

    ```bash
    cargo build --release
    ```

    _Note: You may need to specify a target if cross-compiling._

3.  **Place the binary**:
    The Node.js wrapper expects the binary to be located in `dist/<target-triple>/`.

    For example, on an Apple Silicon Mac:

    ```bash
    mkdir -p js/dist/aarch64-apple-darwin
    cp target/release/motion-core js/dist/aarch64-apple-darwin/
    ```

4.  **Run the wrapper**:
    ```bash
    cd js
    bun link # or npm link
    motion-core --help
    ```

## License

MIT
