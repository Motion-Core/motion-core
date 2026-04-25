# Changelog

All notable changes to **Motion Core** will be documented in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.11.1] - 2026-04-25

### Fixed

- **Canvas / Runtime**: Fixed live prop updates for OGL shader uniforms by avoiding deeply proxied Svelte state for runtime uniform objects.
- **Canvas / Infinite Gallery**: Updated `visibleCount` changes to rebuild gallery planes inside the existing WebGL runtime instead of requiring a component remount.

## [0.11.0] - 2026-04-16

### Changed

- **Canvas / ASCII Renderer**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Card 3D**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Dithered Image**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Fake 3D Image**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Fluid Image Reveal**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Fluid Simulation**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Glass Pane**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Glass Slideshow**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Glitter Cloth**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Globe**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / God Rays**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Halo**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Infinite Gallery**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Interactive Grid**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Lava Lamp**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Neural Noise**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Pixelated Image**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Plasma Grid**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Rubiks Cube**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Specular Band**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Water Ripple**: Migrated rendering pipeline from Three.js/Threlte to OGL.
- **Canvas / Helpers**: Added shared `color` and `fluid-pointer` helpers for unified color conversion and pointer handling.
- **Canvas / Defaults**: Standardized default `backgroundColor` to `#17181A` and aligned default prop values across migrated OGL components.
- **Showcase / Card Stack**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Pointer / Flip Card Stack**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Layout / Flip Grid**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Navigation / Floating Menu**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Canvas / Glass Slideshow**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Canvas / Globe**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Pointer / Image Trail**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Infinite Physics Gallery**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Logo Carousel**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Pointer / MacOS Dock**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Pointer / Magnetic**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Marquee**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Transition / Preloader**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Radial Gallery**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Slideshow**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Split Hover**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Split Reveal**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Stacking Words**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Text Loop**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Text Scramble**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Video Player**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Showcase / Video Slider**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Typography / Weight Wave**: Switched runtime GSAP imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).
- **Helpers / GSAP**: Switched helper imports from `gsap/dist/*` to package entry points (`gsap`, `gsap/<plugin>`).

### Removed

- **Runtime / Dependencies**: Removed `three`, `@threlte/core`, `@threlte/extras`, and `@types/three` from `motion-core` package dependencies and switched to `ogl`.

## [0.10.0] - 2026-04-04

### Added

- **1 component**:
  - **Canvas**: Fluid Image Reveal.

## [0.9.0] - 2026-04-03

### Added

- **1 component**:
  - **Canvas**: Fake 3D Image.

## [0.8.1] - 2026-03-31

### Changed

- **Canvas / Globe**: Switched land/water source of truth from `ne_110m_land.geojson` to COBE-style texture mask sampling (`land-texture.png`) while keeping the existing Three/Threlte instanced-mesh rendering pipeline.

## [0.8.0] - 2026-03-29

### Added

- **1 component**:
  - **Showcase**: Infinite Physics Gallery.

## [0.7.1] - 2026-03-29

### Fixed

- **Canvas / Globe**: Simplified `GlobeMarker` API by removing redundant `headRadius` and using `size` as the single marker size field (including docs/registry updates).

## [0.7.0] - 2026-03-28

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
- **Typography / Stacking Words**: Replaced `bind:this` with `{@attach ...}` for wrapper ref lifecycle.
- **Pointer / Image Trail**: Replaced `bind:this` with `{@attach ...}` for container ref lifecycle.
- **Pointer / Card3D Face Tracker**: Replaced `bind:this` with `{@attach ...}` for video ref lifecycle.
- **Typography / Split Hover**: Replaced `bind:this` with `{@attach ...}` for wrapper/span ref lifecycle.
- **Showcase / Slideshow**: Replaced `bind:this` with `{@attach ...}` for slide/image refs.
- **Showcase / Video Slider**: Replaced `bind:this` with `{@attach ...}` for slider ref lifecycle.
- **Pointer / MacOS Dock**: Replaced `bind:this` with `{@attach ...}` for dock item and tooltip refs.
- **Showcase / Video Player**: Replaced `bind:this` with `{@attach ...}` for player/control/icon refs.
- **Transition / Preloader**: Replaced `bind:this` with `{@attach ...}` for container and staged image refs.
- **Navigation / Floating Menu**: Replaced `bind:this` with `{@attach ...}` for overlay/container/toggle refs.
- **Canvas / ASCII Renderer**: Replaced effect-driven metric assignments with `$derived` values for canvas/image sizing.
- **Canvas / Globe**: Replaced effect-driven derived state (`filteredPositions`, atmosphere scale/config) with `$derived` computations.
- **Canvas / Globe**: Reworked markers from pin meshes to surface-aligned point markers (no pulse animation) with updated default marker size.
- **Canvas / Globe**: Added `markerTooltip` snippet API and exported `GlobeMarkerTooltipContext` for custom tooltip rendering.
- **Canvas / Globe**: Moved tooltip hide/show behavior fully inside the component (occlusion-driven opacity/blur with custom easing and earlier hide threshold).
- **Canvas / Globe**: Adjusted depth/render ordering so marker color is not washed out by additive land dots.

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
