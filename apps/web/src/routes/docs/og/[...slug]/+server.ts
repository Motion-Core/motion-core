import { error } from "@sveltejs/kit";
import satoriStandalone, { init as initSatoriWasm } from "satori/standalone";
import { html } from "satori-html";
import { Resvg, initWasm } from "@resvg/resvg-wasm";
import type { RequestHandler } from "./$types";
import { brandLogoRaw, getDocBySlug, getDocMetadata, siteConfig } from "$lib";

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

const fontDataByOrigin = new Map<string, Promise<[ArrayBuffer, ArrayBuffer]>>();

const fetchFontArrayBuffer = async (origin: string, path: string) => {
	const response = await fetch(new URL(path, origin));
	if (!response.ok) {
		throw new Error(`Failed to load font ${path}: ${response.status}`);
	}
	return response.arrayBuffer();
};

const getFontData = (origin: string) => {
	const cached = fontDataByOrigin.get(origin);
	if (cached) {
		return cached;
	}

	const promise = Promise.all([
		fetchFontArrayBuffer(origin, "/fonts/APK-Galeria-Regular.woff"),
		fetchFontArrayBuffer(origin, "/fonts/APK-Galeria-Medium.woff"),
	]).catch((error: unknown) => {
		fontDataByOrigin.delete(origin);
		throw error;
	});
	fontDataByOrigin.set(origin, promise);
	return promise;
};

type ResvgWasmState = {
	promise?: Promise<void>;
	initialized?: boolean;
};

type SatoriWasmState = {
	promise?: Promise<void>;
	initialized?: boolean;
};

let defaultSatoriPromise:
	| Promise<(typeof import("satori"))["default"]>
	| undefined;

const ogWasmState = globalThis as typeof globalThis & {
	__docsOgResvgWasmState?: ResvgWasmState;
	__docsOgResvgWasmModule?: WebAssembly.Module;
	__docsOgSatoriWasmState?: SatoriWasmState;
	__docsOgYogaWasmModule?: WebAssembly.Module;
};

if (!ogWasmState.__docsOgResvgWasmState) {
	ogWasmState.__docsOgResvgWasmState = {};
}

if (!ogWasmState.__docsOgSatoriWasmState) {
	ogWasmState.__docsOgSatoriWasmState = {};
}

const ensureResvgWasm = (origin: string) => {
	const state = ogWasmState.__docsOgResvgWasmState as ResvgWasmState;
	if (state.initialized) {
		return Promise.resolve();
	}

	if (!state.promise) {
		const precompiledWasmModule = ogWasmState.__docsOgResvgWasmModule;
		const loadWasm = precompiledWasmModule
			? Promise.resolve(precompiledWasmModule)
			: fetch(new URL("/resvg-index_bg.wasm", origin)).then((response) => {
					if (!response.ok) {
						throw new Error(`Failed to load resvg wasm: ${response.status}`);
					}
					return response.arrayBuffer();
				});

		state.promise = loadWasm
			.then((wasmSource) => initWasm(wasmSource))
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

const ensureSatoriWasm = () => {
	const state = ogWasmState.__docsOgSatoriWasmState as SatoriWasmState;
	if (state.initialized || !ogWasmState.__docsOgYogaWasmModule) {
		return Promise.resolve();
	}

	if (!state.promise) {
		state.promise = initSatoriWasm(ogWasmState.__docsOgYogaWasmModule)
			.then(() => {
				state.initialized = true;
			})
			.catch((err: unknown) => {
				const message = err instanceof Error ? err.message : String(err);
				if (message.includes("already initialized")) {
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
	const [apkGaleriaRegular, apkGaleriaMedium] = await getFontData(url.origin);
	await ensureResvgWasm(url.origin);
	const useStandaloneSatori = Boolean(ogWasmState.__docsOgYogaWasmModule);
	if (useStandaloneSatori) {
		await ensureSatoriWasm();
	}

	const markup = html`
		<div
			style="display:flex;flex-direction:column;justify-content:space-between;width:100%;height:100%;padding:40px;background:#ffffff;font-family:FK Grotesk Neue,sans-serif;"
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
	const satoriMarkup = markup as unknown as Parameters<
		typeof satoriStandalone
	>[0];

	const renderSatori = async () => {
		if (useStandaloneSatori) {
			return satoriStandalone(satoriMarkup, {
				width: OG_WIDTH,
				height: OG_HEIGHT,
				fonts: [
					{
						name: "APK Galeria Regular",
						data: apkGaleriaRegular,
						weight: 400,
						style: "normal",
					},
					{
						name: "APK Galeria Medium",
						data: apkGaleriaMedium,
						weight: 600,
						style: "normal",
					},
				],
			});
		}

		if (!defaultSatoriPromise) {
			defaultSatoriPromise = import("satori").then((module) => module.default);
		}
		const defaultSatori = await defaultSatoriPromise;
		return defaultSatori(satoriMarkup, {
			width: OG_WIDTH,
			height: OG_HEIGHT,
			fonts: [
				{
					name: "APK Galeria Regular",
					data: apkGaleriaRegular,
					weight: 400,
					style: "normal",
				},
				{
					name: "APK Galeria Medium",
					data: apkGaleriaMedium,
					weight: 600,
					style: "normal",
				},
			],
		});
	};

	const svg = await renderSatori();
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
