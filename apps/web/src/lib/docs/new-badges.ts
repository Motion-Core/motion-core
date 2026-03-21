import { docsUiConfig } from "$lib/config/docs-ui";
import { docsManifest as componentDocsManifest } from "./generated-manifest";

const MS_PER_DAY = 24 * 60 * 60 * 1000;

type NewMetadata = {
	introducedAt?: string;
	newUntil?: string;
};

const newMetadataBySlug = new Map<string, NewMetadata>(
	componentDocsManifest.map((doc) => [
		doc.slug,
		{
			introducedAt: doc.introducedAt,
			newUntil: doc.newUntil,
		},
	]),
);

function parseDate(value?: string) {
	if (!value) {
		return null;
	}

	const timestamp = Date.parse(value);
	return Number.isFinite(timestamp) ? timestamp : null;
}

export function isNewComponentDoc(slug: string, now = Date.now()) {
	const { enabled, activeDays } = docsUiConfig.sidebar.newBadge;
	if (!enabled || activeDays <= 0) {
		return false;
	}

	const metadata = newMetadataBySlug.get(slug);
	if (!metadata) {
		return false;
	}

	const newUntilTimestamp = parseDate(metadata.newUntil);
	if (newUntilTimestamp !== null) {
		return now <= newUntilTimestamp;
	}

	const introducedAtTimestamp = parseDate(metadata.introducedAt);
	if (introducedAtTimestamp === null) {
		return false;
	}

	const age = now - introducedAtTimestamp;
	return age >= 0 && age <= activeDays * MS_PER_DAY;
}
