import { MS_PER_HOUR, MS_PER_MINUTE, MS_PER_SECOND } from "../constants";
import type { IntervalState } from "../types";

export const clsx = (...args: ClassValue[]) => args.map(flattenClass).filter(Boolean).join(" ");
export const cn = (...inputs: ClassValue[]) => clsx(inputs);

export function partsToMs(p: IntervalState): number {
  return p.hours * MS_PER_HOUR + p.minutes * MS_PER_MINUTE + p.seconds * MS_PER_SECOND + p.milliseconds;
}

export function msToParts(ms: number): IntervalState {
	const total = Math.max(0, Math.floor(ms));
	const hours = Math.floor(total / MS_PER_HOUR);
	const hoursRemainder = total % MS_PER_HOUR;
	const minutes = Math.floor(hoursRemainder / MS_PER_MINUTE);
	const minutesRemainder = hoursRemainder % MS_PER_MINUTE;
	const seconds = Math.floor(minutesRemainder / MS_PER_SECOND);
	const milliseconds = minutesRemainder % MS_PER_SECOND;

	return { hours, minutes, seconds, milliseconds };
}

export function safeParseInt(value: string, fallback = 0): number {
  const n = parseInt(value, 10);
  return Number.isFinite(n) ? n : fallback;
}

/* -------------------------------------------------- UTILITY --------------------------------------------------*/
export type ClassValue = ClassArray | ClassDictionary | string | number | bigint | null | boolean | undefined;
export type ClassDictionary = Record<string, unknown>;
export type ClassArray = ClassValue[];

export interface Variants {
	[key: string]: {
		[key: string]: string;
	};
}

export interface Options {
	variants: Variants;
	defaultVariants: {
		[key: string]: string;
	};
}

export function cva(base: string, options: Options) {
	return function (props: { [key: string]: string | undefined }) {
		const { variants, defaultVariants } = options;
		let classes = base;

		for (const variant in variants) {
			const value = props[variant] || defaultVariants[variant];
			if (value && variants[variant]![value]) {
				classes += ` ${variants[variant]![value]}`;
			}
		}

		if (props.className) {
			classes += ` ${props.className}`;
		}

		return classes;
	};
}

function flattenClass(mix: ClassValue): string | number {
	if (typeof mix === "string" || typeof mix === "number") {
		return mix;
	}

	if (Array.isArray(mix)) {
		return mix.map(flattenClass).filter(Boolean).join(" ");
	}

	if (typeof mix === "object" && mix !== null) {
		return Object.keys(mix)
			.filter(key => mix[key])
			.join(" ");
	}

	return "";
}
