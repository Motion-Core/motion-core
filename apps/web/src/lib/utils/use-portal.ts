import { tick } from "svelte";

/**
 * Usage: <div use:portal={'css selector'} or use:portal={document.body}>
 *
 * @param node
 * @param target
 */
export function portal(
	node: HTMLElement,
	target: string | HTMLElement = "body",
) {
	let targetEl: HTMLElement | null;

	async function update(newTarget: string | HTMLElement) {
		target = newTarget;
		if (typeof target === "string") {
			targetEl = document.querySelector(target);
			if (targetEl === null) {
				await tick();
				targetEl = document.querySelector(target);
			}
			if (targetEl === null) {
				// If target is still null, we fail silently or log a warning
				// defaulting to body is an option, but sticking to explicit target is safer
				console.warn(`Portal target "${target}" not found. Appending to body.`);
				targetEl = document.body;
			}
		} else if (target instanceof HTMLElement) {
			targetEl = target;
		} else {
			throw new Error(
				`Unknown portal target type: ${
					target === null ? "null" : typeof target
				}. Allowed types: string (CSS selector) or HTMLElement.`,
			);
		}
		targetEl.appendChild(node);
	}

	function destroy() {
		if (node.parentNode) {
			node.parentNode.removeChild(node);
		}
	}

	update(target);

	return {
		update,
		destroy,
	};
}
