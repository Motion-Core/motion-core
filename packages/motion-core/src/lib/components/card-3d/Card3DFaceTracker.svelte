<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { FaceLandmarker, FilesetResolver } from "@mediapipe/tasks-vision";
	import { portal } from "../../utils/use-portal";

	interface HeadPosition {
		x: number;
		y: number;
		z: number;
	}

	interface Props {
		/**
		 * Callback fired when head position changes.
		 */
		onHeadMove: (position: HeadPosition) => void;
		/**
		 * Whether to show the video preview.
		 * @default true
		 */
		showPreview?: boolean;
		/**
		 * Additional CSS classes for the container.
		 */
		class?: string;
	}

	let {
		onHeadMove,
		showPreview = true,
		class: className = "",
	}: Props = $props();

	let video = $state<HTMLVideoElement>();
	let faceLandmarker: FaceLandmarker | null = null;
	let animationFrameId: number;
	let isRunning = $state(false);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			const filesetResolver = await FilesetResolver.forVisionTasks(
				"https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@latest/wasm",
			);

			faceLandmarker = await FaceLandmarker.createFromOptions(filesetResolver, {
				baseOptions: {
					modelAssetPath:
						"https://storage.googleapis.com/mediapipe-models/face_landmarker/face_landmarker/float16/1/face_landmarker.task",
					delegate: "GPU",
				},
				runningMode: "VIDEO",
				numFaces: 1,
				outputFaceBlendshapes: false,
				outputFacialTransformationMatrixes: true,
			});

			await startCamera();
		} catch (e) {
			error =
				e instanceof Error ? e.message : "Failed to initialize face tracking";
			console.error("FaceTracker init error:", e);
		}
	});

	onDestroy(() => {
		if (animationFrameId) {
			cancelAnimationFrame(animationFrameId);
		}
		if (video?.srcObject) {
			const tracks = (video.srcObject as MediaStream).getTracks();
			tracks.forEach((track) => track.stop());
		}
		if (faceLandmarker) {
			faceLandmarker.close();
		}
	});

	async function startCamera() {
		if (!video) return;
		try {
			const stream = await navigator.mediaDevices.getUserMedia({
				video: { facingMode: "user", width: 640, height: 480 },
			});
			video.srcObject = stream;
			await video.play();
			isRunning = true;
			detectFace();
		} catch (e) {
			error = "Camera access denied";
			console.error("Camera error:", e);
		}
	}

	function detectFace() {
		if (!faceLandmarker || !video || video.readyState < 2) {
			animationFrameId = requestAnimationFrame(detectFace);
			return;
		}

		const results = faceLandmarker.detectForVideo(video, performance.now());

		if (results.faceLandmarks && results.faceLandmarks.length > 0) {
			const landmarks = results.faceLandmarks[0];

			const nose = landmarks[1];

			const x = (nose.x - 0.5) * 2;
			const y = (nose.y - 0.5) * 2;

			const leftEar = landmarks[234];
			const rightEar = landmarks[454];
			const faceWidth = Math.abs(rightEar.x - leftEar.x);

			const z = (0.4 - faceWidth) * 5;

			onHeadMove({ x, y, z });
		}

		animationFrameId = requestAnimationFrame(detectFace);
	}
</script>

{#if showPreview}
	<div
		use:portal
		class="card-highlight fixed right-4 bottom-4 z-50 rounded-lg border border-border bg-card-muted {className}"
	>
		<video
			bind:this={video}
			playsinline
			muted
			class="h-30 w-40 -scale-x-100 rounded-lg"
		></video>
		{#if error}
			<div
				class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-center text-xs text-accent"
			>
				{error}
			</div>
		{/if}
		{#if !isRunning && !error}
			<div
				class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-center text-xs text-foreground"
			>
				Initializing camera...
			</div>
		{/if}
	</div>
{:else}
	<video bind:this={video} playsinline muted class="hidden"></video>
{/if}
