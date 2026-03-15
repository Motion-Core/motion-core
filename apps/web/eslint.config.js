import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import globals from "globals";

/** @type {import('eslint').Linter.Config[]} */
export default [
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs["flat/recommended"],
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
			},
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts"],
		languageOptions: {
			parserOptions: {
				parser: ts.parser,
			},
		},
	},
	{
		rules: {
			"svelte/no-at-html-tags": "off",
			"svelte/no-navigation-without-resolve": "off",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
					caughtErrorsIgnorePattern: "^_",
				},
			],
		},
	},
	{
		ignores: ["build/", ".svelte-kit/", "dist/"],
	},
];
