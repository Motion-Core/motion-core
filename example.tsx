import React, { useRef, useEffect, useState } from "react";
import * as THREE from "three";

// Shader sources
const fragmentShader = `
precision mediump float;
uniform float time;
uniform sampler2D uDataTexture;
uniform sampler2D uTexture;
uniform vec4 resolution;
varying vec2 vUv;
varying vec3 vPosition;
void main() {
	vec2 newUV = (vUv - vec2(0.5)) * resolution.zw + vec2(0.5);
	vec4 offset = texture2D(uDataTexture, vUv);

	// Apply displacement based on offset
	vec2 displacedUV = newUV - 0.02 * offset.rg;
	vec4 color = texture2D(uTexture, displacedUV);

	gl_FragColor = color;
}
`;

const vertexShader = `
precision mediump float;
uniform float time;
varying vec2 vUv;
varying vec3 vPosition;
void main() {
  vUv = uv;
  vPosition = position;
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;

interface PixelatedImageProps {
	src: string;
	alt?: string;
	className?: string;
	grid?: number;
	mouse?: number;
	strength?: number;
	relaxation?: number;
	style?: React.CSSProperties;
}

interface MouseState {
	x: number;
	y: number;
	prevX: number;
	prevY: number;
	vX: number;
	vY: number;
}

function clamp(number: number, min: number, max: number): number {
	return Math.max(min, Math.min(number, max));
}

class PixelationEffect {
	private scene: THREE.Scene;
	private camera!: THREE.OrthographicCamera;
	private renderer!: THREE.WebGLRenderer;
	private material: THREE.ShaderMaterial | null = null;
	private texture: THREE.DataTexture | null = null;
	private geometry!: THREE.PlaneGeometry;
	private plane!: THREE.Mesh;
	private container: HTMLElement;
	private img: HTMLImageElement;
	private width: number;
	private height: number;
	private time: number = 0;
	private mouse: MouseState;
	private size: number;
	private isPlaying: boolean = true;
	private settings: {
		grid: number;
		mouse: number;
		strength: number;
		relaxation: number;
	};
	private imageAspect: number = 1 / 1.5;
	private animationId: number | null = null;

	constructor(
		container: HTMLElement,
		img: HTMLImageElement,
		options: {
			grid?: number;
			mouse?: number;
			strength?: number;
			relaxation?: number;
		} = {},
	) {
		this.container = container;
		this.img = img;
		this.width = container.offsetWidth;
		this.height = container.offsetHeight;

		this.settings = {
			grid: options.grid || 34,
			mouse: options.mouse || 0.25,
			strength: options.strength || 1,
			relaxation: options.relaxation || 0.9,
		};

		this.size = this.settings.grid;

		this.mouse = {
			x: 0,
			y: 0,
			prevX: 0,
			prevY: 0,
			vX: 0,
			vY: 0,
		};

		this.scene = new THREE.Scene();

		this.renderer = new THREE.WebGLRenderer({
			alpha: true,
			antialias: true,
			preserveDrawingBuffer: false,
			powerPreference: "high-performance",
		});
		this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		this.renderer.setSize(this.width, this.height);
		this.renderer.setClearColor(0x000000, 0);

		const gl = this.renderer.getContext();
		if (!gl) {
			console.error("WebGL context not available");
			return;
		}

		container.appendChild(this.renderer.domElement);

		const frustumSize = 1;
		this.camera = new THREE.OrthographicCamera(
			frustumSize / -2,
			frustumSize / 2,
			frustumSize / 2,
			frustumSize / -2,
			-1000,
			1000,
		);
		this.camera.position.set(0, 0, 2);

		this.geometry = new THREE.PlaneGeometry(1, 1, 1, 1);

		this.regenerateGrid();
		this.addObjects();
		this.resize();
		this.setupEventListeners();
		this.render();
	}

	private setupEventListeners(): void {
		const handleMouseMove = (e: MouseEvent) => {
			const rect = this.container.getBoundingClientRect();
			this.mouse.x = (e.clientX - rect.left) / this.width;
			this.mouse.y = (e.clientY - rect.top) / this.height;

			this.mouse.vX = this.mouse.x - this.mouse.prevX;
			this.mouse.vY = this.mouse.y - this.mouse.prevY;

			this.mouse.prevX = this.mouse.x;
			this.mouse.prevY = this.mouse.y;
		};

		const handleResize = () => {
			this.resize();
		};

		this.container.addEventListener("mousemove", handleMouseMove);
		window.addEventListener("resize", handleResize);

		this.cleanupListeners = () => {
			this.container.removeEventListener("mousemove", handleMouseMove);
			window.removeEventListener("resize", handleResize);
		};
	}

	private cleanupListeners: (() => void) | null = null;

	private resize(): void {
		this.width = this.container.offsetWidth;
		this.height = this.container.offsetHeight;
		this.renderer.setSize(this.width, this.height);

		let a1, a2;
		if (this.height / this.width > this.imageAspect) {
			a1 = (this.width / this.height) * this.imageAspect;
			a2 = 1;
		} else {
			a1 = 1;
			a2 = this.height / this.width / this.imageAspect;
		}

		if (this.material) {
			this.material.uniforms.resolution.value.x = this.width;
			this.material.uniforms.resolution.value.y = this.height;
			this.material.uniforms.resolution.value.z = a1;
			this.material.uniforms.resolution.value.w = a2;
		}

		this.camera.updateProjectionMatrix();
		this.regenerateGrid();
	}

	private regenerateGrid(): void {
		this.size = this.settings.grid;
		const width = this.size;
		const height = this.size;
		const size = width * height;
		const data = new Float32Array(4 * size);

		for (let i = 0; i < size; i++) {
			const r = Math.random() * 255 - 125;
			const r1 = Math.random() * 255 - 125;
			const stride = i * 4;

			data[stride] = r;
			data[stride + 1] = r1;
			data[stride + 2] = r;
			data[stride + 3] = 255;
		}

		this.texture = new THREE.DataTexture(
			data,
			width,
			height,
			THREE.RGBAFormat,
			THREE.FloatType,
		);
		this.texture.magFilter = this.texture.minFilter = THREE.NearestFilter;

		if (this.material) {
			this.material.uniforms.uDataTexture.value = this.texture;
			this.material.uniforms.uDataTexture.value.needsUpdate = true;
		}
	}

	private addObjects(): void {
		this.regenerateGrid();
		const texture = new THREE.Texture(this.img);
		texture.needsUpdate = true;
		texture.wrapS = texture.wrapT = THREE.ClampToEdgeWrapping;
		texture.minFilter = texture.magFilter = THREE.LinearFilter;

		this.material = new THREE.ShaderMaterial({
			side: THREE.DoubleSide,
			uniforms: {
				time: { value: 0 },
				resolution: { value: new THREE.Vector4() },
				uTexture: { value: texture },
				uDataTexture: { value: this.texture },
			},
			vertexShader: vertexShader,
			fragmentShader: fragmentShader,
			transparent: true,
		});

		this.plane = new THREE.Mesh(this.geometry, this.material);
		this.scene.add(this.plane);
	}

	private updateDataTexture(): void {
		if (!this.texture) return;

		const data = this.texture.image.data;

		for (let i = 0; i < data.length; i += 4) {
			data[i] *= this.settings.relaxation;
			data[i + 1] *= this.settings.relaxation;
		}

		const gridMouseX = this.size * this.mouse.x;
		const gridMouseY = this.size * (1 - this.mouse.y);
		const maxDist = this.size * this.settings.mouse;
		const aspect = this.height / this.width;

		for (let i = 0; i < this.size; i++) {
			for (let j = 0; j < this.size; j++) {
				const distance = (gridMouseX - i) ** 2 / aspect + (gridMouseY - j) ** 2;
				const maxDistSq = maxDist ** 2;

				if (distance < maxDistSq) {
					const index = 4 * (i + this.size * j);
					let power = maxDist / Math.sqrt(distance);
					power = clamp(power, 0, 10);

					data[index] += this.settings.strength * 100 * this.mouse.vX * power;
					data[index + 1] -=
						this.settings.strength * 100 * this.mouse.vY * power;
				}
			}
		}

		this.mouse.vX *= 0.9;
		this.mouse.vY *= 0.9;
		this.texture.needsUpdate = true;
	}

	private render = (): void => {
		if (!this.isPlaying) return;

		this.time += 0.05;
		this.updateDataTexture();

		if (this.material) {
			this.material.uniforms.time.value = this.time;
		}

		this.animationId = requestAnimationFrame(this.render);
		this.renderer.render(this.scene, this.camera);
	};

	public destroy(): void {
		this.isPlaying = false;

		if (this.animationId) {
			cancelAnimationFrame(this.animationId);
		}

		if (this.cleanupListeners) {
			this.cleanupListeners();
		}

		if (this.renderer.domElement.parentNode) {
			this.renderer.domElement.parentNode.removeChild(this.renderer.domElement);
		}

		this.renderer.dispose();
		this.geometry.dispose();

		if (this.material) {
			this.material.dispose();
		}

		if (this.texture) {
			this.texture.dispose();
		}
	}
}

const PixelatedImage: React.FC<PixelatedImageProps> = ({
	src,
	alt = "",
	className = "",
	grid = 34,
	mouse = 0.25,
	strength = 1,
	relaxation = 0.9,
	style = {},
}) => {
	const containerRef = useRef<HTMLDivElement>(null);
	const imgRef = useRef<HTMLImageElement>(null);
	const effectRef = useRef<PixelationEffect | null>(null);
	const [isLoaded, setIsLoaded] = useState(false);

	useEffect(() => {
		if (!isLoaded || !containerRef.current || !imgRef.current) return;

		effectRef.current = new PixelationEffect(
			containerRef.current,
			imgRef.current,
			{ grid, mouse, strength, relaxation },
		);

		return () => {
			if (effectRef.current) {
				effectRef.current.destroy();
				effectRef.current = null;
			}
		};
	}, [isLoaded, grid, mouse, strength, relaxation]);

	const handleImageLoad = () => {
		setIsLoaded(true);
	};

	const handleImageError = () => {
		console.warn("Failed to load image, CORS issue or invalid URL");
	};

	return (
		<div
			className={`pixelated-image-container ${className}`}
			style={{ position: "relative", ...style }}
		>
			<img
				ref={imgRef}
				src={src}
				alt={alt}
				crossOrigin="anonymous"
				onLoad={handleImageLoad}
				onError={handleImageError}
				style={{
					width: "100%",
					height: "100%",
					objectFit: "cover",
					visibility: isLoaded ? "hidden" : "visible",
				}}
			/>
			<div
				ref={containerRef}
				style={{
					position: "absolute",
					top: 0,
					left: 0,
					width: "100%",
					height: "100%",
					pointerEvents: "auto",
				}}
			/>
		</div>
	);
};

export default PixelatedImage;
