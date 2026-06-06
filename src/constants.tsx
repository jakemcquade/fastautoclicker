// Jake McQuade //
//  01-06-2026 //

import type { IntervalState, OptionsState, RepeatState } from "./types";

export const MS_PER_HOUR = 3_600_000;
export const MS_PER_MINUTE = 60_000;
export const MS_PER_SECOND = 1_000;
export const UPDATE_CHECK_INTERVAL_MS = 10 * 60 * 1000;

export const DEFAULT_INTERVAL: IntervalState = { hours: 0, minutes: 0, seconds: 0, milliseconds: 100 };
export const DEFAULT_OPTIONS: OptionsState = { mouse_button: 0, click_type: 0 };
export const DEFAULT_REPEAT: RepeatState = { mode: "until_stopped", count: 10 };
