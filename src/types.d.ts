export type MouseButtonKey = keyof typeof MouseButton;
export enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

export type ClickTypeKey = keyof typeof ClickType;
export enum ClickType {
    Single = 0,
    Double = 1,
}

export interface Settings {
    mouse_button: MouseButton;
    click_type: ClickType;
    interval: number;
    hotkey: string;
}

export interface IntervalState {
  hours: number;
  minutes: number;
  seconds: number;
  milliseconds: number;
}

export interface OptionsState {
    mouse_button: MouseButton;
    click_type: ClickType;
}
