import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, it, expect, vi } from "vitest";
import { renderWithProviders } from "@/test/render";
import { NewGame } from "./NewGame";

describe("NewGame", () => {
  const defaults = { onBack: vi.fn(), onStart: vi.fn() };

  it("renders the Manager Creation heading", () => {
    renderWithProviders(<NewGame {...defaults} />);
    expect(
      screen.getByRole("heading", { name: /manager creation/i })
    ).toBeInTheDocument();
  });

  it("renders form fields for first name, last name, nickname, and nationality", () => {
    renderWithProviders(<NewGame {...defaults} />);
    expect(screen.getByLabelText(/first name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/last name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/nickname/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/nationality/i)).toBeInTheDocument();
  });

  it("renders esport type selector with MOBA option", () => {
    renderWithProviders(<NewGame {...defaults} />);
    expect(screen.getByLabelText(/esport type/i)).toBeInTheDocument();
    expect(screen.getByText("MOBA")).toBeInTheDocument();
  });

  it("disables Continue button when required fields are empty", () => {
    renderWithProviders(<NewGame {...defaults} />);
    const btn = screen.getByRole("button", { name: /continue/i });
    expect(btn).toBeDisabled();
  });

  it("enables Continue button when all required fields are filled", async () => {
    const user = userEvent.setup();
    renderWithProviders(<NewGame {...defaults} />);

    await user.type(screen.getByLabelText(/first name/i), "Kim");
    await user.type(screen.getByLabelText(/last name/i), "Jeong-gyun");
    await user.type(screen.getByLabelText(/nickname/i), "kkOma");
    await user.type(screen.getByLabelText(/nationality/i), "KR");

    const btn = screen.getByRole("button", { name: /continue/i });
    expect(btn).toBeEnabled();
  });

  it("calls onStart when Continue is clicked with valid form", async () => {
    const user = userEvent.setup();
    const onStart = vi.fn();
    renderWithProviders(<NewGame onBack={vi.fn()} onStart={onStart} />);

    await user.type(screen.getByLabelText(/first name/i), "Kim");
    await user.type(screen.getByLabelText(/last name/i), "Jeong-gyun");
    await user.type(screen.getByLabelText(/nickname/i), "kkOma");
    await user.type(screen.getByLabelText(/nationality/i), "KR");

    await user.click(screen.getByRole("button", { name: /continue/i }));
    expect(onStart).toHaveBeenCalledTimes(1);
  });

  it("calls onBack when Back button is clicked", async () => {
    const user = userEvent.setup();
    const onBack = vi.fn();
    renderWithProviders(<NewGame onBack={onBack} onStart={vi.fn()} />);

    await user.click(screen.getByRole("button", { name: /back/i }));
    expect(onBack).toHaveBeenCalledTimes(1);
  });
});
