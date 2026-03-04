import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { LoadGame } from "./LoadGame";
import type { SaveEntry } from "./LoadGame";

describe("LoadGame", () => {
  const defaults = {
    onBack: vi.fn(),
    onLoad: vi.fn(),
    saves: [] as SaveEntry[],
    loading: false,
  };

  it("renders the Load Game heading", () => {
    renderWithProviders(<LoadGame {...defaults} />);
    expect(
      screen.getByRole("heading", { name: /load game/i })
    ).toBeInTheDocument();
  });

  it("shows empty state when no saves exist", () => {
    renderWithProviders(<LoadGame {...defaults} />);
    expect(screen.getByText(/no saved games/i)).toBeInTheDocument();
  });

  it("renders save entries when saves are provided", () => {
    const saves: SaveEntry[] = [
      { name: "save_alpha", checksum: "abc123" },
      { name: "save_bravo", checksum: "def456" },
    ];
    renderWithProviders(<LoadGame {...defaults} saves={saves} />);
    expect(screen.getByText("save_alpha")).toBeInTheDocument();
    expect(screen.getByText("save_bravo")).toBeInTheDocument();
  });

  it("shows checksum badge for each save", () => {
    const saves: SaveEntry[] = [
      { name: "my_save", checksum: "abc123def456" },
    ];
    renderWithProviders(<LoadGame {...defaults} saves={saves} />);
    expect(screen.getByText(/abc123/)).toBeInTheDocument();
  });

  it("calls onLoad with the save name when Load button is clicked", async () => {
    const user = userEvent.setup();
    const onLoad = vi.fn();
    const saves: SaveEntry[] = [
      { name: "career_1", checksum: "aaa111" },
    ];
    renderWithProviders(
      <LoadGame {...defaults} saves={saves} onLoad={onLoad} />
    );

    await user.click(screen.getByRole("button", { name: /load/i }));
    expect(onLoad).toHaveBeenCalledWith("career_1");
  });

  it("calls onBack when Back button is clicked", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();
    renderWithProviders(<LoadGame {...defaults} onBack={onBack} />);

    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("shows loading indicator when loading is true", () => {
    renderWithProviders(<LoadGame {...defaults} loading={true} />);
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });
});
