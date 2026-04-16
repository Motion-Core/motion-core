<script lang="ts">
	import { onMount } from "svelte";
	import {
		Camera,
		Mesh,
		Program,
		Renderer,
		Texture,
		Transform,
		Triangle,
		Vec2,
		Vec3,
	} from "ogl";
	import { gsap } from "gsap/dist/gsap";
	import type { Snippet } from "svelte";
	import landTextureUrl from "../../assets/land-texture.png";
	import { type ColorRepresentation, toLinearRgb } from "../../helpers/color";
	import type { GlobeMarker, GlobeMarkerTooltipContext } from "./types";
	import GlobeMarkerItem from "./GlobeMarkerItem.svelte";

	interface FresnelConfig {
		/**
		 * Base body color for the globe surface.
		 * @default "#111113"
		 */
		color?: ColorRepresentation;
		/**
		 * Accent color applied by the Fresnel rim.
		 * @default "#FF6900"
		 */
		rimColor?: ColorRepresentation;
		/**
		 * Controls how tight the Fresnel rim hug is.
		 * Higher values yield a thinner outline.
		 * @default 6
		 */
		rimPower?: number;
		/**
		 * Intensity multiplier for the Fresnel rim color.
		 * @default 1.5
		 */
		rimIntensity?: number;
	}

	interface AtmosphereConfig {
		/**
		 * Color of the atmosphere glow.
		 * @default "#FF6900"
		 */
		color?: ColorRepresentation;
		/**
		 * Size of the atmosphere relative to the globe radius.
		 * @default 1.1
		 */
		scale?: number;
		/**
		 * Falloff power of the glow. Higher values mean a sharper edge.
		 * @default 12.0
		 */
		power?: number;
		/**
		 * Base coefficient for the intensity calculation.
		 * Controls how far the glow extends inwards.
		 * @default 0.9
		 */
		coefficient?: number;
		/**
		 * Global intensity multiplier.
		 * @default 2.0
		 */
		intensity?: number;
	}

	interface Props {
		/**
		 * Radius of the sphere.
		 * @default 2
		 */
		radius: number;
		/**
		 * Optional overrides for the Fresnel shader uniforms.
		 */
		fresnelConfig?: FresnelConfig;
		/**
		 * Optional configuration for the atmospheric halo.
		 */
		atmosphereConfig?: AtmosphereConfig;
		/**
		 * Number of points rendered along the globe surface.
		 * @default 15000
		 */
		pointCount?: number;
		/**
		 * Size of each point in world units.
		 * @default 0.05
		 */
		pointSize?: number;
		/**
		 * Color applied to points representing land.
		 * @default "#f77114"
		 */
		landPointColor?: ColorRepresentation;
		/**
		 * Whether the globe should auto-rotate.
		 * @default true
		 */
		autoRotate?: boolean;
		/**
		 * Whether to lock the camera's polar angle.
		 * @default true
		 */
		lockedPolarAngle?: boolean;
		/**
		 * Markers to display on the globe.
		 */
		markers?: GlobeMarker[];
		/**
		 * Optional custom tooltip renderer for markers.
		 */
		markerTooltip?: Snippet<[GlobeMarkerTooltipContext]>;
		/**
		 * Coordinates [lat, lon] to focus on.
		 */
		focusOn?: [number, number] | null;
	}

	interface ProjectedMarker {
		marker: GlobeMarker;
		index: number;
		screenX: number;
		screenY: number;
		visibility: number;
	}

	interface UniformUpdaterState {
		radius: number;
		pointCount: number;
		pointSize: number;
		landPointColor: ColorRepresentation;
		fresnelConfig: Required<FresnelConfig>;
		atmosphereConfig: Required<AtmosphereConfig>;
	}

	const PI = Math.PI;
	const DEG2RAD = PI / 180;
	const EPSILON = 1e-6;
	const COBE_GLOBE_RADIUS = 0.8;
	const AUTO_ROTATE_SPEED = (2 * PI) / 30;
	const ROTATE_SENSITIVITY = 0.005;
	const SMOOTHING_STRENGTH = 14;
	const LOCKED_POLAR_ANGLE = 1.5;
	const LOCKED_THETA = Math.asin(Math.cos(LOCKED_POLAR_ANGLE));
	const MIN_THETA = -PI * 0.5 + 0.001;
	const MAX_THETA = PI * 0.5 - 0.001;
	const VISIBILITY_MIN_DOT = 0.24;
	const VISIBILITY_MAX_DOT = 0.48;
	const MAX_SHADER_MARKERS = 128;
	const SHADER_MARKER_SIZE_SCALE = 0.5;
	const MIN_SHADER_MARKER_SIZE = 0.003;
	const MAX_SHADER_MARKER_SIZE = 0.06;

	const defaultFresnelConfig: Required<FresnelConfig> = {
		color: "#17181A",
		rimColor: "#FF6900",
		rimPower: 6,
		rimIntensity: 1.5,
	};

	const defaultAtmosphereConfig: Required<AtmosphereConfig> = {
		color: "#FF6900",
		scale: 1.1,
		power: 12.0,
		coefficient: 0.9,
		intensity: 1.1,
	};

	let {
		radius,
		fresnelConfig = {},
		atmosphereConfig = {},
		pointCount = 15000,
		pointSize = 0.05,
		landPointColor = "#f77114",
		autoRotate = true,
		lockedPolarAngle = true,
		markers = [],
		markerTooltip,
		focusOn = null,
	}: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let projectedMarkers = $state<ProjectedMarker[]>([]);

	const resolvedFresnelConfig = $derived({
		...defaultFresnelConfig,
		...fresnelConfig,
	});
	const resolvedAtmosphereConfig = $derived({
		...defaultAtmosphereConfig,
		...atmosphereConfig,
	});

	let updateUniforms = $state<((state: UniformUpdaterState) => void) | null>(
		null,
	);
	let syncFocusTarget = $state<
		((target: [number, number] | null) => void) | null
	>(null);
	let focusTween: gsap.core.Tween | null = null;

	const clamp = (value: number, min: number, max: number) =>
		Math.min(max, Math.max(min, value));

	const clampTheta = (value: number, lockPolar: boolean) =>
		lockPolar ? LOCKED_THETA : clamp(value, MIN_THETA, MAX_THETA);

	const smoothstep = (value: number, edge0: number, edge1: number) => {
		if (Math.abs(edge1 - edge0) <= EPSILON) {
			return value >= edge1 ? 1 : 0;
		}
		const t = clamp((value - edge0) / (edge1 - edge0), 0, 1);
		return t * t * (3 - 2 * t);
	};

	const toScale = (nextRadius: number) => Math.max(0.001, nextRadius / 2);
	const toPointRadius = (nextPointSize: number) =>
		Math.max(0.001, nextPointSize * 0.16);

	function normalizeAngle(value: number): number {
		const wrapped = (((value + PI) % (2 * PI)) + 2 * PI) % (2 * PI);
		return wrapped - PI;
	}

	function shortestAngleTarget(current: number, next: number): number {
		const delta = normalizeAngle(next - current);
		return current + delta;
	}

	function lonLatToCartesian(lon: number, lat: number, r: number) {
		const lonRad = lon * DEG2RAD;
		const latRad = lat * DEG2RAD;

		const y = r * Math.sin(latRad);
		const rXZ = r * Math.cos(latRad);
		const x = rXZ * Math.sin(lonRad);
		const z = rXZ * Math.cos(lonRad);

		return { x, y, z };
	}

	function cartesianToRotation(x: number, y: number, z: number) {
		const length = Math.hypot(x, y, z);
		if (length <= EPSILON) {
			return { phi: 0, theta: 0 };
		}
		const nx = x / length;
		const ny = y / length;
		const nz = z / length;

		return {
			phi: Math.atan2(-nx, nz),
			theta: Math.asin(clamp(ny, -1, 1)),
		};
	}

	function applyRotation(
		x: number,
		y: number,
		z: number,
		phi: number,
		theta: number,
	) {
		const cx = Math.cos(theta);
		const cy = Math.cos(phi);
		const sx = Math.sin(theta);
		const sy = Math.sin(phi);

		return {
			rx: cy * x + sy * z,
			ry: sy * sx * x + cx * y - cy * sx * z,
			rz: -sy * cx * x + sx * y + cy * cx * z,
		};
	}

	function cubicBezierAt(
		t: number,
		p0: number,
		p1: number,
		p2: number,
		p3: number,
	): number {
		const u = 1 - t;
		return (
			u * u * u * p0 + 3 * u * u * t * p1 + 3 * u * t * t * p2 + t * t * t * p3
		);
	}

	function cubicBezierDerivativeAt(
		t: number,
		p0: number,
		p1: number,
		p2: number,
		p3: number,
	): number {
		const u = 1 - t;
		return (
			3 * u * u * (p1 - p0) + 6 * u * t * (p2 - p1) + 3 * t * t * (p3 - p2)
		);
	}

	function dynamicEase(value: number): number {
		const clamped = clamp(value, 0, 1);
		let t = clamped;
		for (let i = 0; i < 5; i++) {
			const x = cubicBezierAt(t, 0, 0.625, 0, 1);
			const dx = cubicBezierDerivativeAt(t, 0, 0.625, 0, 1);
			if (Math.abs(dx) < 1e-6) break;
			t = clamp(t - (x - clamped) / dx, 0, 1);
		}
		return cubicBezierAt(t, 0, 0.05, 1, 1);
	}

	$effect(() => {
		if (!updateUniforms) return;
		updateUniforms({
			radius,
			pointCount,
			pointSize,
			landPointColor,
			fresnelConfig: resolvedFresnelConfig,
			atmosphereConfig: resolvedAtmosphereConfig,
		});
	});

	$effect(() => {
		if (!syncFocusTarget) return;
		syncFocusTarget(focusOn);
	});

	onMount(() => {
		const targetCanvas = canvas;
		if (!targetCanvas) return;

		const renderer = new Renderer({
			canvas: targetCanvas,
			alpha: true,
			antialias: true,
			dpr: typeof window !== "undefined" ? window.devicePixelRatio : 1,
		});
		const gl = renderer.gl;
		gl.clearColor(0, 0, 0, 0);

		const camera = new Camera(gl);
		camera.position.z = 1;

		const globeScene = new Transform();
		const atmosphereScene = new Transform();
		const geometry = new Triangle(gl);
		const markerData = new Array<number>(MAX_SHADER_MARKERS * 4).fill(0);
		const markerColorData = new Array<number>(MAX_SHADER_MARKERS * 3).fill(0);
		const landTexture = new Texture(gl, {
			image: new Uint8Array([0, 0, 0, 255]),
			width: 1,
			height: 1,
			format: gl.RGBA,
			type: gl.UNSIGNED_BYTE,
			minFilter: gl.NEAREST,
			magFilter: gl.NEAREST,
			generateMipmaps: false,
			wrapS: gl.REPEAT,
			wrapT: gl.REPEAT,
		});

		const uniforms = {
			uResolution: { value: new Vec2(1, 1) },
			uRotation: { value: new Vec2(0, clampTheta(0, lockedPolarAngle)) },
			uScale: { value: toScale(radius) },
			uDots: { value: Math.max(1, Math.floor(pointCount)) },
			uPointRadius: { value: toPointRadius(pointSize) },
			uBaseColor: { value: new Vec3(0, 0, 0) },
			uRimColor: { value: new Vec3(0, 0, 0) },
			uRimPower: { value: resolvedFresnelConfig.rimPower },
			uRimIntensity: { value: resolvedFresnelConfig.rimIntensity },
			uAtmosphereColor: { value: new Vec3(0, 0, 0) },
			uAtmosphereScale: { value: resolvedAtmosphereConfig.scale },
			uAtmospherePower: { value: resolvedAtmosphereConfig.power },
			uAtmosphereCoefficient: { value: resolvedAtmosphereConfig.coefficient },
			uAtmosphereIntensity: { value: resolvedAtmosphereConfig.intensity },
			uLandPointColor: { value: new Vec3(0, 0, 0) },
			uLandTexture: { value: landTexture },
			uMarkerCount: { value: 0 },
			uMarkerData: { value: markerData },
			uMarkerColor: { value: markerColorData },
		};

		const vertexShader = `
			attribute vec2 uv;
			attribute vec2 position;
			varying vec2 vUv;

			void main() {
				vUv = uv;
				gl_Position = vec4(position, 0.0, 1.0);
			}
		`;

		const globeFragmentShader = `
			precision highp float;

			varying vec2 vUv;

			uniform vec2 uResolution;
			uniform vec2 uRotation;
			uniform float uScale;
			uniform float uDots;
			uniform float uPointRadius;
			uniform vec3 uBaseColor;
			uniform vec3 uRimColor;
			uniform float uRimPower;
			uniform float uRimIntensity;
			uniform vec3 uLandPointColor;
			uniform sampler2D uLandTexture;
			uniform float uMarkerCount;
			uniform vec4 uMarkerData[${MAX_SHADER_MARKERS}];
			uniform vec3 uMarkerColor[${MAX_SHADER_MARKERS}];

			const float kPi = 3.141592653589793;
			const float kTau = 6.283185307179586;
			const float kPhi = 1.618033988749895;
			const float kSqrt5 = 2.23606797749979;
			const float kSphereRadius = 0.8;
			const int kMaxMarkers = ${MAX_SHADER_MARKERS};

			float byDots;

			mat3 rotate(float theta, float phi) {
				float cx = cos(theta);
				float cy = cos(phi);
				float sx = sin(theta);
				float sy = sin(phi);
				return mat3(
					cy, sy * sx, -sy * cx,
					0.0, cx, sx,
					sy, cy * -sx, cy * cx
				);
			}

			vec3 nearestFibonacciLattice(vec3 p, out float m) {
				p = p.xzy;

				float k = max(2.0, floor(log2(kSqrt5 * uDots * kPi * (1.0 - p.z * p.z)) * 0.72021));
				vec2 f = floor(pow(kPhi, k) / kSqrt5 * vec2(1.0, kPhi) + 0.5);
				vec2 br1 = fract((f + 1.0) * (kPhi - 1.0)) * kTau - 3.883222;
				vec2 br2 = -2.0 * f;
				vec2 sp = vec2(atan(p.y, p.x), p.z - 1.0);
				vec2 c = floor(vec2(
					br2.y * sp.x - br1.y * (sp.y * uDots + 1.0),
					-br2.x * sp.x + br1.x * (sp.y * uDots + 1.0)
				) / (br1.x * br2.y - br2.x * br1.y));

				float mindist = kPi;
				vec3 minip = vec3(0.0, 0.0, 1.0);

				for (float s = 0.0; s < 4.0; s += 1.0) {
					vec2 o = vec2(mod(s, 2.0), floor(s * 0.5));
					float idx = dot(f, c + o);
					if (idx > uDots) continue;

					float a = idx;
					float b = 0.0;
					if (a >= 16384.0) a -= 16384.0, b += 0.868872;
					if (a >= 8192.0) a -= 8192.0, b += 0.934436;
					if (a >= 4096.0) a -= 4096.0, b += 0.467218;
					if (a >= 2048.0) a -= 2048.0, b += 0.733609;
					if (a >= 1024.0) a -= 1024.0, b += 0.866804;
					if (a >= 512.0) a -= 512.0, b += 0.433402;
					if (a >= 256.0) a -= 256.0, b += 0.216701;
					if (a >= 128.0) a -= 128.0, b += 0.108351;
					if (a >= 64.0) a -= 64.0, b += 0.554175;
					if (a >= 32.0) a -= 32.0, b += 0.777088;
					if (a >= 16.0) a -= 16.0, b += 0.888544;
					if (a >= 8.0) a -= 8.0, b += 0.944272;
					if (a >= 4.0) a -= 4.0, b += 0.472136;
					if (a >= 2.0) a -= 2.0, b += 0.236068;
					if (a >= 1.0) a -= 1.0, b += 0.618034;

					float theta = fract(b) * kTau;
					float cosphi = 1.0 - 2.0 * idx * byDots;
					float sinphi = sqrt(max(0.0, 1.0 - cosphi * cosphi));
					vec3 samplePoint = vec3(cos(theta) * sinphi, sin(theta) * sinphi, cosphi);

					float dist = length(p - samplePoint);
					if (dist < mindist) {
						mindist = dist;
						minip = samplePoint;
					}
				}

				m = mindist;
				return minip.xzy;
			}

			vec2 pointToMaskUV(vec3 p) {
				float lengthP = length(p);
				if (lengthP <= 0.0) {
					return vec2(0.0, 0.0);
				}

				vec3 n = p / lengthP;

				float nx = n.z;
				float ny = n.y;
				float nz = -n.x;

				float gPhi = asin(clamp(ny, -1.0, 1.0));
				float cosPhi = cos(gPhi);

				float gTheta = 0.0;
				if (abs(cosPhi) > 1e-6) {
					float thetaInput = clamp(-nx / cosPhi, -1.0, 1.0);
					gTheta = acos(thetaInput);
					if (nz < 0.0) {
						gTheta = -gTheta;
					}
				}

				return vec2(
					fract((gTheta * 0.5) / kPi),
					fract(gPhi / kPi + 0.5)
				);
			}

			vec3 linearToSrgb(vec3 color) {
				vec3 safe = max(color, vec3(0.0));
				vec3 low = safe * 12.92;
				vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
				vec3 cutoff = step(vec3(0.0031308), safe);
				return mix(low, high, cutoff);
			}

			void main() {
				byDots = 1.0 / max(1.0, uDots);

				vec2 uv = vUv * 2.0 - 1.0;
				uv.x *= uResolution.x / max(1.0, uResolution.y);
				uv /= max(0.0001, uScale);

				float l = dot(uv, uv);
				float globeR2 = kSphereRadius * kSphereRadius;

				vec3 color = vec3(0.0);
				float alpha = 0.0;

				if (l <= globeR2) {
					float dis;
					vec3 p = normalize(vec3(uv, sqrt(max(0.0, globeR2 - l))));
					mat3 rot = rotate(uRotation.y, uRotation.x);
					vec3 globePoint = p * rot;
					vec3 samplePoint = nearestFibonacciLattice(globePoint, dis);
					vec2 mapUv = pointToMaskUV(samplePoint);
					float land = texture2D(uLandTexture, mapUv).r;

					float landDots = step(0.5, land) * smoothstep(uPointRadius, 0.0, dis);

					float dotNV = clamp(p.z / kSphereRadius, 0.0, 1.0);
					float rim = pow(1.0 - dotNV, max(0.0001, uRimPower)) * uRimIntensity;
					// Match the geometric foreshortening from the old instanced point mesh:
					// near the silhouette points should visually fade instead of staying fully crisp.
					float dotFade = smoothstep(0.04, 0.28, dotNV);
					landDots *= dotFade;

					vec3 markerColor = vec3(0.0);
					float markerMask = 0.0;
					float markerWeightSum = 0.0;
					for (int i = 0; i < kMaxMarkers; i++) {
						if (float(i) >= uMarkerCount) {
							break;
						}

						vec4 marker = uMarkerData[i];
						float markerDist = length(globePoint - marker.xyz);
						float markerDot = smoothstep(marker.w, marker.w * 0.62, markerDist);
						markerMask = max(markerMask, markerDot);
						markerWeightSum += markerDot;
						markerColor += uMarkerColor[i] * markerDot;
					}

					if (markerWeightSum > 0.0) {
						markerColor /= markerWeightSum;
					}

					vec3 surface = uBaseColor;
					surface += uRimColor * rim;
					surface += uLandPointColor * (landDots * (1.0 - markerMask));

					// Keep marker color clean and dominant over land dots.
					vec3 boostedMarker = markerColor * (1.0 + 0.25 * markerMask);
					surface = mix(surface, boostedMarker, markerMask);

					color += surface;
					alpha = 1.0;
				}

				gl_FragColor = vec4(linearToSrgb(color), clamp(alpha, 0.0, 1.0));
			}
		`;

		const atmosphereFragmentShader = `
			precision highp float;

			varying vec2 vUv;

			uniform vec2 uResolution;
			uniform float uScale;
			uniform vec3 uAtmosphereColor;
			uniform float uAtmosphereScale;
			uniform float uAtmospherePower;
			uniform float uAtmosphereCoefficient;
			uniform float uAtmosphereIntensity;

			const float kSphereRadius = 0.8;

			vec3 linearToSrgb(vec3 color) {
				vec3 safe = max(color, vec3(0.0));
				vec3 low = safe * 12.92;
				vec3 high = 1.055 * pow(safe, vec3(1.0 / 2.4)) - 0.055;
				vec3 cutoff = step(vec3(0.0031308), safe);
				return mix(low, high, cutoff);
			}

			void main() {
				vec2 uv = vUv * 2.0 - 1.0;
				uv.x *= uResolution.x / max(1.0, uResolution.y);
				uv /= max(0.0001, uScale);

				float globeR = kSphereRadius;
				float atmosphereR = kSphereRadius * max(1.0, uAtmosphereScale);
				float l = dot(uv, uv);
				float radial = sqrt(l);

				if (radial <= globeR) {
					discard;
				}

				float shellWidth = max(1e-5, atmosphereR - globeR);
				float x = (radial - globeR) / shellWidth;
				if (x > 3.0) {
					discard;
				}

				// Smooth outward blur profile (no hard ring cutoff at shell boundary).
				float falloff = exp(-pow(max(0.0, x), 1.2) * max(0.15, uAtmospherePower * 0.09));
				float finalFactor =
					falloff * uAtmosphereIntensity * max(0.0, uAtmosphereCoefficient);

				vec3 finalColor = uAtmosphereColor * finalFactor;
				float alpha = finalFactor;

				gl_FragColor = vec4(linearToSrgb(finalColor), clamp(alpha, 0.0, 1.0));
			}
		`;

		const globeProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: globeFragmentShader,
			uniforms,
			transparent: true,
			depthTest: false,
			depthWrite: false,
		});

		const atmosphereProgram = new Program(gl, {
			vertex: vertexShader,
			fragment: atmosphereFragmentShader,
			uniforms,
			transparent: true,
			depthTest: false,
			depthWrite: false,
		});
		atmosphereProgram.setBlendFunc(gl.SRC_ALPHA, gl.ONE);

		const globeMesh = new Mesh(gl, {
			geometry,
			program: globeProgram,
			frustumCulled: false,
		});
		globeMesh.setParent(globeScene);

		const atmosphereMesh = new Mesh(gl, {
			geometry,
			program: atmosphereProgram,
			frustumCulled: false,
		});
		atmosphereMesh.setParent(atmosphereScene);

		let currentScale = toScale(radius);
		const tempColor = new Vec3();
		const setColor = (
			target: Vec3,
			value: ColorRepresentation,
			fallback: [number, number, number],
		) => {
			const [r, g, b] = toLinearRgb(value, fallback);
			target.set(r, g, b);
		};

		updateUniforms = (state) => {
			currentScale = toScale(state.radius);
			uniforms.uScale.value = currentScale;
			uniforms.uDots.value = Math.max(1, Math.floor(state.pointCount));
			uniforms.uPointRadius.value = toPointRadius(state.pointSize);

			setColor(uniforms.uBaseColor.value, state.fresnelConfig.color, [
				17 / 255,
				17 / 255,
				19 / 255,
			]);
			setColor(uniforms.uRimColor.value, state.fresnelConfig.rimColor, [
				1,
				105 / 255,
				0,
			]);
			uniforms.uRimPower.value = Math.max(0.0001, state.fresnelConfig.rimPower);
			uniforms.uRimIntensity.value = Math.max(
				0,
				state.fresnelConfig.rimIntensity,
			);

			setColor(uniforms.uAtmosphereColor.value, state.atmosphereConfig.color, [
				1,
				105 / 255,
				0,
			]);
			uniforms.uAtmosphereScale.value = Math.max(
				1,
				state.atmosphereConfig.scale,
			);
			uniforms.uAtmospherePower.value = Math.max(
				0.0001,
				state.atmosphereConfig.power,
			);
			uniforms.uAtmosphereCoefficient.value = Math.max(
				0,
				state.atmosphereConfig.coefficient,
			);
			uniforms.uAtmosphereIntensity.value = Math.max(
				0,
				state.atmosphereConfig.intensity,
			);

			setColor(tempColor, state.landPointColor, [
				247 / 255,
				113 / 255,
				20 / 255,
			]);
			uniforms.uLandPointColor.value.set(tempColor.x, tempColor.y, tempColor.z);
		};

		updateUniforms({
			radius,
			pointCount,
			pointSize,
			landPointColor,
			fresnelConfig: resolvedFresnelConfig,
			atmosphereConfig: resolvedAtmosphereConfig,
		});

		let width = 1;
		let height = 1;
		const resize = () => {
			const host = targetCanvas.parentElement ?? targetCanvas;
			const { width: hostWidth, height: hostHeight } =
				host.getBoundingClientRect();
			width = Math.max(1, Math.round(hostWidth));
			height = Math.max(1, Math.round(hostHeight));
			renderer.setSize(width, height);
			uniforms.uResolution.value.set(gl.canvas.width, gl.canvas.height);
		};

		resize();
		const observer = new ResizeObserver(resize);
		observer.observe(targetCanvas);
		if (targetCanvas.parentElement)
			observer.observe(targetCanvas.parentElement);

		const startTheta = clampTheta(0, lockedPolarAngle);
		let phi = 0;
		let theta = startTheta;
		let targetPhi = phi;
		let targetTheta = startTheta;

		const syncMarkers = (
			currentPhi: number,
			currentTheta: number,
			currentScaleValue: number,
		) => {
			const markerRadius = COBE_GLOBE_RADIUS;
			const aspect = width / Math.max(1, height);
			const markerCount = Math.min(markers.length, MAX_SHADER_MARKERS);
			markerData.fill(0);
			markerColorData.fill(0);
			uniforms.uMarkerCount.value = markerCount;

			const nextMarkers: ProjectedMarker[] = [];
			for (let index = 0; index < markers.length; index++) {
				const marker = markers[index];
				const pos = lonLatToCartesian(
					marker.location[1],
					marker.location[0],
					markerRadius,
				);
				const rotated = applyRotation(
					pos.x,
					pos.y,
					pos.z,
					currentPhi,
					currentTheta,
				);

				const ndcX = (rotated.rx / aspect) * currentScaleValue;
				const ndcY = -rotated.ry * currentScaleValue;
				const screenX = (ndcX + 1) * 0.5;
				const screenY = (ndcY + 1) * 0.5;

				const frontDot = rotated.rz / markerRadius;
				const rawVisibility = smoothstep(
					frontDot,
					VISIBILITY_MIN_DOT,
					VISIBILITY_MAX_DOT,
				);
				const visibility = dynamicEase(rawVisibility);

				nextMarkers.push({
					marker,
					index,
					screenX,
					screenY,
					visibility,
				});

				if (index >= markerCount) continue;

				const unitPos = lonLatToCartesian(
					marker.location[1],
					marker.location[0],
					1,
				);
				const markerDataOffset = index * 4;
				markerData[markerDataOffset] = unitPos.x;
				markerData[markerDataOffset + 1] = unitPos.y;
				markerData[markerDataOffset + 2] = unitPos.z;
				markerData[markerDataOffset + 3] = clamp(
					(marker.size ?? 0.05) * SHADER_MARKER_SIZE_SCALE,
					MIN_SHADER_MARKER_SIZE,
					MAX_SHADER_MARKER_SIZE,
				);

				const [r, g, b] = toLinearRgb(marker.color ?? "#ffffff", [1, 1, 1]);
				const markerColorOffset = index * 3;
				markerColorData[markerColorOffset] = r;
				markerColorData[markerColorOffset + 1] = g;
				markerColorData[markerColorOffset + 2] = b;
			}

			projectedMarkers = nextMarkers;
		};

		syncFocusTarget = (target) => {
			focusTween?.kill();
			focusTween = null;

			if (!target) return;

			const [lat, lon] = target;
			const nextDirection = lonLatToCartesian(lon, lat, 1);
			const targetRotation = cartesianToRotation(
				nextDirection.x,
				nextDirection.y,
				nextDirection.z,
			);

			const desiredTheta = clampTheta(targetRotation.theta, lockedPolarAngle);
			const desiredPhi = shortestAngleTarget(targetPhi, targetRotation.phi);

			const tweenState = { phi: targetPhi, theta: targetTheta };
			focusTween = gsap.to(tweenState, {
				phi: desiredPhi,
				theta: desiredTheta,
				duration: 1.5,
				ease: "power2.inOut",
				onUpdate: () => {
					targetPhi = tweenState.phi;
					targetTheta = clampTheta(tweenState.theta, lockedPolarAngle);
				},
				overwrite: true,
			});
		};

		syncFocusTarget(focusOn);

		let dragging = false;
		let activePointerId = -1;
		let lastPointerX = 0;
		let lastPointerY = 0;

		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0) return;
			dragging = true;
			activePointerId = event.pointerId;
			lastPointerX = event.clientX;
			lastPointerY = event.clientY;
			targetCanvas.setPointerCapture(event.pointerId);
			focusTween?.kill();
			focusTween = null;
		};

		const onPointerMove = (event: PointerEvent) => {
			if (!dragging || event.pointerId !== activePointerId) return;
			const dx = event.clientX - lastPointerX;
			const dy = event.clientY - lastPointerY;
			lastPointerX = event.clientX;
			lastPointerY = event.clientY;

			targetPhi += dx * ROTATE_SENSITIVITY;
			targetTheta = clampTheta(
				targetTheta + dy * ROTATE_SENSITIVITY,
				lockedPolarAngle,
			);
		};

		const stopDragging = (event: PointerEvent) => {
			if (event.pointerId !== activePointerId) return;
			dragging = false;
			activePointerId = -1;
		};

		targetCanvas.addEventListener("pointerdown", onPointerDown);
		targetCanvas.addEventListener("pointermove", onPointerMove);
		targetCanvas.addEventListener("pointerup", stopDragging);
		targetCanvas.addEventListener("pointercancel", stopDragging);
		targetCanvas.addEventListener("lostpointercapture", stopDragging);

		let disposed = false;
		const image = new Image();
		image.onload = () => {
			if (disposed) return;
			landTexture.image = image;
			landTexture.generateMipmaps = true;
			landTexture.minFilter = gl.NEAREST_MIPMAP_NEAREST;
			landTexture.magFilter = gl.NEAREST;
			landTexture.needsUpdate = true;
		};
		image.onerror = (error) => {
			console.warn("GlobeScene: failed to load land mask texture", error);
		};
		image.src = landTextureUrl;

		let raf = 0;
		let previous = 0;
		const tick = (now: number) => {
			const delta = previous ? (now - previous) / 1000 : 0;
			previous = now;

			if (autoRotate) {
				targetPhi -= AUTO_ROTATE_SPEED * delta;
			}
			targetTheta = clampTheta(targetTheta, lockedPolarAngle);

			const easing = 1 - Math.exp(-delta * SMOOTHING_STRENGTH);
			phi += (targetPhi - phi) * easing;
			theta += (targetTheta - theta) * easing;

			uniforms.uRotation.value.set(phi, theta);

			syncMarkers(phi, theta, currentScale);
			renderer.render({ scene: globeScene, camera, clear: true });
			renderer.render({ scene: atmosphereScene, camera, clear: false });
			raf = window.requestAnimationFrame(tick);
		};

		raf = window.requestAnimationFrame(tick);

		return () => {
			disposed = true;
			focusTween?.kill();
			focusTween = null;
			window.cancelAnimationFrame(raf);
			observer.disconnect();
			targetCanvas.removeEventListener("pointerdown", onPointerDown);
			targetCanvas.removeEventListener("pointermove", onPointerMove);
			targetCanvas.removeEventListener("pointerup", stopDragging);
			targetCanvas.removeEventListener("pointercancel", stopDragging);
			targetCanvas.removeEventListener("lostpointercapture", stopDragging);

			globeMesh.setParent(null);
			atmosphereMesh.setParent(null);
			geometry.remove();
			globeProgram.remove();
			atmosphereProgram.remove();
		};
	});
</script>

<canvas
	bind:this={canvas}
	class="absolute inset-0 block h-full w-full"
	style="width:100%;height:100%;touch-action:none;"
	aria-hidden="true"
></canvas>

<div class="pointer-events-none absolute inset-0 overflow-hidden">
	{#each projectedMarkers as projected, i (projected.marker.label || i)}
		<GlobeMarkerItem
			marker={projected.marker}
			index={projected.index}
			screenX={projected.screenX}
			screenY={projected.screenY}
			visibility={projected.visibility}
			tooltip={markerTooltip}
		/>
	{/each}
</div>
