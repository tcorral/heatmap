export type TrackingGate = {
  isTrackingEnabled(): boolean;
  setTrackingEnabled(enabled: boolean): void;
};

export function createTrackingGate(initialEnabled = true): TrackingGate {
  let enabled = initialEnabled;
  return {
    isTrackingEnabled: () => enabled,
    setTrackingEnabled: (value: boolean) => {
      enabled = value;
    },
  };
}
