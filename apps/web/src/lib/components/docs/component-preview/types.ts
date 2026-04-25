import type { Snippet } from "svelte";

export type ComponentPreviewValue = string | number | boolean;

export type SourceTab = {
	name: string;
	code: string;
	language?: string;
};

export type BasePreviewControl = {
	name: string;
	label: string;
	description?: string;
	defaultValue?: ComponentPreviewValue;
};

export type BooleanPreviewControl = BasePreviewControl & {
	type: "boolean";
};

export type NumberPreviewControl = BasePreviewControl & {
	type: "number";
	min?: number;
	max?: number;
	step?: number;
	unit?: string;
};

export type TextPreviewControl = BasePreviewControl & {
	type: "text" | "color";
	placeholder?: string;
};

export type SelectPreviewControl = BasePreviewControl & {
	type: "select";
	options: Array<{
		label: string;
		value: string | number;
	}>;
};

export type ComponentPreviewControl =
	| BooleanPreviewControl
	| NumberPreviewControl
	| TextPreviewControl
	| SelectPreviewControl;

export type ComponentPreviewValues = Record<string, ComponentPreviewValue>;

export type ComponentPreviewChildren = Snippet<[ComponentPreviewValues]>;

export const getDefaultControlValue = (
	control: ComponentPreviewControl,
): ComponentPreviewValue => {
	if (control.defaultValue !== undefined) {
		return control.defaultValue;
	}

	if (control.type === "boolean") {
		return false;
	}

	if (control.type === "number") {
		return control.min ?? 0;
	}

	if (control.type === "select") {
		return control.options[0]?.value ?? "";
	}

	if (control.type === "color") {
		return "#ff6900";
	}

	return "";
};
