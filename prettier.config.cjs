/** @type {import('prettier').Config} */
module.exports = {
	useTabs: true,
	singleQuote: false,
	plugins: ["prettier-plugin-svelte", "prettier-plugin-tailwindcss"],
	tailwindStylesheet: "./apps/docs/src/routes/layout.css",
	tailwindFunctions: ["cn", "clsx"],
	overrides: [
		{
			files: "*.svelte",
			options: {
				parser: "svelte",
			},
		},
		{
			files: "*.svx",
			options: {
				parser: "mdx",
			},
		},
	],
};
