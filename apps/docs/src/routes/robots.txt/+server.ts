import type { RequestHandler } from "@sveltejs/kit";

const directives = ["User-agent: *", "Allow: /"];

const toSitemapUrl = (origin: string) => new URL("/sitemap.xml", origin).href;

export const GET: RequestHandler = ({ url }) => {
	const lines = [...directives, `Sitemap: ${toSitemapUrl(url.origin)}`];
	const body = lines.join("\n");

	return new Response(body, {
		headers: {
			"content-type": "text/plain; charset=utf-8",
			"cache-control": "public, max-age=3600",
		},
	});
};
