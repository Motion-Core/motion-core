/** @type {import('prettier').Config} */
module.exports = {
	useTabs: true,
	singleQuote: false,
	plugins: ["prettier-plugin-svelte"],
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
