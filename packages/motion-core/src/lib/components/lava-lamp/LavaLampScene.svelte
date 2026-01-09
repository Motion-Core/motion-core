<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import { Vector4, ShaderMaterial, Color } from "three";

	interface Props {
		/**
		 * Base color of the lava blobs.
		 * @default "#18181b"
		 */
		color?: string;
		/**
		 * Color of the fresnel effect.
		 * @default "#ff6900"
		 */
		fresnelColor?: string;
		/**
		 * Speed of the lava animation.
		 * @default 1.0
		 */
		speed?: number;
		/**
		 * Fresnel power for the edge lighting effect.
		 * @default 3.0
		 */
		fresnelPower?: number;
		/**
		 * Base radius of the blobs.
		 * @default 1
		 */
		radius?: number;
		/**
		 * Smoothness of the blob blending (metaball effect).
		 * @default 0.1
		 */
		smoothness?: number;
	}

	let {
		color = "#18181b",
		fresnelColor = "#ff6900",
		speed = 1.0,
		fresnelPower = 3.0,
		radius = 1,
		smoothness = 0.1,
	}: Props = $props();

	const { size } = useThrelte();

	let time = 0;
	let material = $state<ShaderMaterial>();

	const resolutionUniform = new Vector4();
	const colorUniform = new Color();
	const fresnelColorUniform = new Color();

	const updateResolution = () => {
		const width = $size.width;
		const height = $size.height;
		const imageAspect = 1;
		let a1, a2;

		if (height / width > imageAspect) {
			a1 = (width / height) * imageAspect;
			a2 = 1;
		} else {
			a1 = 1;
			a2 = height / width / imageAspect;
		}

		resolutionUniform.set(width, height, a1, a2);
	};

	$effect(() => {
		updateResolution();
	});

	$effect(() => {
		colorUniform.set(color);
		fresnelColorUniform.set(fresnelColor);
		if (material) {
			material.uniforms.uColor.value.copy(colorUniform);
			material.uniforms.uFresnelColor.value.copy(fresnelColorUniform);
			material.uniforms.uFresnelPower.value = fresnelPower;
			material.uniforms.uRadius.value = radius;
			material.uniforms.uSmoothness.value = smoothness;
		}
	});

	const vertexShader = `
    varying vec2 vUv;
    uniform float uTime;
    uniform vec4 uResolution;

    void main() {
        vUv = uv;
        gl_Position = vec4(position, 1.0);
    }
  `;

	const fragmentShader = `
    precision highp float;
    varying vec2 vUv;
    uniform float uTime;
    uniform vec4 uResolution;
    uniform vec3 uColor;
    uniform vec3 uFresnelColor;
    uniform float uFresnelPower;
    uniform float uRadius;
    uniform float uSmoothness;

    float PI = 3.141592653589793238;

    mat4 rotationMatrix(vec3 axis, float angle) {
        axis = normalize(axis);
        float s = sin(angle);
        float c = cos(angle);
        float oc = 1.0 - c;
        return mat4(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
                    oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
                    oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
                    0.0,                                0.0,                                0.0,                                1.0);
    }

    vec3 rotate(vec3 v, vec3 axis, float angle) {
        mat4 m = rotationMatrix(axis, angle);
        return (m * vec4(v, 1.0)).xyz;
    }

    float smin( float a, float b, float k ) {
        k *= 6.0;
        float h = max( k-abs(a-b), 0.0 )/k;
        return min(a,b) - h*h*h*k*(1.0/6.0);
    }

    float sphereSDF(vec3 p, float r) {
        return length(p) - r;
    }

    float sdf(vec3 p) {
        vec3 p1 = rotate(p, vec3(0.0, 0.0, 1.0), uTime/5.0);
        vec3 p2 = rotate(p, vec3(1.), -uTime/5.0);
        vec3 p3 = rotate(p, vec3(1., 1., 0.), -uTime/4.5);
        vec3 p4 = rotate(p, vec3(0., 1., 0.), -uTime/4.0);

        float r = uRadius;

        float final = sphereSDF(p1 - vec3(-0.5, 0.0, 0.0), 0.35 * r);
        float nextSphere = sphereSDF(p2 - vec3(0.55, 0.0, 0.0), 0.3 * r);
        final = smin(final, nextSphere, uSmoothness);
        nextSphere = sphereSDF(p2 - vec3(-0.8, 0.0, 0.0), 0.2 * r);
        final = smin(final, nextSphere, uSmoothness);
        nextSphere = sphereSDF(p3 - vec3(1.0, 0.0, 0.0), 0.15 * r);
        final = smin(final, nextSphere, uSmoothness);
        nextSphere = sphereSDF(p4 - vec3(0.45, -0.45, 0.0), 0.15 * r);
        final = smin(final, nextSphere, uSmoothness);

        return final;
    }

    vec3 getNormal(vec3 p) {
        float d = 0.001;
        return normalize(vec3(
            sdf(p + vec3(d, 0.0, 0.0)) - sdf(p - vec3(d, 0.0, 0.0)),
            sdf(p + vec3(0.0, d, 0.0)) - sdf(p - vec3(0.0, d, 0.0)),
            sdf(p + vec3(0.0, 0.0, d)) - sdf(p - vec3(0.0, 0.0, d))
        ));
    }

    float rayMarch(vec3 rayOrigin, vec3 ray) {
        float t = 0.0;
        for (int i = 0; i < 100; i++) {
            vec3 p = rayOrigin + ray * t;
            float d = sdf(p);
            if (d < 0.001) return t;
            t += d;
            if (t > 100.0) break;
        }
        return -1.0;
    }

    void main() {
        vec2 newUV = (vUv - vec2(0.5)) * uResolution.zw + vec2(0.5);
        vec3 cameraPos = vec3(0.0, 0.0, 5.0);
        vec3 ray = normalize(vec3((vUv - vec2(0.5)) * uResolution.zw, -1));

        float t = rayMarch(cameraPos, ray);
        if (t > 0.0) {
            vec3 p = cameraPos + ray * t;
            vec3 normal = getNormal(p);
            float fresnel = pow(1.0 + dot(ray, normal), uFresnelPower);


            vec3 color = mix(uColor, uFresnelColor, fresnel);

            gl_FragColor = vec4(color, 1.0);
        } else {
             gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        }
        #include <colorspace_fragment>
    }
  `;

	useTask((delta) => {
		time += delta * speed;
		if (material) {
			material.uniforms.uTime.value = time;
			material.uniforms.uResolution.value.copy(resolutionUniform);
		}
	});
</script>

<T.OrthographicCamera
	makeDefault
	position={[0, 0, 2]}
	left={-0.5}
	right={0.5}
	top={0.5}
	bottom={-0.5}
	near={-1000}
	far={1000}
/>

<T.Mesh>
	<T.PlaneGeometry args={[2, 2]} />
	<T.ShaderMaterial
		bind:ref={material}
		{vertexShader}
		{fragmentShader}
		uniforms={{
			uTime: { value: 0 },
			uResolution: { value: resolutionUniform },
			uColor: { value: colorUniform },
			uFresnelColor: { value: fresnelColorUniform },
			uFresnelPower: { value: fresnelPower },
			uRadius: { value: radius },
			uSmoothness: { value: smoothness },
		}}
		transparent
	/>
</T.Mesh>
