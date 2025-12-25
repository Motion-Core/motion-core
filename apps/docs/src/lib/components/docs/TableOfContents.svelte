<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import { cn } from "$lib/utils/cn";
	import { page } from "$app/stores";

	type TocItem = {
		id: string;
		text: string;
		level: number;
		element: HTMLElement;
	};

	type Props = {
		selector?: string;
	};

	const props = $props();
	const selector = $derived(
		(props as Props).selector ?? "[data-doc-content] h2, [data-doc-content] h3",
	);

	let headings = $state<Omit<TocItem, "element">[]>([]);
	let activeId = $state("");
	let indicatorTop = $state(0);
	let indicatorHeight = $state(0);
	let lineHeight = $state(0);

	const ACTIVE_OFFSET = 140;
	const linkRefs = new SvelteMap<string, HTMLAnchorElement>();
	const linkPositions = new SvelteMap<
		string,
		{ top: number; height: number }
	>();
	let linksWrapper = $state<HTMLOListElement | null>(null);
	let rafId: number | null = null;

	const currentPath = $derived($page.url.pathname);

	const slugify = (value: string) =>
		value
			.normalize("NFD")
			.replace(/[\u0300-\u036f]/g, "")
			.toLowerCase()
			.trim()
			.replace(/[^a-z0-9]+/g, "-")
			.replace(/^-+|-+$/g, "");

	function scheduleMeasurement() {
		if (typeof window === "undefined") {
			return;
		}

		if (rafId) {
			window.cancelAnimationFrame(rafId);
		}
		rafId = window.requestAnimationFrame(() => {
			rafId = null;
			measureLinks();
			updateIndicator();
		});
	}

	function registerLink(node: HTMLElement, id?: string) {
		let currentId = id ?? "";

		const assign = () => {
			if (!currentId) return;
			linkRefs.set(currentId, node as HTMLAnchorElement);
			scheduleMeasurement();
		};

		assign();

		return {
			update(newId?: string) {
				if (newId === currentId) return;
				if (currentId) {
					linkRefs.delete(currentId);
					linkPositions.delete(currentId);
				}
				currentId = newId ?? "";
				assign();
			},
			destroy() {
				if (currentId) {
					linkRefs.delete(currentId);
					linkPositions.delete(currentId);
				}
				scheduleMeasurement();
			},
		};
	}

	function measureLinks() {
		if (!linksWrapper) {
			lineHeight = 0;
			return;
		}

		linkPositions.clear();
		for (const [id, node] of linkRefs.entries()) {
			linkPositions.set(id, {
				top: node.offsetTop,
				height: node.offsetHeight,
			});
		}
		lineHeight = linksWrapper.scrollHeight;
	}

	function updateIndicator(targetId = activeId) {
		const pos = linkPositions.get(targetId);

		if (!pos) {
			indicatorTop = 0;
			indicatorHeight = 0;
			return;
		}

		indicatorTop = pos.top;
		indicatorHeight = pos.height;
	}

	function collectHeadings() {
		if (typeof document === "undefined") {
			headings = [];
			activeId = "";
			lineHeight = 0;
			indicatorTop = 0;
			indicatorHeight = 0;
			return undefined;
		}

		const slugCounts = new SvelteMap<string, number>();
		const nodeList = Array.from(document.querySelectorAll(selector)).filter(
			(node): node is HTMLElement => node instanceof HTMLElement,
		);

		const parsed: TocItem[] = [];

		for (const node of nodeList) {
			const text = node.textContent?.trim() ?? "";
			if (!text) continue;

			let id = node.id;
			if (!id) {
				let baseSlug = slugify(text);
				if (!baseSlug) {
					baseSlug = `section-${parsed.length + 1}`;
				}
				const count = slugCounts.get(baseSlug);
				if (typeof count === "number") {
					const nextCount = count + 1;
					slugCounts.set(baseSlug, nextCount);
					baseSlug = `${baseSlug}-${nextCount}`;
				} else {
					slugCounts.set(baseSlug, 0);
				}
				id = baseSlug;
				node.id = id;
			}

			const level = Number(node.tagName.replace("H", "")) || 2;
			parsed.push({
				id,
				text,
				level,
				element: node,
			});
		}

		headings = parsed.map(({ element: _element, ...rest }) => rest);
		activeId = parsed[0]?.id ?? "";
		lineHeight = 0;
		indicatorTop = 0;
		indicatorHeight = 0;
		scheduleMeasurement();

		if (!parsed.length) {
			return undefined;
		}

		const updateActive = () => {
			let current = parsed[0]?.id ?? "";
			const container =
				document.getElementById("docs-content-container") ?? window;
			const isWindow = container === window;
			const scrollY = isWindow
				? window.scrollY
				: (container as HTMLElement).scrollTop;
			const viewportHeight = isWindow
				? window.innerHeight
				: (container as HTMLElement).clientHeight;
			const scrollHeight = isWindow
				? document.documentElement.scrollHeight
				: (container as HTMLElement).scrollHeight;

			for (const item of parsed) {
				const top = item.element.getBoundingClientRect().top;
				if (top - ACTIVE_OFFSET <= 0) {
					current = item.id;
				}
			}

			const last = parsed[parsed.length - 1];
			if (last) {
				const scrolledBottom = scrollY + viewportHeight;
				if (scrolledBottom >= scrollHeight - 20) {
					current = last.id;
				}
			}

			activeId = current;
			window.requestAnimationFrame(() => updateIndicator(current));
		};

		const container =
			document.getElementById("docs-content-container") ?? window;

		if (parsed.length > 0) {
			const isWindow = container === window;
			const scrollY = isWindow
				? window.scrollY
				: (container as HTMLElement).scrollTop;

			if (scrollY < ACTIVE_OFFSET) {
				activeId = parsed[0].id;
				updateIndicator(activeId);
			} else {
				updateActive();
			}
		}

		const handleResize = () => {
			updateActive();
			scheduleMeasurement();
		};

		container.addEventListener("scroll", updateActive, { passive: true });
		window.addEventListener("resize", handleResize);

		return () => {
			container.removeEventListener("scroll", updateActive);
			window.removeEventListener("resize", handleResize);
		};
	}

	$effect(() => {
		const path = currentPath;
		void path;
		let cleanup: (() => void) | undefined;

		const timer = setTimeout(() => {
			cleanup = collectHeadings();
		}, 50);

		return () => {
			clearTimeout(timer);
			cleanup?.();
		};
	});

	$effect(() => {
		if (typeof window === "undefined" || !linksWrapper) {
			return;
		}
		scheduleMeasurement();
	});
