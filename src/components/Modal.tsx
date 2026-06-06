import { useEffect, useState, type AnimationEvent, type ReactNode } from "react";

import { cn } from "../lib/util";

export interface ModalProps {
    open: boolean;
    onClose: () => void;
    onExited?: () => void;
    labelledBy?: string;
    className?: string;
    children: ReactNode;
}

export default function Modal({ open, onClose, onExited, labelledBy, className, children }: ModalProps) {
    const [render, setRender] = useState(open);

    useEffect(() => {
        if (open) setRender(true);
    }, [open]);

    if (!render) return null;

    const closing = !open;
    const handleAnimationEnd = (e: AnimationEvent<HTMLDivElement>) => {
        if (closing && e.target === e.currentTarget) {
            setRender(false);
            onExited?.();
        }
    };

    return (
        <div
            className={`fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm ${closing ? "overlay-out pointer-events-none" : "overlay-in"}`}
            onClick={onClose}
            onAnimationEnd={handleAnimationEnd}
            role="dialog"
            aria-modal="true"
            aria-labelledby={labelledBy}
        >
            <div
                className={cn(
                    "w-[88vw] max-w-sm rounded-2xl border border-(--border-subtle) bg-(--bg-window) p-4 shadow-2xl",
                    closing ? "dialog-out" : "dialog-in",
                    className,
                )}
                onClick={(e) => e.stopPropagation()}
            >
                {children}
            </div>
        </div>
    );
}
