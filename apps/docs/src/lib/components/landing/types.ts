export type SocialLink = { label: string; href: string };
export type ComponentInfo = {
	name: string;
	video?: string;
	poster?: string;
	slug: string;
	category?: string;
	dependencies?: Record<string, string>;
};
