# Changelog

All notable changes to **Motion Core** will be documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- **Showcase / Card Stack**: Replaced `bind:this` with `{@attach ...}` for container ref management.
- **Pointer / Magnetic**: Replaced `bind:this` with `{@attach ...}` for element ref lifecycle.
- **Showcase / Marquee**: Replaced `bind:this` with `{@attach ...}` for container ref lifecycle.
- **Layout / Flip Grid**: Replaced `bind:this` with `{@attach ...}` for container ref lifecycle.
- **Canvas / Interactive Grid**: Replaced `bind:this` with `{@attach ...}` for container ref lifecycle.
- **Showcase / Radial Gallery**: Replaced `bind:this` with `{@attach ...}` for container ref lifecycle.
- **Typography / Split Reveal**: Replaced `bind:this` with `{@attach ...}` for wrapper ref lifecycle.
- **Typography / Weight Wave**: Replaced `bind:this` with `{@attach ...}` for wrapper ref lifecycle.
- **Typography / Text Scramble**: Replaced `bind:this` with `{@attach ...}` for wrapper ref lifecycle.

## [0.6.0] - 2026-03-28

### Added

- **1 component**:
  - **Pointer**: Flip Card Stack.

- **GSAP helpers**:
  - Added `registerPluginOnce`.
  - Added `ensureMotionCoreEase`.
  - Included helper manifests for GSAP-based components.

### Changed

- **Scroll behavior**: Normalized `scrollElement` scroller resolution in:
  - **Showcase**: Card Stack, Marquee.
  - **Typography**: Split Reveal, Stacking Words.

## [0.5.1] - 2026-03-25

### Added

- **1 component**:
  - **Typography**: Stacking Words.

### Fixed

- **GSAP imports**: Standardized runtime GSAP imports to `gsap/dist/gsap` in:
  - **Layout**: Flip Grid.
  - **Canvas**: Globe.
  - **Showcase**: Marquee.
  - **Showcase**: Radial Gallery.

- **Lifecycle**: Proper cleanups/kills in:
  - **Transition / Preloader**: Kills GSAP timeline on component destroy.
  - **Canvas / Globe**: Cancels previous camera focus tween and cleans up tween lifecycle.
  - **Pointer / MacOS Dock**: Cleans up GSAP tweens on effect rerun and component destroy.
  - **Showcase / Slideshow**: Cleans up active GSAP timeline on destroy.
  - **Showcase / Video Player Slider**: Guards refs and cleans up GSAP tweens on effect cleanup.

## [0.5.0] - 2026-03-18

### Added

- **3 components**:
  - **Canvas**: God Rays.
  - **Canvas**: Specular Band.
  - **Canvas**: Halo.

## [0.4.1] - 2026-03-15

### Fixed

- **Video Player**: Added accessible slider semantics.

### Changed

- **Design Tokens**: Naming convention of the background colors.

## [0.4.0] - 2026-02-21

### Added

- **1 component**:
  - **Canvas**: Glitter Cloth.

## [0.3.0] - 2026-01-18

### Added

- **1 component**:
  - **Showcase**: Video Player.

## [0.2.0] - 2026-01-16

### Added

- **2 components**:
  - **Canvas**: Fluid Simulation.
  - **Showcase**: Card Stack.

## [0.1.0] - 2026-01-11

### Added

- Initial release of **Motion Core**, a high-performance Svelte 5 animation library.
- Monorepo structure managed by Nx and Bun.
- Registry System allowing for individual component installation via CLI.
- Documentation app built with SvelteKit featuring interactive demos.
- **28 components**:
  - **Canvas**: ASCII Renderer, Dithered Image, Glass Pane, Glass Slideshow, Globe, Infinite Gallery, Interactive Grid, Lava Lamp, Neural Noise, Pixelated Image, Plasma Grid, Rubik's Cube, Water Ripple.
  - **Showcase**: Logo Carousel, Marquee, Radial Gallery, Slideshow.
  - **Typography**: Split Hover, Split Reveal, Text Loop, Text Scramble, Weight Wave.
  - **Pointer**: Image Trail, MacOS Dock, Magnetic.
  - **Navigation**: Floating Menu.
  - **Layout**: Flip Grid.
  - **Transition**: Preloader.
