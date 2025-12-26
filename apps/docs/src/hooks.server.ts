import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
	return resolve(event, {
		preload: ({ type }) => type !== "css",
	});
};
