import { cleanup, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import { DashboardApp } from "../src/DashboardApp";
import type {
  DashboardApiClient,
  ElementsResponse,
  HeatmapResponse,
  StillResponse,
} from "../src/api/types";

const sliceWithClicks: ElementsResponse = {
  unique_sessions: 2,
  elements: [
    {
      element_id: "checkout.pay",
      click_count: 4,
      unique_sessions: 2,
    },
  ],
};

const heatmapBins: HeatmapResponse = {
  bins: [{ bin_x: 50, bin_y: 50, count: 4 }],
};

const stillCheckout: StillResponse = {
  content_type: "image/png",
  image_base64: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==",
};

function createDashboardClient(
  options: {
    elements?: ElementsResponse | Error;
    heatmap?: HeatmapResponse | Error;
    still?: StillResponse | null | Error;
  } = {},
): DashboardApiClient {
  const elementsResult = options.elements ?? sliceWithClicks;
  const heatmapResult = options.heatmap ?? heatmapBins;
  const stillResult = options.still === undefined ? stillCheckout : options.still;

  return {
    getElements: vi.fn(() => {
      if (elementsResult instanceof Error) {
        return Promise.reject(elementsResult);
      }
      return Promise.resolve(elementsResult);
    }),
    getHeatmap: vi.fn(() => {
      if (heatmapResult instanceof Error) {
        return Promise.reject(heatmapResult);
      }
      return Promise.resolve(heatmapResult);
    }),
    getStill: vi.fn(() => {
      if (stillResult instanceof Error) {
        return Promise.reject(stillResult);
      }
      if (stillResult === null) {
        return Promise.resolve(null);
      }
      return Promise.resolve(stillResult);
    }),
  };
}

describe("heatmap overlay on optional stills", () => {
  it("should pass filter slice to elements, heatmap, and still queries", async () => {
    const client = createDashboardClient();
    const user = userEvent.setup();
    render(<DashboardApp client={client} autoLoad={false} />);

    await user.selectOptions(screen.getByRole("combobox", { name: /platform/i }), "ios");
    await user.clear(screen.getByRole("textbox", { name: /^screen$/i }));
    await user.type(screen.getByRole("textbox", { name: /^screen$/i }), "Checkout");
    await user.clear(screen.getByRole("textbox", { name: /layout version/i }));
    await user.type(screen.getByRole("textbox", { name: /layout version/i }), "2024-10");
    await user.clear(screen.getByRole("textbox", { name: /app version/i }));
    await user.type(screen.getByRole("textbox", { name: /app version/i }), "2.0.0");
    await user.click(screen.getByRole("button", { name: /load totals/i }));

    await waitFor(() => {
      expect(client.getElements).toHaveBeenCalledWith(
        expect.objectContaining({
          platform: "ios",
          screen: "Checkout",
          layoutVersion: "2024-10",
          appVersion: "2.0.0",
        }),
      );
    });
    expect(client.getHeatmap).toHaveBeenCalledWith(
      expect.objectContaining({
        platform: "ios",
        layoutVersion: "2024-10",
      }),
    );
    expect(client.getStill).toHaveBeenCalledWith(
      expect.objectContaining({
        platform: "ios",
        layoutVersion: "2024-10",
      }),
    );
  });

  it("should show counts without overlay image when still is missing", async () => {
    const client = createDashboardClient({ still: null });
    render(<DashboardApp client={client} autoLoad />);

    expect(
      await screen.findByRole("status", { name: /^unique sessions$/i }),
    ).toHaveTextContent("2");
    expect(screen.queryByTestId("heatmap-overlay")).not.toBeInTheDocument();
    expect(screen.queryByRole("img", { name: /reference still/i })).not.toBeInTheDocument();
  });

  it("should use the still as overlay background when present", async () => {
    const client = createDashboardClient();
    render(<DashboardApp client={client} autoLoad />);

    const overlay = await screen.findByTestId("heatmap-overlay");
    expect(overlay).toBeInTheDocument();
    const stillImage = screen.getByLabelText(/reference still/i);
    expect(stillImage.getAttribute("src")).toMatch(/^data:image\/png;base64,/);
    expect(within(overlay).getByLabelText(/reference still/i)).toBe(stillImage);
  });

  it("should request overlay bins for the selected layout version only", async () => {
    const user = userEvent.setup();
    const client = createDashboardClient();
    render(<DashboardApp client={client} autoLoad={false} />);

    await user.clear(screen.getByRole("textbox", { name: /layout version/i }));
    await user.type(screen.getByRole("textbox", { name: /layout version/i }), "layout-a");
    await user.click(screen.getByRole("button", { name: /load totals/i }));

    await screen.findByTestId("heatmap-overlay");
    expect(client.getHeatmap).toHaveBeenCalledWith(
      expect.objectContaining({ layoutVersion: "layout-a" }),
    );
    expect(client.getStill).toHaveBeenCalledWith(
      expect.objectContaining({ layoutVersion: "layout-a" }),
    );
  });

  it("should distinguish overlay loading, empty still, and overlay errors", async () => {
    const slowStill: DashboardApiClient = {
      getElements: vi.fn().mockResolvedValue(sliceWithClicks),
      getHeatmap: vi.fn().mockResolvedValue(heatmapBins),
      getStill: vi.fn(
        (): Promise<StillResponse> =>
          new Promise((resolve) => {
            setTimeout(() => resolve(stillCheckout), 80);
          }),
      ),
    };
    const { unmount: unmountSlow } = render(<DashboardApp client={slowStill} autoLoad />);
    expect(
      screen.getByRole("status", { name: /loading density overlay/i }),
    ).toBeInTheDocument();
    await screen.findByTestId("heatmap-overlay");
    expect(
      screen.queryByRole("status", { name: /loading density overlay/i }),
    ).not.toBeInTheDocument();
    unmountSlow();
    cleanup();

    const noStillClient = createDashboardClient({ still: null });
    render(<DashboardApp client={noStillClient} autoLoad />);
    await screen.findByRole("status", { name: /^unique sessions$/i });
    expect(screen.queryByTestId("heatmap-overlay")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("status", { name: /loading density overlay/i }),
    ).not.toBeInTheDocument();

    cleanup();
    const errorClient = createDashboardClient({
      heatmap: new Error("heatmap unavailable"),
    });
    render(<DashboardApp client={errorClient} autoLoad />);
    const overlayAlert = await screen.findByRole("alert", {
      name: /density overlay/i,
    });
    expect(overlayAlert).toHaveTextContent(/heatmap unavailable/i);
  });

  it("should keep overlay region keyboard reachable when a still exists", async () => {
    render(<DashboardApp client={createDashboardClient()} autoLoad />);

    const overlay = await screen.findByTestId("heatmap-overlay");
    expect(overlay).toHaveAttribute("tabindex", "0");
    overlay.focus();
    expect(overlay).toHaveFocus();
    expect(screen.getByRole("combobox", { name: /platform/i })).toBeInTheDocument();
  });
});
