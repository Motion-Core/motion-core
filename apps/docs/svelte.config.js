import { fileURLToPath, URL } from "node:url";
import adapter from "@sveltejs/adapter-vercel";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { escapeSvelte, mdsvex } from "mdsvex";
import { createHighlighter } from "shiki";

const tableCellFormatter = () => {
	return (tree) => {
		const ancestors = [];

		const visit = (node, parent = null, index = 0) => {
			if (!node) return;

			const isElement = node.type === "element";
			const isRoot = node.type === "root";

			if (isElement) {
				ancestors.push(node);
			}

			if (
				node.type === "text" &&
				typeof node.value === "string" &&
				node.value.includes("\\|")
			) {
				const directParent = ancestors[ancestors.length - 1];
				const grandParent = ancestors[ancestors.length - 2];
				const isCodeBlock =
					directParent?.tagName === "code" && grandParent?.tagName === "pre";

				if (!isCodeBlock) {
					node.value = node.value.replace(/\\\|/g, "|");
				}
			}

			if (
				isElement &&
				node.tagName === "code" &&
				node.children?.length === 1 &&
				node.children[0]?.type === "text" &&
				typeof node.children[0]?.value === "string"
			) {
				const parentNode = ancestors[ancestors.length - 2];
				const isBlockCode = parentNode?.tagName === "pre";
				const insideTableCell = ancestors.some(
					(ancestor) =>
						ancestor !== node &&
						ancestor.type === "element" &&
						(ancestor.tagName === "td" || ancestor.tagName === "th"),
				);

				let raw = node.children[0].value;
				if (raw.includes("\\|")) {
					raw = raw.replace(/\\\|/g, "|");
					node.children[0].value = raw;
				}

				if (
					!isBlockCode &&
					insideTableCell &&
					raw.includes("|") &&
					parent?.children
				) {
					const segments = raw.split("|").map((segment) => segment.trim());
					if (segments.length > 1) {
						const replacements = segments.flatMap((segment, segmentIndex) => {
							const codeNode = {
								type: "element",
								tagName: "code",
								properties: node.properties ?? {},
								children: [
									{
										type: "text",
										value: segment,
									},
								],
							};

							if (segmentIndex === segments.length - 1) {
								return [codeNode];
							}

							return [codeNode, { type: "text", value: " " }];
						});

						parent.children.splice(index, 1, ...replacements);
						ancestors.pop();
						replacements.forEach((child, childIndex) =>
							visit(child, parent, index + childIndex),
						);
						return;
					}
				}
			}

			const childNodes = isElement || isRoot ? (node.children ?? []) : [];
			for (let i = 0; i < childNodes.length; i += 1) {
				visit(childNodes[i], node, i);
			}

			if (isElement) {
				ancestors.pop();
			}
		};

		visit(tree);
	};
};

const themes = {
	light: "github-light",
	dark: "github-dark",
};
const highlighter = await createHighlighter({
	themes: Object.values(themes),
	langs: ["svelte", "bash", "json", "typescript"],
});

const markdownLayout = fileURLToPath(
	new URL("./src/lib/components/docs/MarkdownLayout.svelte", import.meta.url),
);

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: [".svelte", ".svx"],
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [
		mdsvex({
			extensions: [".svx"],
			layout: {
				docs: markdownLayout,
			},
			rehypePlugins: [tableCellFormatter],
			highlight: {
				highlighter: async (code, lang = "text") => {
					const lightHtml = escapeSvelte(
						highlighter.codeToHtml(code, {
							lang,
							theme: themes.light,
						}),
					);
					const darkHtml = escapeSvelte(
						highlighter.codeToHtml(code, {
							lang,
							theme: themes.dark,
						}),
					);
					const htmlLightProp = JSON.stringify(lightHtml);
					const htmlDarkProp = JSON.stringify(darkHtml);
					const langProp = JSON.stringify(lang);
					const rawProp = JSON.stringify(code);
					return `<svelte:component this={Reflect.get(globalThis, "__MarkdownPre")} lang={${langProp}} htmlLight={${htmlLightProp}} htmlDark={${htmlDarkProp}} raw={${rawProp}} />`;
				},
			},
		}),
		vitePreprocess(),
	],

	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		adapter: adapter(),
		alias: {
			"motion-core": fileURLToPath(
				new URL("../../packages/motion-core/src/lib", import.meta.url),
			),
		},
	},
};

export default config;
