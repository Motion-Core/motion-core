/**
 * Canonical site-level metadata shared across SEO tags, manifests, and feeds.
 * Keep this object project-specific when using the docs template for a new brand.
 */
export const siteConfig = {
	/** Primary site name used in titles and Open Graph site fields. */
	name: "Motion Core",
	/** Compact site name for environments with strict length limits. */
	shortName: "Motion Core",
	/** Public canonical URL used to build absolute links. */
	url: "https://motion-core.dev",
	/** Default SEO description for the homepage and fallback metadata. */
	description:
		"Svelte-native motion toolkit with GSAP and Three.js powered components, demos, and CLI-driven workflows.",
	/** Author shown in metadata and structured data. */
	author: "Marek Jóźwiak",
	/** Primary SEO keywords for indexing and discovery. */
	keywords: [
		"motion core",
		"svelte animation",
		"gsap",
		"three.js",
		"component library",
		"svelte components",
		"motion toolkit",
		"cli",
		"registry",
	],
	/** Default social preview image path. */
	ogImage: "/og-image.jpg",
	/** External profile links used by docs actions and metadata. */
	links: {
		github: "https://github.com/motion-core/motion-core",
		twitter: "https://github.com/motion-core/motion-core",
	},
	/** Package metadata used in installation snippets and docs helpers. */
	package: {
		name: "motion-core",
	},
};

/** Inferred type for strongly-typed consumers of `siteConfig`. */
export type SiteConfig = typeof siteConfig;
