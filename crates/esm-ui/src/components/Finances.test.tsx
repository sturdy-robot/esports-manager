import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Finances } from "./Finances";
import type { Transaction } from "./Finances";

const MOCK_TRANSACTIONS: Transaction[] = [
  { id: "t1", description: "Player salary — Faker", amount: -45000, date: "Jan 1", category: "Salary" },
  { id: "t2", description: "Sponsor payment — TechCorp", amount: 50000, date: "Jan 1", category: "Sponsor" },
  { id: "t3", description: "Scrim facility rental", amount: -8000, date: "Jan 2", category: "Operations" },
  { id: "t4", description: "Prize money — LCK Week 1", amount: 25000, date: "Jan 3", category: "Prize" },
];

describe("Finances", () => {
  const defaults = {
    balance: 1200000,
    income: 75000,
    expenses: 53000,
    transactions: MOCK_TRANSACTIONS,
  };

  it("renders the Finances heading", () => {
    renderWithProviders(<Finances {...defaults} />);
    expect(
      screen.getByRole("heading", { name: /finances/i })
    ).toBeInTheDocument();
  });

  it("shows balance amount", () => {
    renderWithProviders(<Finances {...defaults} />);
    expect(screen.getByText("$1,200,000")).toBeInTheDocument();
  });

  it("shows income and expenses summary", () => {
    renderWithProviders(<Finances {...defaults} />);
    expect(screen.getByText("$75,000")).toBeInTheDocument();
    expect(screen.getByText("$53,000")).toBeInTheDocument();
  });

  it("renders transaction rows", () => {
    renderWithProviders(<Finances {...defaults} />);
    expect(screen.getByText("Player salary — Faker")).toBeInTheDocument();
    expect(screen.getByText("Sponsor payment — TechCorp")).toBeInTheDocument();
  });

  it("shows positive amounts in green and negative in red", () => {
    renderWithProviders(<Finances {...defaults} />);
    const positive = screen.getByText("+$50,000");
    const negative = screen.getByText("-$45,000");
    expect(positive).toBeInTheDocument();
    expect(negative).toBeInTheDocument();
  });

  it("shows transaction categories", () => {
    renderWithProviders(<Finances {...defaults} />);
    expect(screen.getByText("Salary")).toBeInTheDocument();
    expect(screen.getByText("Sponsor")).toBeInTheDocument();
  });

  it("renders empty state when no transactions", () => {
    renderWithProviders(<Finances balance={0} income={0} expenses={0} transactions={[]} />);
    expect(screen.getByText(/no transactions/i)).toBeInTheDocument();
  });
});
