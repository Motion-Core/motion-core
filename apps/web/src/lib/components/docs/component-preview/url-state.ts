import type {
	ComponentPreviewControl,
	ComponentPreviewValue,
	ComponentPreviewValues,
} from "./types";
import { getDefaultControlValue } from "./types";

const valuesEqual = (
	left: ComponentPreviewValue | undefined,
	right: ComponentPreviewValue | undefined,
) => String(left) === String(right);

const coerceNumber = (value: number, control: ComponentPreviewControl) => {
	if (control.type !== "number") return value;

	const min = control.min ?? Number.NEGATIVE_INFINITY;
	const max = control.max ?? Number.POSITIVE_INFINITY;
	return Math.min(max, Math.max(min, value));
};

const parseControlValue = (
	control: ComponentPreviewControl,
	rawValue: string | null,
): ComponentPreviewValue | undefined => {
	if (rawValue === null) return undefined;

	if (control.type === "boolean") {
		if (rawValue === "true") return true;
		if (rawValue === "false") return false;
		return undefined;
	}

	if (control.type === "number") {
		const value = Number(rawValue);
		return Number.isFinite(value) ? coerceNumber(value, control) : undefined;
	}

	if (control.type === "select") {
		const option = control.options.find(
			(candidate) => String(candidate.value) === rawValue,
		);

		return option?.value;
	}

	return rawValue;
};

export const readControlValuesFromSearch = (
	controls: ComponentPreviewControl[],
	search: string,
): ComponentPreviewValues => {
	const params = new URLSearchParams(search);
	const values: ComponentPreviewValues = {};

	for (const control of controls) {
		const parsed = parseControlValue(control, params.get(control.name));
		values[control.name] = parsed ?? getDefaultControlValue(control);
	}

	return values;
};

export const writeControlValuesToSearch = (
	controls: ComponentPreviewControl[],
	values: ComponentPreviewValues,
	search: string,
) => {
	const params = new URLSearchParams(search);

	for (const control of controls) {
		const value = values[control.name] ?? getDefaultControlValue(control);
		const defaultValue = getDefaultControlValue(control);

		if (valuesEqual(value, defaultValue)) {
			params.delete(control.name);
		} else {
			params.set(control.name, String(value));
		}
	}

	const nextSearch = params.toString();
	return nextSearch ? `?${nextSearch}` : "";
};
