import { screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { renderWithProviders } from "@/test/render";
import { Inbox } from "./Inbox";
import type { InboxMessage } from "./Inbox";

const MOCK_MESSAGES: InboxMessage[] = [
  {
    id: "1",
    subject: "Contract Expiring",
    category: "Contract",
    priority: "Urgent",
    day: 15,
    read: false,
  },
  {
    id: "2",
    subject: "Scrim Result vs Gen.G",
    category: "Match",
    priority: "Info",
    day: 14,
    read: true,
  },
  {
    id: "3",
    subject: "Transfer Offer for Zeus",
    category: "Transfer",
    priority: "Action",
    day: 14,
    read: false,
  },
];

describe("Inbox", () => {
  it("renders the Inbox heading", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    expect(
      screen.getByRole("heading", { name: /inbox/i })
    ).toBeInTheDocument();
  });

  it("renders a row for each message", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    expect(screen.getByText("Contract Expiring")).toBeInTheDocument();
    expect(screen.getByText("Scrim Result vs Gen.G")).toBeInTheDocument();
    expect(screen.getByText("Transfer Offer for Zeus")).toBeInTheDocument();
  });

  it("shows priority badges", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    expect(screen.getByText("Urgent")).toBeInTheDocument();
    expect(screen.getByText("Info")).toBeInTheDocument();
    expect(screen.getByText("Action")).toBeInTheDocument();
  });

  it("shows category labels", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    expect(screen.getByText("Contract")).toBeInTheDocument();
    expect(screen.getByText("Match")).toBeInTheDocument();
    expect(screen.getByText("Transfer")).toBeInTheDocument();
  });

  it("visually distinguishes unread messages", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    const unreadSubjects = screen
      .getAllByTestId("message-row")
      .filter((row) => row.getAttribute("data-unread") === "true");
    expect(unreadSubjects.length).toBe(2);
  });

  it("shows blocking indicator for urgent messages", () => {
    renderWithProviders(<Inbox messages={MOCK_MESSAGES} />);
    expect(screen.getByTitle(/blocks continue/i)).toBeInTheDocument();
  });

  it("renders empty state when no messages", () => {
    renderWithProviders(<Inbox messages={[]} />);
    expect(screen.getByText(/no messages/i)).toBeInTheDocument();
  });
});