</script>

{#if headings.length > 0}
	<nav class="sticky top-14 hidden lg:block">
		<div
			class="mb-4 flex items-center gap-2 text-xs font-medium tracking-wide text-foreground uppercase"
		>
			<svg
				width="15"
				height="15"
				viewBox="0 0 15 15"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
			>
				<path
					d="M2 4.5C2 4.22386 2.22386 4 2.5 4H12.5C12.7761 4 13 4.22386 13 4.5C13 4.77614 12.7761 5 12.5 5H2.5C2.22386 5 2 4.77614 2 4.5ZM2 7.5C2 7.22386 2.22386 7 2.5 7H7.5C7.77614 7 8 7.22386 8 7.5C8 7.77614 7.77614 8 7.5 8H2.5C2.22386 8 2 7.77614 2 7.5ZM2 10.5C2 10.2239 2.22386 10 2.5 10H10.5C10.7761 10 11 10.2239 11 10.5C11 10.7761 10.7761 11 10.5 11H2.5C2.22386 11 2 10.7761 2 10.5Z"
					fill="currentColor"
					fill-rule="evenodd"
					clip-rule="evenodd"
				/>
			</svg>
			On this page
		</div>
		<div class="relative flex">
			<div
				class="relative mr-2 w-px bg-border"
				style={`height:${lineHeight}px`}
			>
				{#if indicatorHeight > 0}
					<div
						class="absolute left-0 w-px bg-accent transition-[transform,height] duration-300 ease-out"
						style={`transform: translateY(${indicatorTop}px); height: ${indicatorHeight}px;`}
					></div>
				{/if}
			</div>
			<ol class="relative flex flex-col text-sm" bind:this={linksWrapper}>
				{#each headings as heading (heading.id)}
					<li>
						<a
							href={`#${heading.id}`}
							class={cn(
								"block px-3 py-1 font-normal transition-[color] duration-150 ease-out hover:text-foreground",
								heading.level > 2 && "pl-6 text-sm",
								activeId === heading.id ? "text-accent" : "text-foreground/70",
							)}
							aria-current={activeId === heading.id ? "location" : undefined}
							use:registerLink={heading.id}
						>
							{heading.text}
						</a>
					</li>
				{/each}
			</ol>
		</div>
	</nav>
{:else}
	<div class="hidden text-sm text-foreground/45 lg:block">No headings</div>
{/if}
