import { cn } from "../lib/util";

export interface SegmentedOption<T> {
    label: string;
    value: T;
}

export interface SegmentedProps<T extends string | number> {
    value: T;
    options: SegmentedOption<T>[];
    onChange: (value: T) => void;
    disabled?: boolean;
    "aria-label"?: string;
}

export default function Segmented<T extends string | number>({
    value,
    options,
    onChange,
    disabled,
    "aria-label": ariaLabel,
}: SegmentedProps<T>) {
    return (
        <div className="seg" role="tablist" aria-label={ariaLabel} aria-disabled={disabled}>
            {options.map((option) => {
                const selected = option.value === value;
                return (
                    <button
                        key={String(option.value)}
                        type="button"
                        role="tab"
                        aria-selected={selected}
                        disabled={disabled}
                        className={cn("seg-item")}
                        onClick={() => onChange(option.value)}
                    >
                        {option.label}
                    </button>
                );
            })}
        </div>
    );
}
