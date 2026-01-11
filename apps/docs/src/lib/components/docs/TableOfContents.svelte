<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import { cn } from "$lib/utils/cn";
	import { page } from "$app/state";

	type TocItem = {
		id: string;
		text: string;
		level: number;
		element: HTMLElement;
	};

	type IndicatorRange = {
		startId: string;
		endId: string;
	};

	type PathPoint = {
		x: number;
		y: number;
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
	let indicatorBottom = $state(0);
	let lineHeight = $state(0);
	let svgPath = $state("");
	let svgWidth = $state(40);
	let indicatorRange = $state<IndicatorRange | null>(null);
	let pendingIndicatorFrame: number | null = null;

	const ACTIVE_OFFSET = 140;
	const VISIBLE_BUFFER = 24;
	const CORNER_RADIUS = 2;
	const linkRefs = new SvelteMap<string, HTMLAnchorElement>();
	const linkPositions = new SvelteMap<
		string,
		{ top: number; height: number }
	>();
	const headingOrder = new SvelteMap<string, number>();
	let linksWrapper = $state<HTMLOListElement | null>(null);

	const currentPath = $derived(page.url.pathname);

	const slugify = (value: string) =>
		value
			.normalize("NFD")
			.replace(/[\u0300-\u036f]/g, "")
			.toLowerCase()
			.trim()
			.replace(/[^a-z0-9]+/g, "-")
			.replace(/^-+|-+$/g, "");

	function registerLink(node: HTMLElement, id?: string) {
		let currentId = id ?? "";

		const assign = () => {
			if (!currentId) return;
			linkRefs.set(currentId, node as HTMLAnchorElement);
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
			},
		};
	}

	function buildRoundedPath(points: PathPoint[], radius: number) {
		if (points.length === 0) return "";
		if (points.length === 1) {
			const [point] = points;
			return `M ${point.x} ${point.y}`;
		}

		const commands: string[] = [`M ${points[0].x} ${points[0].y}`];

		for (let i = 1; i < points.length; i++) {
			const point = points[i];
			const prev = points[i - 1];

			if (i === points.length - 1) {
				commands.push(` L ${point.x} ${point.y}`);
				continue;
			}

			const next = points[i + 1];
			const prevVecX = point.x - prev.x;
			const prevVecY = point.y - prev.y;
			const nextVecX = next.x - point.x;
			const nextVecY = next.y - point.y;
			const prevLen = Math.hypot(prevVecX, prevVecY);
			const nextLen = Math.hypot(nextVecX, nextVecY);

			if (prevLen === 0 || nextLen === 0) {
				commands.push(` L ${point.x} ${point.y}`);
				continue;
			}

			const prevDirX = prevVecX / prevLen;
			const prevDirY = prevVecY / prevLen;
			const nextDirX = nextVecX / nextLen;
			const nextDirY = nextVecY / nextLen;
			const dot = prevDirX * nextDirX + prevDirY * nextDirY;

			if (Math.abs(dot) > 0.999) {
				commands.push(` L ${point.x} ${point.y}`);
				continue;
			}

			const cornerRadius = Math.min(radius, prevLen / 2, nextLen / 2);
			const entryX = point.x - prevDirX * cornerRadius;
			const entryY = point.y - prevDirY * cornerRadius;
			const exitX = point.x + nextDirX * cornerRadius;
			const exitY = point.y + nextDirY * cornerRadius;

			commands.push(` L ${entryX} ${entryY}`);
			commands.push(` Q ${point.x} ${point.y} ${exitX} ${exitY}`);
		}

		return commands.join("");
	}

	function updateLayout() {
		if (!linksWrapper || headings.length === 0) {
			lineHeight = 0;
			return;
		}

		linkPositions.clear();
		const polyline: PathPoint[] = [];
		let maxW = 0;
		const indentStep = 12;
		const strokeWidth = 1;
		const halfStroke = strokeWidth / 2;

		headings.forEach((heading) => {
			const node = linkRefs.get(heading.id);
			if (!node) return;

			const style = window.getComputedStyle(node);
			const paddingTop = parseFloat(style.paddingTop) || 0;
			const paddingBottom = parseFloat(style.paddingBottom) || 0;
			const positionTop = node.offsetTop + paddingTop;
			const positionBottom = node.offsetTop + node.offsetHeight - paddingBottom;
			const positionHeight = Math.max(0, positionBottom - positionTop);

			linkPositions.set(heading.id, {
				top: positionTop,
				height: positionHeight,
			});

			const x = (heading.level - 2) * indentStep + halfStroke;
			const top = positionTop;
			const bottom = Math.max(positionTop, positionBottom);

			polyline.push({ x, y: top });
			polyline.push({ x, y: bottom });

			maxW = Math.max(maxW, x + halfStroke);
		});

		svgPath = buildRoundedPath(polyline, CORNER_RADIUS);
		svgWidth = Math.max(40, maxW + 10);
		lineHeight = linksWrapper.scrollHeight;
	}

	function updateIndicator(range?: IndicatorRange) {
		const appliedRange =
			range ??
			indicatorRange ??
			(activeId ? { startId: activeId, endId: activeId } : null);

		if (!appliedRange) {
			indicatorRange = null;
			indicatorTop = 0;
			indicatorHeight = 0;
			indicatorBottom = 0;
			return;
		}

		if (range) {
			indicatorRange = range;
		} else if (!indicatorRange) {
			indicatorRange = appliedRange;
		}

		const startPos = linkPositions.get(appliedRange.startId);
		const endPos = linkPositions.get(appliedRange.endId);

		if (!startPos || !endPos) {
			indicatorTop = 0;
			indicatorHeight = 0;
			indicatorBottom = 0;
			return;
		}

		const top = Math.min(startPos.top, endPos.top);
		const bottom = Math.max(
			startPos.top + startPos.height,
			endPos.top + endPos.height,
		);

		indicatorTop = top;
		indicatorHeight = Math.max(0, bottom - top);
		indicatorBottom = bottom;
	}

	function scheduleIndicatorUpdate(range?: IndicatorRange | null) {
		if (typeof window === "undefined") {
			if (range) {
				updateIndicator(range);
			} else {
				updateIndicator();
			}
			return;
		}

		if (pendingIndicatorFrame !== null) {
			window.cancelAnimationFrame(pendingIndicatorFrame);
		}

		pendingIndicatorFrame = window.requestAnimationFrame(() => {
			pendingIndicatorFrame = null;
			if (range) {
				updateIndicator(range);
			} else {
				updateIndicator();
			}
		});
	}

	function collectHeadings() {
		if (typeof document === "undefined") {
			headings = [];
			activeId = "";
			lineHeight = 0;
			indicatorTop = 0;
			indicatorHeight = 0;
			indicatorBottom = 0;
			indicatorRange = null;
			headingOrder.clear();
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

		headingOrder.clear();
		parsed.forEach(({ id }, index) => {
			headingOrder.set(id, index);
		});

		headings = parsed.map(({ element: _element, ...rest }) => rest);
		activeId = parsed[0]?.id ?? "";

		lineHeight = 0;
		indicatorTop = 0;
		indicatorHeight = 0;
		indicatorBottom = 0;
		indicatorRange = null;

		requestAnimationFrame(() => updateLayout());

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
			const containerBounds = isWindow
				? { top: 0, bottom: viewportHeight }
				: (container as HTMLElement).getBoundingClientRect();
			const viewportTop = containerBounds.top - VISIBLE_BUFFER;
			const viewportBottom = containerBounds.bottom + VISIBLE_BUFFER;
			const visibleIds: string[] = [];

			for (const item of parsed) {
				const rect = item.element.getBoundingClientRect();
				if (rect.bottom >= viewportTop && rect.top <= viewportBottom) {
					visibleIds.push(item.id);
				}
				if (rect.top - ACTIVE_OFFSET <= viewportTop + VISIBLE_BUFFER) {
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
			const range: IndicatorRange | null =
				visibleIds.length > 0
					? {
							startId: visibleIds[0],
							endId: visibleIds[visibleIds.length - 1],
						}
					: current
						? { startId: current, endId: current }
						: null;

			scheduleIndicatorUpdate(range);
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
				const initialRange: IndicatorRange = {
					startId: activeId,
					endId: activeId,
				};
				scheduleIndicatorUpdate(initialRange);
			} else {
				updateActive();
			}
		}

		const handleResize = () => {
			updateActive();
			updateLayout();
		};

		container.addEventListener("scroll", updateActive, { passive: true });
		window.addEventListener("resize", handleResize);

		return () => {
			container.removeEventListener("scroll", updateActive);
			window.removeEventListener("resize", handleResize);
			if (pendingIndicatorFrame !== null) {
				window.cancelAnimationFrame(pendingIndicatorFrame);
				pendingIndicatorFrame = null;
			}
		};
	}

	function isLinkHighlighted(id: string) {
		if (!indicatorRange) {
			return activeId === id;
		}

		const startIndex = headingOrder.get(indicatorRange.startId);
		const endIndex = headingOrder.get(indicatorRange.endId);
		const currentIndex = headingOrder.get(id);

		if (
			startIndex === undefined ||
			endIndex === undefined ||
			currentIndex === undefined
		) {
			return activeId === id;
		}

		const min = Math.min(startIndex, endIndex);
		const max = Math.max(startIndex, endIndex);

		return currentIndex >= min && currentIndex <= max;
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
		if (typeof window === "undefined" || !linksWrapper) return;

		const observer = new ResizeObserver(() => {
			updateLayout();
			updateIndicator();
		});

		observer.observe(linksWrapper);

		return () => {
			observer.disconnect();
		};
	});
