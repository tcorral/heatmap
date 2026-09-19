export type WindowPosition = {
    x: number;
    y: number;
};
export type WindowViewport = {
    width: number;
    height: number;
};
export declare function readHeatmapElementId(target: EventTarget | null): string | undefined;
export declare function readVisibleWindowViewport(): WindowViewport | null;
export declare function normalizeWindowPosition(clientX: number, clientY: number, viewport?: WindowViewport): WindowPosition | null;
//# sourceMappingURL=window-click.d.ts.map