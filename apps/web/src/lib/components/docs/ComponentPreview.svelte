<script lang="ts">
	import { onDestroy, onMount, tick } from "svelte";
	import { Flip } from "gsap/Flip";
	import { gsap } from "gsap";
	import { cn } from "$lib/utils/cn";
	import CodePanel from "./component-preview/CodePanel.svelte";
	import ControlsPanel from "./component-preview/ControlsPanel.svelte";
	import PreviewFrame from "./component-preview/PreviewFrame.svelte";
	import {
		getDefaultControlValue,
		type ComponentPreviewChildren,
		type ComponentPreviewControl,
		type ComponentPreviewValue,
		type ComponentPreviewValues,
		type SourceTab,
	} from "./component-preview/types";
	import {
		readControlValuesFromSearch,
		writeControlValuesToSearch,
	} from "./component-preview/url-state";

	type ComponentProps = {
		code?: string;
		language?: string;
		label?: string;
		children?: ComponentPreviewChildren;
		codeSlot?: import("svelte").Snippet;
		sources?: SourceTab[];
		controls?: ComponentPreviewControl[];
		refreshOnFullScreen?: boolean;
		refreshOnControlChange?: boolean;
		controlRefreshDelay?: number;
		class?: string;
		[key: string]: unknown;
	};

	const {
		children,
		codeSlot,
		code: providedCode,
		language: providedLanguage,
		label: providedLabel,
		sources: providedSources,
		controls: providedControls = [],
		refreshOnFullScreen = false,
		refreshOnControlChange = false,
		controlRefreshDelay = 120,
		class: className = "",
		...restProps
	}: ComponentProps = $props();

	let isFullScreen = $state(false);
	let previewKey = $state(0);
	let previewRef = $state<HTMLElement>();
	let placeholderRef = $state<HTMLElement>();
	let controlValues = $state<ComponentPreviewValues>({});
	let hasInitializedUrlState = $state(false);
	let controlRefreshTimer: ReturnType<typeof setTimeout> | null = null;

	const controls = $derived(providedControls);
	const tabs = $derived(
		(() => {
			const normalized =
				providedSources?.filter((tab): tab is SourceTab =>
					Boolean(tab?.code),
				) ?? [];

			if (normalized.length > 0) {
				return normalized;
			}

			if (providedCode) {
				return [
					{
						name: providedLabel ?? "Code",
						code: providedCode,
						language: providedLanguage,
					},
				];
			}

			return [];
		})() as SourceTab[],
	);

	const buildDefaultValues = () =>
		Object.fromEntries(
			controls.map((control) => [
				control.name,
				getDefaultControlValue(control),
			]),
		) as ComponentPreviewValues;

	const reloadPreview = () => {
		previewKey += 1;
	};

	const scheduleControlRefresh = () => {
		if (!refreshOnControlChange) return;

		if (controlRefreshTimer) {
			clearTimeout(controlRefreshTimer);
		}

		controlRefreshTimer = setTimeout(() => {
			controlRefreshTimer = null;
			reloadPreview();
		}, controlRefreshDelay);
	};

	const mergeValues = (
		nextValues: ComponentPreviewValues,
	): { values: ComponentPreviewValues; changed: boolean } => {
		const defaults = buildDefaultValues();
		const mergedValues: ComponentPreviewValues = {};
		let changed = false;

		for (const [name, defaultValue] of Object.entries(defaults)) {
			const nextValue = nextValues[name] ?? defaultValue;
			mergedValues[name] = nextValue;

			if (controlValues[name] !== nextValue) {
				changed = true;
			}
		}

		if (
			Object.keys(controlValues).length !== Object.keys(mergedValues).length
		) {
			changed = true;
		}

		return { values: mergedValues, changed };
	};

	const applyControlValues = (
		nextValues: ComponentPreviewValues,
		options: { refresh?: boolean } = {},
	) => {
		const { values, changed } = mergeValues(nextValues);
		if (!changed) return;

		controlValues = values;

		if (options.refresh) {
			scheduleControlRefresh();
		}
	};

	const readUrlState = () => {
		if (typeof window === "undefined") return buildDefaultValues();
		return readControlValuesFromSearch(controls, window.location.search);
	};

	const writeUrlState = () => {
		if (typeof window === "undefined" || !hasInitializedUrlState) return;

		const nextSearch = writeControlValuesToSearch(
			controls,
			controlValues,
			window.location.search,
		);
		const nextUrl = `${window.location.pathname}${nextSearch}${window.location.hash}`;
		const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;

		if (nextUrl === currentUrl) return;
		window.history.replaceState(window.history.state, "", nextUrl);
	};

	const resetControls = () => {
		applyControlValues(buildDefaultValues());
		scheduleControlRefresh();
	};

	const updateControl = (name: string, value: ComponentPreviewValue) => {
		applyControlValues({
			...controlValues,
			[name]: value,
		});
		scheduleControlRefresh();
	};

	$effect(() => {
		applyControlValues(controlValues);
	});

	$effect(() => {
		writeUrlState();
	});

	onMount(() => {
		gsap.registerPlugin(Flip);
		applyControlValues(readUrlState(), { refresh: refreshOnControlChange });
		hasInitializedUrlState = true;

		const handlePopState = () => {
			applyControlValues(readUrlState(), { refresh: refreshOnControlChange });
		};

		window.addEventListener("popstate", handlePopState);

		return () => {
			window.removeEventListener("popstate", handlePopState);
		};
	});

	onDestroy(() => {
		if (controlRefreshTimer) {
			clearTimeout(controlRefreshTimer);
		}
	});

	const toggleFullScreen = async () => {
		if (!previewRef || !placeholderRef) return;

		if (!isFullScreen) {
			const state = Flip.getState(previewRef);
			const rect = previewRef.getBoundingClientRect();
			placeholderRef.style.height = `${rect.height}px`;
			placeholderRef.style.width = `${rect.width}px`;

			isFullScreen = true;
			await tick();

			document.body.appendChild(previewRef);

			previewRef.style.setProperty("position", "fixed", "important");
			previewRef.style.setProperty("top", "0", "important");
			previewRef.style.setProperty("left", "0", "important");
			previewRef.style.setProperty("width", "100vw", "important");
			previewRef.style.setProperty("height", "100dvh", "important");
			previewRef.style.setProperty("margin", "0", "important");

			Flip.from(state, {
				duration: 0.5,
				ease: "power3.inOut",
				absolute: true,
				zIndex: 50,
				onComplete: () => {
					if (refreshOnFullScreen) {
						reloadPreview();
					}
				},
			});
		} else {
			Flip.fit(previewRef, placeholderRef, {
				duration: 0.5,
				ease: "power3.inOut",
				absolute: true,
				zIndex: 50,
				onComplete: () => {
					if (!previewRef || !placeholderRef) return;
					isFullScreen = false;
					placeholderRef.appendChild(previewRef);
					placeholderRef.style.height = "";
					placeholderRef.style.width = "";
					previewRef.style.cssText = "";
					if (refreshOnFullScreen) {
						reloadPreview();
					}
				},
			});
		}
	};
</script>

<section
	class={cn(
		"relative w-full rounded-lg bg-background-inset p-1.5 inset-shadow",
	)}
	{...restProps}
>
	<div class="flex h-full flex-col rounded-md">
		<PreviewFrame
			bind:previewRef
			bind:placeholderRef
			{children}
			values={controlValues}
			{previewKey}
			{isFullScreen}
			class={className}
			onReload={reloadPreview}
			onToggleFullScreen={toggleFullScreen}
		/>
		<ControlsPanel
			{controls}
			values={controlValues}
			onChange={updateControl}
			onReset={resetControls}
		/>
		<CodePanel {tabs} {codeSlot} />
	</div>
</section>
