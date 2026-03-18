<script lang="ts">
	import { T, useTask, useThrelte } from "@threlte/core";
	import * as THREE from "three";

	interface Props {
		/**
		 * Camera rotation speed multiplier.
		 * @default 0.5
		 */
		rotationSpeed?: number;
		/**
		 * Color of the background.
		 * @default "#000000"
		 */
		backgroundColor?: THREE.ColorRepresentation;
		/**
		 * Distance of the camera from the center.
		 * @default 3.0
		 */
		cameraDistance?: number;
		/**
		 * Field of View (FOV) of the camera in degrees.
		 * @default 55.0
		 */
		fov?: number;
		/**
		 * Sun light direction vector (X).
		 * @default 0.0
		 */
		sunX?: number;
		/**
		 * Sun light direction vector (Y).
		 * @default 0.0
		 */
		sunY?: number;
		/**
		 * Sun light direction vector (Z).
		 * @default 1.0
		 */
		sunZ?: number;
		/**
		 * Overall intensity/brightness of the scattering effect.
		 * @default 1.0
		 */
		intensity?: number;
	}

	let {
		rotationSpeed = 0.5,
		backgroundColor = "#000000",
		cameraDistance = 3.0,
		fov = 55.0,
		sunX = 0.0,
		sunY = 0.0,
		sunZ = 1.0,
		intensity = 1.0,
	}: Props = $props();

	let material = $state<THREE.ShaderMaterial>();
	const { size } = useThrelte();
	const resolutionUniform = new THREE.Vector2(1, 1);
	const sunDirUniform = new THREE.Vector3(0, 0, 1);
	const backgroundColorUniform = new THREE.Color();

	const vertexShader = `
		varying vec2 vUv;
		void main() {
			vUv = uv;
			gl_Position = vec4(position, 1.0);
		}
	`;

	const fragmentShader = `
		precision highp float;
		varying vec2 vUv;

		uniform float uTime;
		uniform vec2 uResolution;
		uniform vec3 uBackgroundColor;
		uniform float uRotationSpeed;
		uniform float uCameraDistance;
		uniform float uFov;
		uniform vec3 uSunDir;
		uniform float uIntensity;

		const float PI = 3.14159265359;
		const float MAX = 10000.0;
		const float R_INNER = 1.0;
		const float R = 1.5;
		const int NUM_OUT_SCATTER = 8;
		const int NUM_IN_SCATTER = 40; // Adjusted for performance but kept high for quality

		// ray intersects sphere
		vec2 ray_vs_sphere( vec3 p, vec3 dir, float r ) {
			float b = dot( p, dir );
			float c = dot( p, p ) - r * r;
			float d = b * b - c;
			if ( d < 0.0 ) {
				return vec2( MAX, -MAX );
			}
			d = sqrt( d );
			return vec2( -b - d, -b + d );
		}

		// mie
		float phase_mie( float g, float c, float cc ) {
			float gg = g * g;
			float a = ( 1.0 - gg ) * ( 1.0 + cc );
			float b = 1.0 + gg - 2.0 * g * c;
			b *= sqrt( b );
			b *= 2.0 + gg;
			return ( 3.0 / 8.0 / PI ) * a / b;
		}

		// Rayleigh
		float phase_ray( float cc ) {
			return ( 3.0 / 16.0 / PI ) * ( 1.0 + cc );
		}

		float density( vec3 p, float ph ) {
			return exp( -max( length( p ) - R_INNER, 0.0 ) / ph );
		}

		float optic( vec3 p, vec3 q, float ph ) {
			vec3 s = ( q - p ) / float( NUM_OUT_SCATTER );
			vec3 v = p + s * 0.5;
			float sum = 0.0;
			for ( int i = 0; i < NUM_OUT_SCATTER; i++ ) {
				sum += density( v, ph );
				v += s;
			}
			sum *= length( s );
			return sum;
		}

		vec3 in_scatter( vec3 o, vec3 dir, vec2 e, vec3 l ) {
			const float ph_ray = 0.05;
			const float ph_mie = 0.02;
			const vec3 k_ray = vec3( 3.8, 13.5, 33.1 );
			const vec3 k_mie = vec3( 21.0 );
			const float k_mie_ex = 1.1;

			vec3 sum_ray = vec3( 0.0 );
			vec3 sum_mie = vec3( 0.0 );
			float n_ray0 = 0.0;
			float n_mie0 = 0.0;
			float len = ( e.y - e.x ) / float( NUM_IN_SCATTER );
			vec3 s = dir * len;
			vec3 v = o + dir * ( e.x + len * 0.5 );

			for ( int i = 0; i < NUM_IN_SCATTER; i++, v += s ) {
				float d_ray = density( v, ph_ray ) * len;
				float d_mie = density( v, ph_mie ) * len;
				n_ray0 += d_ray;
				n_mie0 += d_mie;

				vec2 f = ray_vs_sphere( v, l, R );
				vec3 u = v + l * f.y;
				float n_ray1 = optic( v, u, ph_ray );
				float n_mie1 = optic( v, u, ph_mie );
				vec3 att = exp( - ( n_ray0 + n_ray1 ) * k_ray - ( n_mie0 + n_mie1 ) * k_mie * k_mie_ex );
				sum_ray += d_ray * att;
				sum_mie += d_mie * att;
			}
			float c  = dot( dir, -l );
			float cc = c * c;
			vec3 scatter = sum_ray * k_ray * phase_ray( cc ) + sum_mie * k_mie * phase_mie( -0.78, c, cc );
			return scatter;
		}

		mat3 rot3xy( vec2 angle ) {
			vec2 c = cos( angle );
			vec2 s = sin( angle );
			return mat3(
				c.y      ,  0.0, -s.y,
				s.y * s.x,  c.x,  c.y * s.x,
				s.y * c.x, -s.x,  c.y * c.x
			);
		}

		vec3 ray_dir( float fov, vec2 size, vec2 uv ) {
			vec2 xy = uv * size - size * 0.5;
			float cot_half_fov = tan( radians( 90.0 - fov * 0.5 ) );
			float z = size.y * 0.5 * cot_half_fov;
			return normalize( vec3( xy, -z ) );
		}

		void mainImage( out vec4 fragColor, in vec2 uv ) {
			vec3 dir = ray_dir( uFov, uResolution.xy, uv );
			vec3 eye = vec3( 0.0, 0.0, uCameraDistance );
			mat3 rot = rot3xy( vec2( 0.0, uTime * uRotationSpeed ) );
			dir = rot * dir;
			eye = rot * eye;
			vec3 l = normalize(uSunDir);
			vec2 e = ray_vs_sphere( eye, dir, R );
			if ( e.x > e.y ) {
				fragColor = vec4( uBackgroundColor, 1.0 );
				return;
			}
			vec2 f = ray_vs_sphere( eye, dir, R_INNER );
			e.y = min( e.y, f.x );
			vec3 I = in_scatter( eye, dir, e, l );
			fragColor = vec4( uBackgroundColor + (I * uIntensity * 10.0), 1.0 );
		}

		void main() {
			vec4 fragColor;
			mainImage(fragColor, vUv);
			gl_FragColor = fragColor;
			#include <colorspace_fragment>
		}
	`;

	$effect(() => {
		resolutionUniform.set($size.width, $size.height);
	});

	$effect(() => {
		sunDirUniform.set(sunX, sunY, sunZ).normalize();
		backgroundColorUniform.set(backgroundColor);

		if (!material) return;
		material.uniforms.uRotationSpeed.value = rotationSpeed;
		material.uniforms.uBackgroundColor.value.copy(backgroundColorUniform);
		material.uniforms.uCameraDistance.value = cameraDistance;
		material.uniforms.uFov.value = fov;
		material.uniforms.uSunDir.value.copy(sunDirUniform);
		material.uniforms.uIntensity.value = intensity;
	});

	useTask((delta) => {
		if (!material) return;
		material.uniforms.uTime.value += delta;
	});
</script>

<T.Mesh>
	<T.PlaneGeometry args={[2, 2]} />
	<T.ShaderMaterial
		bind:ref={material}
		{vertexShader}
		{fragmentShader}
		depthTest={false}
		depthWrite={false}
		uniforms={{
			uTime: { value: 0.0 },
			uResolution: { value: resolutionUniform },
			uBackgroundColor: { value: backgroundColorUniform },
			uRotationSpeed: { value: rotationSpeed },
			uCameraDistance: { value: cameraDistance },
			uFov: { value: fov },
			uSunDir: { value: sunDirUniform.clone() },
			uIntensity: { value: intensity },
		}}
	/>
</T.Mesh>
