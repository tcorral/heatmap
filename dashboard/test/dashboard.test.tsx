import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import { DashboardApp } from "../src/DashboardApp";
import type { DashboardApiClient, ElementsResponse } from "../src/api/types";

const sliceWithClicks: ElementsResponse = {
  unique_sessions: 3,
  elements: [
    {
      element_id: "checkout.pay",
      click_count: 12,
      unique_sessions: 3,
    },
    {
      element_id: "checkout.coupon",
      click_count: 5,
      unique_sessions: 2,
    },
  ],
};

function createClient(
  result: ElementsResponse | Error,
): DashboardApiClient {
  if (result instanceof Error) {
    return {
      getElements: vi.fn().mockRejectedValue(result),
      getHeatmap: vi.fn().mockResolvedValue({ bins: [] }),
      getStill: vi.fn().mockResolvedValue(null),
    };
  }
  return {
    getElements: vi.fn().mockResolvedValue(result),
    getHeatmap: vi.fn().mockResolvedValue({ bins: [] }),
    getStill: vi.fn().mockResolvedValue(null),
  };
}

describe("heatmap dashboard element totals", () => {
  it("should list checkout pay counts for all five platforms in one result", async () => {
    const client = createClient({
      unique_sessions: 5,
      elements: [
        { platform: "web", element_id: "checkout.pay", click_count: 1, unique_sessions: 1 },
        { platform: "ios", element_id: "checkout.pay", click_count: 2, unique_sessions: 1 },
        { platform: "android", element_id: "checkout.pay", click_count: 3, unique_sessions: 1 },
        { platform: "macos", element_id: "checkout.pay", click_count: 4, unique_sessions: 1 },
        { platform: "windows", element_id: "checkout.pay", click_count: 5, unique_sessions: 1 },
      ],
    });
    render(<DashboardApp client={client} autoLoad />);

    const comparison = await screen.findByRole("table", {
      name: /checkout pay by platform/i,
    });
    expect(within(comparison).getByRole("row", { name: /web/i })).toHaveTextContent("1");
    expect(within(comparison).getByRole("row", { name: /iOS/ })).toHaveTextContent("2");
    expect(within(comparison).getByRole("row", { name: /Android/ })).toHaveTextContent("3");
    expect(within(comparison).getByRole("row", { name: /macOS/ })).toHaveTextContent("4");
    expect(within(comparison).getByRole("row", { name: /Windows/ })).toHaveTextContent("5");
    expect(screen.queryByRole("button", { name: /upload still/i })).not.toBeInTheDocument();
  });
  it("should show totals, unique sessions, and top elements when the slice has clicks", async () => {
    const client = createClient(sliceWithClicks);
    render(<DashboardApp client={client} autoLoad />);

    expect(
      await screen.findByRole("status", { name: /^unique sessions$/i }),
    ).toHaveTextContent("3");

    const table = screen.getByRole("table", { name: /element totals/i });
    expect(within(table).getByRole("cell", { name: "12" })).toBeInTheDocument();
    expect(screen.getByText("Checkout → Pay")).toBeInTheDocument();

    const topList = screen.getByRole("list", { name: /top elements/i });
    expect(within(topList).getByText(/checkout\.pay/)).toBeInTheDocument();
  });

  it("should show zero counts and no overlay when the slice is empty", async () => {
    render(
      <DashboardApp
        client={createClient({ unique_sessions: 0, elements: [] })}
        autoLoad
      />,
    );

    expect(
      await screen.findByRole("status", { name: /^unique sessions$/i }),
    ).toHaveTextContent("0");
    expect(screen.getByRole("table", { name: /element totals/i })).toBeInTheDocument();
    expect(screen.queryByTestId("heatmap-overlay")).not.toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });

  it("should show a visible query error without crashing when the query fails", async () => {
    render(
      <DashboardApp client={createClient(new Error("network down"))} autoLoad />,
    );

    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(/network down/i);
    expect(screen.getByRole("heading", { name: /heatmap dashboard/i })).toBeInTheDocument();
  });

  it("should expose keyboard-reachable filters and counts with visible accessible names", async () => {
    const user = userEvent.setup();
    render(<DashboardApp client={createClient(sliceWithClicks)} autoLoad={false} />);

    await user.tab();
    expect(screen.getByRole("combobox", { name: /platform/i })).toHaveFocus();

    await user.tab();
    expect(screen.getByRole("textbox", { name: /^screen$/i })).toHaveFocus();

    await user.tab();
    expect(screen.getByRole("textbox", { name: /layout version/i })).toHaveFocus();

    await user.tab();
    expect(screen.getByRole("textbox", { name: /app version/i })).toHaveFocus();

    await user.click(screen.getByRole("button", { name: /load totals/i }));

    await waitFor(() => {
      expect(
        screen.getByRole("status", { name: /^unique sessions$/i }),
      ).toBeInTheDocument();
    });
  });

  it("should not show a login wall", () => {
    render(<DashboardApp client={createClient(sliceWithClicks)} autoLoad={false} />);

    expect(screen.queryByLabelText(/password/i)).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /sign in/i })).not.toBeInTheDocument();
    expect(screen.getByRole("heading", { name: /heatmap dashboard/i })).toBeInTheDocument();
  });

  it("should show checkout totals without scroll-depth or scroll-map UI", async () => {
    const client = createClient(sliceWithClicks);
    render(<DashboardApp client={client} autoLoad />);

    expect(
      await screen.findByRole("status", { name: /^unique sessions$/i }),
    ).toHaveTextContent("3");
    expect(screen.getByRole("table", { name: /element totals/i })).toBeInTheDocument();
    expect(screen.getByRole("list", { name: /top elements/i })).toBeInTheDocument();

    const main = screen.getByRole("main");
    const copy = main.textContent ?? "";
    expect(copy).not.toMatch(/scroll[-\s]?depth/i);
    expect(copy).not.toMatch(/scroll[-\s]?map/i);
    expect(screen.queryByTestId("scroll-depth-map")).not.toBeInTheDocument();
    expect(screen.queryByTestId("scroll-map")).not.toBeInTheDocument();
  });

  it("should distinguish loading from empty results", async () => {
    const client: DashboardApiClient = {
      getElements: vi.fn((): Promise<ElementsResponse> => {
        return new Promise((resolve) => {
          setTimeout(() => resolve({ unique_sessions: 0, elements: [] }), 50);
        });
      }),
      getHeatmap: vi.fn().mockResolvedValue({ bins: [] }),
      getStill: vi.fn().mockResolvedValue(null),
    };

    render(<DashboardApp client={client} autoLoad />);

    expect(screen.getByRole("status", { name: /loading element totals/i })).toBeInTheDocument();

    await waitFor(() => {
      expect(
        screen.queryByRole("status", { name: /loading element totals/i }),
      ).not.toBeInTheDocument();
    });
    const uniqueSessions = await screen.findByRole("status", {
      name: /^unique sessions$/i,
    });
    expect(uniqueSessions).toHaveTextContent("0");
  });
});
