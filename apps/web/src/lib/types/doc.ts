export type DocItem = {
	slug: string;
	name: string;
	category?: string;
	items?: DocItem[];
};
