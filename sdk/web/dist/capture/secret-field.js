const SECRET_AUTOCOMPLETE = new Set([
    "password",
    "new-password",
    "cc-number",
    "cc-exp",
    "cc-csc",
]);
export function isSecretInteractionTarget(target) {
    const field = resolveInputOrTextarea(target);
    if (!field) {
        return false;
    }
    return isSecretInputOrTextarea(field);
}
function resolveInputOrTextarea(target) {
    if (!(target instanceof Element)) {
        return null;
    }
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
        return target;
    }
    const ancestor = target.closest("input, textarea");
    if (ancestor instanceof HTMLInputElement || ancestor instanceof HTMLTextAreaElement) {
        return ancestor;
    }
    return null;
}
function isSecretInputOrTextarea(field) {
    if (field instanceof HTMLInputElement && field.type === "password") {
        return true;
    }
    const autocomplete = field.getAttribute("autocomplete");
    if (!autocomplete) {
        return false;
    }
    return SECRET_AUTOCOMPLETE.has(autocomplete.trim().toLowerCase());
}
