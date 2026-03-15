import { error } from "@sveltejs/kit";
import satori from "satori";
import { html } from "satori-html";
import { Resvg, initWasm } from "@resvg/resvg-wasm";
import type { RequestHandler } from "./$types";
import {
	brandLogoRaw,
	getDocBySlug,
	getDocMetadata,
	aeonikProRegularDataUri,
	aeonikProSemiBoldDataUri,
	siteConfig,
} from "$lib";

const OG_WIDTH = 1200;
const OG_HEIGHT = 630;
const MAX_TITLE_LENGTH = 88;
const MAX_DESCRIPTION_LENGTH = 180;
const canonicalOrigin = new URL(siteConfig.url).origin;

const clampText = (value: string, maxLength: number) => {
	const text = value.trim();
	if (text.length <= maxLength) return text;
	return `${text.slice(0, maxLength - 1).trimEnd()}…`;
};

const dataUriToArrayBuffer = (dataUri: string) => {
	const base64 = dataUri.slice(dataUri.indexOf(",") + 1);

	if (typeof Buffer !== "undefined") {
		const bytes = Buffer.from(base64, "base64");
		return bytes.buffer.slice(
			bytes.byteOffset,
			bytes.byteOffset + bytes.byteLength,
		);
	}

	const binary = atob(base64);
	const bytes = new Uint8Array(binary.length);
	for (let index = 0; index < binary.length; index += 1) {
		bytes[index] = binary.charCodeAt(index);
	}
	return bytes.buffer;
};

const fontDataPromise = Promise.all([
	Promise.resolve(dataUriToArrayBuffer(aeonikProRegularDataUri)),
	Promise.resolve(dataUriToArrayBuffer(aeonikProSemiBoldDataUri)),
]);

type ResvgWasmState = {
	promise?: Promise<void>;
	initialized?: boolean;
};

const resvgState = globalThis as typeof globalThis & {
	__docsOgResvgWasmState?: ResvgWasmState;
};
if (!resvgState.__docsOgResvgWasmState) {
	resvgState.__docsOgResvgWasmState = {};
}

const ensureResvgWasm = (origin: string) => {
	const state = resvgState.__docsOgResvgWasmState as ResvgWasmState;
	if (state.initialized) {
		return Promise.resolve();
	}

	if (!state.promise) {
		state.promise = fetch(new URL("/resvg-index_bg.wasm", origin))
			.then((response) => {
				if (!response.ok) {
					throw new Error(`Failed to load resvg wasm: ${response.status}`);
				}
				return response.arrayBuffer();
			})
			.then((buffer) => initWasm(buffer))
			.then(() => {
				state.initialized = true;
			})
			.catch((err: unknown) => {
				const message = err instanceof Error ? err.message : String(err);
				if (message.includes("Already initialized")) {
					state.initialized = true;
					return;
				}
				state.promise = undefined;
				throw err;
			});
	}
	return state.promise;
};

const logoDataUri = `data:image/svg+xml,${encodeURIComponent(
	brandLogoRaw.replaceAll("currentColor", "#ff6900"),
)}`;
const LOGO_DISPLAY_HEIGHT = 78;

const extractLogoAspectRatio = (svgMarkup: string) => {
	const viewBoxMatch = svgMarkup.match(/viewBox="([^"]+)"/i);
	if (viewBoxMatch) {
		const [, rawViewBox] = viewBoxMatch;
		const values = rawViewBox
			.trim()
			.split(/\s+/)
			.map((value) => Number(value));
		if (
			values.length === 4 &&
			Number.isFinite(values[2]) &&
			Number.isFinite(values[3]) &&
			values[2] > 0 &&
			values[3] > 0
		) {
			return values[2] / values[3];
		}
	}

	const widthMatch = svgMarkup.match(/width="([^"]+)"/i);
	const heightMatch = svgMarkup.match(/height="([^"]+)"/i);
	if (widthMatch && heightMatch) {
		const width = Number(widthMatch[1]);
		const height = Number(heightMatch[1]);
		if (
			Number.isFinite(width) &&
			Number.isFinite(height) &&
			width > 0 &&
			height > 0
		) {
			return width / height;
		}
	}

	return 1;
};

const logoDisplayWidth = Math.round(
	LOGO_DISPLAY_HEIGHT * extractLogoAspectRatio(brandLogoRaw),
);

export const GET: RequestHandler = async ({ params, url }) => {
	const rawSlug = (params.slug ?? "").replace(/^\/+|\/+$/g, "");
	const slug =
		rawSlug === "" || rawSlug === "index" || rawSlug === "docs" ? "" : rawSlug;

	const metadata = getDocMetadata(`/docs/${slug}`);
	if (!metadata) {
		throw error(404, "Document not found");
	}

	const category = getDocBySlug(metadata.slug)?.category ?? "Documentation";
	const title = clampText(metadata.title, MAX_TITLE_LENGTH);
	const description = clampText(
		metadata.description ?? "Documentation for Motion GPU.",
		MAX_DESCRIPTION_LENGTH,
	);
	const pageUrl = new URL(`/docs/${metadata.slug}`, canonicalOrigin).href;
	const [aeonikProRegular, aeonikProSemiBold] = await fontDataPromise;
	await ensureResvgWasm(url.origin);

	const markup = html`
		<div
			style="display:flex;flex-direction:column;justify-content:space-between;width:100%;height:100%;padding:40px;background:#ffffff;font-family:Aeonik Pro Regular,sans-serif;"
		>
			<div
				style="display:flex;align-items:flex-start;justify-content:space-between;"
			>
				<img
					src="${logoDataUri}"
					alt=""
					style="display:flex;width:${logoDisplayWidth}px;height:${LOGO_DISPLAY_HEIGHT}px;"
				/>
				<div style="display:flex;font-size:24px;color:#8a8f98;font-weight:400;">
					${pageUrl}
				</div>
			</div>
			<div style="display:flex;flex-direction:column;gap:24px;">
				<div
					style="display:flex;font-size:21px;letter-spacing:0.06em;text-transform:uppercase;color:#8a8f98;font-weight:400;"
				>
					${category}
				</div>
				<div
					style="display:flex;max-width:1060px;font-size:98px;line-height:0.99;color:#111318;font-weight:500;"
				>
					${title}
				</div>
				<div
					style="display:flex;max-width:1020px;font-size:36px;line-height:1.28;color:#5f6672;font-weight:400;"
				>
					${description}
				</div>
			</div>
		</div>
	`;

	const svg = await satori(markup, {
		width: OG_WIDTH,
		height: OG_HEIGHT,
		fonts: [
			{
				name: "Aeonik Pro Regular",
				data: aeonikProRegular,
				weight: 400,
				style: "normal",
			},
			{
				name: "Aeonik Pro SemiBold",
				data: aeonikProSemiBold,
				weight: 600,
				style: "normal",
			},
		],
	});
	const rendered = new Resvg(svg, {
		fitTo: { mode: "width", value: OG_WIDTH },
	}).render();
	const png = rendered.asPng();
	const pngBody = new Uint8Array(png.byteLength);
	pngBody.set(png);

	return new Response(pngBody, {
		headers: {
			"content-type": "image/png",
			"cache-control": "public, max-age=3600",
		},
	});
};
