export function createTrackingGate(initialEnabled = true) {
    let enabled = initialEnabled;
    return {
        isTrackingEnabled: () => enabled,
        setTrackingEnabled: (value) => {
            enabled = value;
        },
    };
}
