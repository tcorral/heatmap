import { wireDomClicksToWasm } from "./wasm-click-bridge.js";
import { wireDomScrollToWasm } from "./wasm-scroll-bridge.js";
export function wireDomCaptureToWasm(recorder, options) {
    const stopClicks = wireDomClicksToWasm(recorder, options);
    const stopScroll = wireDomScrollToWasm(recorder, options);
    return () => {
        stopClicks();
        stopScroll();
    };
}
