export const HEATMAP_STORE_KEY_ANONYMOUS_ID = "heatmap.anonymous_id";
export const HEATMAP_STORE_KEY_USER_ID = "heatmap.user_id";
export function readIdentityValue(key) {
    try {
        return localStorage.getItem(key);
    }
    catch {
        return null;
    }
}
export function writeIdentityValue(key, value) {
    try {
        if (value === null) {
            localStorage.removeItem(key);
            return;
        }
        localStorage.setItem(key, value);
    }
    catch {
        // fail-closed: storage errors must not crash the host
    }
}
