<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import {
		Vector2,
		ShaderMaterial,
		MirroredRepeatWrapping,
		LinearFilter,
		Color,
	} from "three";
	import { useTexture } from "@threlte/extras";

	interface Props {
		/**
		 * The image source URL.
		 */
		image: string;
		/**
		 * Grid density for the ASCII effect.
		 * @default 25.0
		 */
		density?: number;
		/**
		 * Intensity of the ASCII character generation threshold.
		 * @default 25.0
		 */
		strength?: number;
		/**
		 * Foreground color of the ASCII characters.
		 * @default "#00ff00"
		 */
		color?: string;
		/**
		 * Background color.
		 * @default "#000000"
		 */
		backgroundColor?: string;
	}

	let {
		image,
		density = 25.0,
		strength = 25.0,
		color = "#00ff00",
		backgroundColor = "#000000",
	}: Props = $props();

	let time = 0;
	const { size, renderer } = useThrelte();

	let canvasWidth = $state(1);
	let canvasHeight = $state(1);
	let imageWidth = $state(1);
	let imageHeight = $state(1);

	const coverScaleUniform = new Vector2(1, 1);
	const coverOffsetUniform = new Vector2(0, 0);
	const colorUniform = new Color();
	const backgroundColorUniform = new Color();

	const updateCoverUniforms = () => {
		if (
			canvasWidth <= 0 ||
			canvasHeight <= 0 ||
			imageWidth <= 0 ||
			imageHeight <= 0
		) {
			return;
		}

		const screenAspect = canvasWidth / canvasHeight;
		const imageAspect = imageWidth / imageHeight;

		let scaleX = 1;
		let scaleY = 1;
		let offsetX = 0;
		let offsetY = 0;

		if (screenAspect > imageAspect) {
			scaleY = imageAspect / screenAspect;
			offsetY = (1 - scaleY) * 0.5;
		} else {
			scaleX = screenAspect / imageAspect;
			offsetX = (1 - scaleX) * 0.5;
		}

		coverScaleUniform.set(scaleX, scaleY);
		coverOffsetUniform.set(offsetX, offsetY);
	};

	$effect(() => {
		const nextWidth = $size.width;
		const nextHeight = $size.height;
		canvasWidth = nextWidth;
		canvasHeight = nextHeight;
		updateCoverUniforms();
	});

	$effect(() => {
		colorUniform.set(color);
		backgroundColorUniform.set(backgroundColor);
		if (material) {
			material.uniforms.uColor.value.copy(colorUniform);
			material.uniforms.uBackgroundColor.value.copy(backgroundColorUniform);
		}
	});

	const vertexShader = `
    varying vec2 vUv;
    void main() {
      vUv = uv;
      gl_Position = vec4(position, 1.0);
    }
  `;

	const fragmentShader = `
    uniform float uTime;
    uniform vec2 uResolution;
    uniform sampler2D uTexture;
    uniform vec2 uCoverScale;
    uniform vec2 uCoverOffset;
    uniform float uDensity;
    uniform float uStrength;
    uniform vec3 uColor;
    uniform vec3 uBackgroundColor;

    varying vec2 vUv;

    float digit(vec2 p, float intensity){
        p = (fract(p) - 0.5) * 1.2 + 0.5;

        if (p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) return 0.0;

        float x = fract(p.x * 5.);
        float y = fract((1. - p.y) * 5.);
        int i = int(floor((1. - p.y) * 5.));
        int j = int(floor(p.x * 5.));
        int n = (i-2)*(i-2)+(j-2)*(j-2);
        float f = float(n)/16.;
        float isOn = smoothstep(0.1, 0.2, intensity - f);
        return isOn * (0.2 + y*4./5.) * (0.75 + x/4.);
    }

    float onOff(float a, float b, float c)
    {
        return step(c, sin(uTime + a*cos(uTime*b)));
    }

    float displace(vec2 look)
    {
        float y = (look.y-mod(uTime/4.,1.));
        float window = 1./(1.+50.*y*y);
        return sin(look.y*20. + uTime)/80.*onOff(4.,2.,.8)*(1.+cos(uTime*60.))*window;
    }

    void main() {
        vec2 p = vUv;
        float aspect = uResolution.x / uResolution.y;
        p.x *= aspect;

        vec2 pDisplaced = p;
        pDisplaced.x += displace(p) * 0.5;

        vec2 grid = vec2(3., 1.) * uDensity;

        vec2 cellIndex = floor(pDisplaced * grid);
        vec2 cellCenterP = (cellIndex + 0.5) / grid;

        vec2 cellCenterUV = cellCenterP;
        cellCenterUV.x /= aspect;

        vec2 cellCenterUVCover = (cellCenterUV * uCoverScale) + uCoverOffset;

        vec3 texColor = texture2D(uTexture, cellCenterUVCover).rgb;

        float intensity = dot(texColor, vec3(0.299, 0.587, 0.114));
        intensity = pow(intensity, 2.8);

        float bar = mod(p.y + uTime*20., 1.) < 0.2 ?  1.4  : 1.;

        vec2 gridP = pDisplaced * grid;
        float middle = digit(gridP, intensity * 1.3 * uStrength);

        float off = 0.002;
        float sum = 0.;
        for (float i = -1.; i < 2.; i+=1.){
            for (float j = -1.; j < 2.; j+=1.){
                vec2 offsetGridP = gridP + vec2(off*i*grid.x, off*j*grid.y);
                sum += digit(offsetGridP, intensity * 1.3 * uStrength);
            }
        }

        vec3 emission = vec3(0.6)*middle + sum/15.*uColor * bar;
        vec3 final = uBackgroundColor + emission;

        gl_FragColor = vec4(final, 1.0);
        #include <colorspace_fragment>
    }
  `;

	const texture = $derived(
		useTexture(image, {
			transform: (tex) => {
				tex.wrapS = MirroredRepeatWrapping;
				tex.wrapT = MirroredRepeatWrapping;
				tex.minFilter = LinearFilter;
				tex.magFilter = LinearFilter;
				tex.anisotropy = renderer
					? renderer.capabilities.getMaxAnisotropy()
					: 1;
				return tex;
			},
		}),
	);

	$effect(() => {
		const tex = $texture;
		if (tex && tex.image) {
			imageWidth = tex.image.width;
			imageHeight = tex.image.height;
			updateCoverUniforms();
		}
	});

	let material = $state<ShaderMaterial>();

	useTask((delta) => {
		time += delta;
		if (material) {
			material.uniforms.uTime.value = time;
			material.uniforms.uResolution.value.set($size.width, $size.height);
			material.uniforms.uCoverScale.value = coverScaleUniform;
			material.uniforms.uCoverOffset.value = coverOffsetUniform;
			material.uniforms.uDensity.value = density;
			material.uniforms.uStrength.value = strength;
		}
	});
</script>

{#if $texture}
	<T.Mesh>
		<T.PlaneGeometry args={[2, 2]} />
		<T.ShaderMaterial
			bind:ref={material}
			{vertexShader}
			{fragmentShader}
			uniforms={{
				uTime: { value: 0 },
				uResolution: { value: new Vector2(1, 1) },
				uTexture: { value: $texture },
				uCoverScale: { value: coverScaleUniform },
				uCoverOffset: { value: coverOffsetUniform },
				uDensity: { value: density },
				uStrength: { value: strength },
				uColor: { value: colorUniform },
				uBackgroundColor: { value: backgroundColorUniform },
			}}
		/>
	</T.Mesh>
{/if}
