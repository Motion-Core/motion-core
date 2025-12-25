import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import getWasm from "shiki/wasm";
import githubLight from "shiki/themes/github-light.mjs";
import githubDark from "shiki/themes/github-dark.mjs";

import typescript from "shiki/langs/typescript.mjs";
import svelte from "shiki/langs/svelte.mjs";
import xml from "shiki/langs/xml.mjs";
import bash from "shiki/langs/bash.mjs";
import json from "shiki/langs/json.mjs";

let highlighter: Awaited<ReturnType<typeof createHighlighterCore>> | null =
	null;

export async function getHighlighter() {
	if (!highlighter) {
		highlighter = await createHighlighterCore({
			themes: [githubLight, githubDark],
			langs: [typescript, svelte, xml, bash, json],
			engine: createOnigurumaEngine(getWasm),
		});
	}
	return highlighter;
}
