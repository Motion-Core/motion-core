import adapter from "@sveltejs/adapter-auto";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath, URL } from "node:url";
import { mdsvex, escapeSvelte } from "mdsvex";
import { createHighlighter } from "shiki";

const themes = {
	light: "github-light",
	dark: "github-dark",
};
const highlighter = await createHighlighter({
	themes: Object.values(themes),
	langs: ["svelte", "bash"],
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
					}
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
