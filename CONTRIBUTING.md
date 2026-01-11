# Contributing to Motion Core

First off, thank you for considering contributing to Motion Core! It's people like you who make this project better for everyone.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

Motion Core is a monorepo managed by [Nx](https://nx.dev) and [Bun](https://bun.sh). It contains the core library, documentation, and a Rust-based CLI.

### Prerequisites

- **Bun** (v1.3.3+) - Required for workspace management.
- **Rust** (Latest) - Required for CLI development.
- **Node.js** - Required for some internal tooling.

### Installation

1. Fork and clone the repository.
2. Install dependencies:
   ```bash
   bun install
   ```
3. Bootstrap the workspace:
   ```bash
   bun run workspace:bootstrap
   ```

## Development Workflow

### Working on Components

Most contributions will happen in `packages/motion-core`. We use SvelteKit (in `apps/docs`) as a development playground.

1. Start the dev server:
   ```bash
   bun run dev
   ```
2. Navigate to `http://localhost:5173`.
3. Modify components in `packages/motion-core/src/lib/components`.

### Adding a New Component

1. Create a new directory in `packages/motion-core/src/lib/components/[slug]`.
2. Add your `.svelte` component (using Svelte 5 Runes).
3. Create a `component.json` file in the same directory. Use the existing components as a reference.
4. Run the registry builder to update the manifest:
   ```bash
   bun run workspace:bootstrap
   ```
5. Add a demo page in `apps/docs/src/routes/docs/[slug]/`.

### Working on the CLI

The CLI is written in Rust and located in `motion-core-cli`.

- `bun run cli:check`: Check for compilation errors.
- `bun run cli:test`: Run Rust tests.
- `bun run cli:build`: Build the binary.

## Coding Standards

- **Svelte 5**: Use Runes (`$state`, `$derived`, `$effect`) and Snippets. Avoid legacy Svelte 4 syntax.
- **Styling**: Use Tailwind CSS. For dynamic class merging, use the `cn` utility.
- **Animations**: Prefer **GSAP** for complex timelines and **Threlte** for WebGL/3D.
- **TypeScript**: All code must be strictly typed.

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New component or feature.
- `fix:` Bug fix.
- `docs:` Documentation changes.
- `style:` Formatting, missing semi colons, etc.
- `refactor:` Code change that neither fixes a bug nor adds a feature.
- `perf:` Code change that improves performance.
- `test:` Adding missing tests.
- `chore:` Changes to the build process or auxiliary tools.

## Pull Request Process

1. Create a new branch from `main`.
2. Ensure your code passes linting and formatting:
   ```bash
   bun run lint && check && format
   ```
3. If you added a component, ensure it has a proper `component.json` and a demo in the docs.
4. Submit a PR with a clear description of the changes.
5. Once approved, a maintainer will merge your PR.

## Licensing

By contributing to Motion Core, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