</script>

{#if headings.length > 0}
	<nav class="sticky top-12 hidden px-3 lg:block">
		<div
			class="mb-2 flex items-center gap-2 text-xs font-medium tracking-wide text-foreground/45 uppercase"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				aria-hidden="true"
				width="16"
				height="16"
				fill="currentColor"
				viewBox="0 0 256 256"
				><path
					d="M224,128a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM40,72H216a8,8,0,0,0,0-16H40a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16Z"
				></path></svg
			>
			On this page
		</div>
		<div class="relative flex px-2">
			<div
				class="pointer-events-none absolute top-0 left-1 h-full w-10"
				style={`
                    mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 ${svgWidth} ${lineHeight}' width='${svgWidth}' height='${lineHeight}' preserveAspectRatio='none'%3E%3Cpath d='${svgPath}' stroke='black' stroke-width='1' fill='none'/%3E%3C/svg%3E");
                    -webkit-mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 ${svgWidth} ${lineHeight}' width='${svgWidth}' height='${lineHeight}' preserveAspectRatio='none'%3E%3Cpath d='${svgPath}' stroke='black' stroke-width='1' fill='none'/%3E%3C/svg%3E");
                    mask-repeat: no-repeat;
                    -webkit-mask-repeat: no-repeat;
                    mask-position: left top;
                    -webkit-mask-position: left top;
                    mask-size: 100% 100%;
                    -webkit-mask-size: 100% 100%;
                `}
			>
				<div class="absolute inset-0 h-full w-full bg-border"></div>

				{#if indicatorHeight > 0}
					<div
						class="absolute left-0 w-full bg-accent transition-all duration-450 ease-out"
						style={`
                            top: ${indicatorTop}px;
                            bottom: ${Math.max(0, lineHeight - indicatorBottom)}px;
                        `}
					></div>
				{/if}
			</div>

			<ol class="relative flex flex-col pl-3 text-sm" bind:this={linksWrapper}>
				{#each headings as heading (heading.id)}
					<li
						class="transition-colors duration-150 ease-out"
						style={`padding-left: ${(heading.level - 2) * 12}px`}
					>
						<a
							href={`#${heading.id}`}
							class={cn(
								"block py-1.5 transition-[color] duration-150 ease-out",
								isLinkHighlighted(heading.id)
									? "text-accent"
									: "text-foreground/70 hover:text-foreground",
							)}
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
