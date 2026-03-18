export { default as DocNavigation } from "./components/docs/navigation/DocNavigation.svelte";
export { default as TableOfContents } from "./components/docs/TableOfContents.svelte";
export { default as DocsSidebar } from "./components/docs/navigation/DocsSidebar.svelte";
export { default as MobileSidebar } from "./components/docs/navigation/MobileSidebar.svelte";
export { default as DocShareActions } from "./components/docs/DocShareActions.svelte";
export { default as MobileDocShareActions } from "./components/docs/MobileDocShareActions.svelte";
export { default as CommandPalette } from "./components/docs/search/CommandPalette.svelte";
export { default as ScrollArea } from "./components/ui/ScrollArea.svelte";
export { default as InstallationTabs } from "./components/docs/InstallationTabs.svelte";
export { default as Step } from "./components/docs/markdown/Step.svelte";
export { default as Steps } from "./components/docs/markdown/Steps.svelte";

export { brandingConfig } from "./config/branding";
export { siteConfig, type SiteConfig } from "./config/site";
export {
	docsUiConfig,
	resolveDocAssistantUrls,
	resolveRepositoryDocUrl,
	resolveTocSelector,
	availablePackageManagers,
	type DocsUiConfig,
	type PackageManagerOption,
} from "./config/docs-ui";

export {
	docsManifest,
	getAdjacentDocs,
	getDocBySlug,
	getDocHref,
} from "./docs/manifest";
export { getDocMetadata, type DocMetadata } from "./docs/metadata";

export { default as fkGroteskNeueRegularDataUri } from "$lib/assets/fonts/fkgroteskneue-satori-400.ttf?inline";
export { default as fkGroteskNeueSemiBoldDataUri } from "$lib/assets/fonts/fkgroteskneue-satori-600.ttf?inline";
export { default as brandLogoRaw } from "./assets/motion-core-logo.svg?raw";
