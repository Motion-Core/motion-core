<script lang="ts">
	import { onMount } from "svelte";
	import gsap from "gsap";
	import { cn } from "../../utils/cn";

	interface Props {
		texts: string[];
		interval?: number;
		class?: string;
	}

	let { texts, interval = 2000, class: className }: Props = $props();
	let currentIndex = $state(0);
	let isFirst = $state(true);

	function gsapTransition(
		node: HTMLElement,
		{ direction }: { direction: "in" | "out" },
	) {
		gsap.killTweensOf(node);
		const parent = node.parentElement;

		if (direction === "in") {
			if (isFirst) {
				gsap.set(node, { yPercent: 0, opacity: 1, filter: "blur(0px)" });
				return { duration: 0, tick: () => {} };
			}

			if (parent) {
				gsap.fromTo(
					parent,
					{ width: parent.offsetWidth },
					{ width: node.offsetWidth, duration: 0.35, ease: "power2.inOut" },
				);
			}

			gsap.fromTo(
				node,
				{ yPercent: 50, opacity: 0, filter: "blur(6px)" },
				{
					yPercent: 0,
					opacity: 1,
					filter: "blur(0px)",
					duration: 0.3,
					delay: 0.25,
					ease: "back.out(1.2)",
				},
			);
			return { duration: 900, tick: () => {} };
		}

		if (parent) parent.style.width = `${parent.offsetWidth}px`;
		Object.assign(node.style, {
			position: "absolute",
			top: "0",
			left: "0",
			width: "100%",
		});
		gsap.to(node, {
			yPercent: -50,
			opacity: 0,
			duration: 0.2,
			ease: "power2.in",
		});
		return { duration: 300, tick: () => {} };
	}

	onMount(() => {
		const loopInterval = setInterval(() => {
			if (document.hidden) return;
			isFirst = false;
			currentIndex = (currentIndex + 1) % texts.length;
		}, interval);
		return () => clearInterval(loopInterval);
	});
</script>

<span
	class={cn(
		"relative inline-block align-middle font-inherit text-inherit",
		className,
	)}
	style="clip-path: inset(-100vh 0 -100vh 0);"
>
	{#key currentIndex}
		<span
			class={cn(
				"whitespace-nowrap font-inherit text-inherit",
				isFirst ? "relative inline-block" : "absolute left-0 top-0",
			)}
			in:gsapTransition={{ direction: "in" }}
			out:gsapTransition={{ direction: "out" }}
		>
			{texts[currentIndex]}
		</span>
	{/key}
</span>
