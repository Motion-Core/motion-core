import { error } from "@sveltejs/kit";
import ImageResponse from "@takumi-rs/image-response";
import type { RequestHandler } from "./$types";
import apkGaleriaRegularDataUri from "$lib/assets/fonts/APK-Galeria-Regular.woff2?inline";
import apkGaleriaMediumDataUri from "$lib/assets/fonts/APK-Galeria-Medium.woff2?inline";
import { brandLogoRaw, getDocBySlug, getDocMetadata, siteConfig } from "$lib";

const OG_WIDTH = 1200;
const OG_HEIGHT = 630;
const MAX_TITLE_LENGTH = 88;
const MAX_DESCRIPTION_LENGTH = 180;
const canonicalOrigin = new URL(siteConfig.url).origin;

type TakumiElement = {
	type: string;
	props: Record<string, unknown>;
	key: string | null;
};

type TakumiChild = TakumiElement | string;

const el = (
	type: string,
	props: Record<string, unknown> = {},
	...children: TakumiChild[]
): TakumiElement => ({
	type,
	key: null,
	props:
		children.length === 0
			? props
			: {
					...props,
					children: children.length === 1 ? children[0] : children,
				},
});

const clampText = (value: string, maxLength: number) => {
	const text = value.trim();
	if (text.length <= maxLength) return text;
	return `${text.slice(0, maxLength - 1).trimEnd()}…`;
};

const dataUriToArrayBuffer = (dataUri: string) => {
	const base64 = dataUri.slice(dataUri.indexOf(",") + 1);
	const binary = atob(base64);
	const bytes = new Uint8Array(binary.length);
	for (let index = 0; index < binary.length; index += 1) {
		bytes[index] = binary.charCodeAt(index);
	}
	return bytes.buffer;
};

const fontDataPromise = Promise.all([
	Promise.resolve(dataUriToArrayBuffer(apkGaleriaRegularDataUri)),
	Promise.resolve(dataUriToArrayBuffer(apkGaleriaMediumDataUri)),
]);

const takumiFontLoaders = [
	{
		key: "apk-galeria-regular",
		name: "APK Galeria",
		weight: 400,
		style: "normal" as const,
		data: async () => (await fontDataPromise)[0],
	},
	{
		key: "apk-galeria-medium",
		name: "APK Galeria",
		weight: 500,
		style: "normal" as const,
		data: async () => (await fontDataPromise)[1],
	},
];

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

export const GET: RequestHandler = async ({ params }) => {
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

	const component = el(
		"div",
		{
			style: {
				display: "flex",
				flexDirection: "column",
				justifyContent: "space-between",
				width: "100%",
				height: "100%",
				padding: 40,
				background: "#ffffff",
				fontFamily: "APK Galeria, sans-serif",
			},
		},
		el(
			"div",
			{
				style: {
					display: "flex",
					alignItems: "flex-start",
					justifyContent: "space-between",
				},
			},
			el("img", {
				src: logoDataUri,
				alt: "",
				style: {
					display: "flex",
					width: logoDisplayWidth,
					height: LOGO_DISPLAY_HEIGHT,
				},
			}),
			el(
				"div",
				{
					style: {
						display: "flex",
						fontSize: 24,
						color: "#8a8f98",
						fontWeight: 400,
					},
				},
				pageUrl,
			),
		),
		el(
			"div",
			{
				style: {
					display: "flex",
					flexDirection: "column",
					gap: 24,
				},
			},
			el(
				"div",
				{
					style: {
						display: "flex",
						fontSize: 21,
						letterSpacing: "0.06em",
						textTransform: "uppercase",
						color: "#8a8f98",
						fontWeight: 400,
					},
				},
				category,
			),
			el(
				"div",
				{
					style: {
						display: "flex",
						maxWidth: 1060,
						fontSize: 98,
						lineHeight: 0.99,
						color: "#111318",
						fontWeight: 400,
					},
				},
				title,
			),
			el(
				"div",
				{
					style: {
						display: "flex",
						maxWidth: 1020,
						fontSize: 36,
						lineHeight: 1.28,
						color: "#5f6672",
						fontWeight: 400,
					},
				},
				description,
			),
		),
	);

	const response = new ImageResponse(component, {
		width: OG_WIDTH,
		height: OG_HEIGHT,
		format: "png",
		fonts: takumiFontLoaders,
		headers: {
			"content-type": "image/png",
			"cache-control": "public, max-age=3600",
		},
	});

	await response.ready;
	return response;
};
