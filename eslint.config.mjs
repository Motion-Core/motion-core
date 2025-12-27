import js from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import svelteParser from "svelte-eslint-parser";
import tseslint from "typescript-eslint";

/** @type {import("eslint").Linter.Config[]} */
export default [
	{
		ignores: [
			"**/node_modules/**",
			"**/dist/**",
			"**/build/**",
			"**/.svelte-kit/**",
			"apps/docs/.svelte-kit/**",
			"apps/docs/static/**",
			"apps/docs/svelte-check-run.log",
			"apps/docs/.vercel/**",
			"packages/**/dist/**",
			".vercel/**",
		],
	},
	...svelte.configs["flat/recommended"],
	...svelte.configs["flat/prettier"],
	js.configs.recommended,
	...tseslint.configs.recommended,
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: tseslint.parser,
				extraFileExtensions: [".svelte"],
			},
		},
		rules: {
			"no-undef": "off",
			"svelte/no-navigation-without-resolve": "off",
			"svelte/no-at-html-tags": "off",
		},
	},
	{
		settings: {
			svelte: {
				kit: {
					files: {
						routes: "apps/docs/src/routes",
					},
				},
			},
		},
		rules: {
			"@typescript-eslint/no-unused-vars": [
				"warn",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
					ignoreRestSiblings: true,
				},
			],
			"no-console": [
				"warn",
				{
					allow: ["warn", "error"],
				},
			],
		},
	},
	eslintConfigPrettier,
];
