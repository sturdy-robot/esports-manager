import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Settings } from "./Settings";

describe("Settings", () => {
  const defaults = { onBack: vi.fn() };

  it("renders the Settings heading", () => {
    renderWithProviders(<Settings {...defaults} />);
    expect(
      screen.getByRole("heading", { name: /^settings$/i, level: 1 }),
    ).toBeInTheDocument();
  });

  it("renders a saves folder path field", () => {
    renderWithProviders(<Settings {...defaults} />);
    expect(screen.getByLabelText(/saves folder/i)).toBeInTheDocument();
  });

  it("renders the appearance section", () => {
    renderWithProviders(<Settings {...defaults} />);
    expect(screen.getByText(/appearance/i)).toBeInTheDocument();
    expect(
      screen.getByText(/broadcaster command center \+ performance lab/i),
    ).toBeInTheDocument();
  });

  it("calls onBack when Back is clicked", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();
    renderWithProviders(<Settings onBack={onBack} />);

    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("saves folder path is editable", async () => {
    const user = userEvent.setup();
    renderWithProviders(<Settings {...defaults} />);

    const input = screen.getByLabelText(/saves folder/i) as HTMLInputElement;
    await user.clear(input);
    await user.type(input, "/tmp/my-saves");
    expect(input.value).toBe("/tmp/my-saves");
  });
});
