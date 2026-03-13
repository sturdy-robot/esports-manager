import { DollarSign, TrendingUp, TrendingDown, Receipt } from "lucide-react";

export interface Transaction {
  id: string;
  description: string;
  amount: number;
  date: string;
  category: string;
}

interface FinancesProps {
  balance: number;
  income: number;
  expenses: number;
  transactions: Transaction[];
}

function formatCurrency(value: number): string {
  return `$${Math.abs(value).toLocaleString("en-US")}`;
}

function SummaryCard({
  icon,
  label,
  value,
  color,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  color: string;
}) {
  return (
    <div
      className="app-panel flex flex-col gap-3 p-5 rounded-2xl glow-hover transition-all"
      style={{
        backgroundColor: "var(--bg-surface)",
      }}
    >
      <div className="flex items-center justify-between">
        <span className="app-eyebrow" style={{ color: "var(--text-muted)" }}>
          {label}
        </span>
        <span style={{ color }}>{icon}</span>
      </div>
      <span
        className="text-[28px] leading-none font-bold tabular-nums tracking-tight font-display"
        style={{ color: "var(--text-primary)" }}
      >
        {value}
      </span>
    </div>
  );
}

export function Finances({
  balance,
  income,
  expenses,
  transactions,
}: FinancesProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">Club Operations</span>
        <h2 className="app-page-title">Finances</h2>
        <p className="app-page-subtitle">
          Track runway, review income and expenses, and monitor how each
          transaction shapes season flexibility.
        </p>
      </div>

      {/* Summary cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <SummaryCard
          icon={<DollarSign size={18} />}
          label="Balance"
          value={formatCurrency(balance)}
          color="var(--color-accent-cyan)"
        />
        <SummaryCard
          icon={<TrendingUp size={18} />}
          label="Income"
          value={formatCurrency(income)}
          color="var(--color-win)"
        />
        <SummaryCard
          icon={<TrendingDown size={18} />}
          label="Expenses"
          value={formatCurrency(expenses)}
          color="var(--color-loss)"
        />
      </div>

      {/* Transactions */}
      {transactions.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <Receipt size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No transactions yet</p>
        </div>
      )}

      {transactions.length > 0 && (
        <div
          className="app-panel rounded-2xl overflow-hidden"
          style={{
            backgroundColor: "var(--bg-surface)",
          }}
        >
          <table className="w-full">
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  backgroundColor: "rgba(255, 255, 255, 0.03)",
                }}
              >
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Date
                </th>
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Description
                </th>
                <th
                  className="text-left text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Category
                </th>
                <th
                  className="text-right text-xs font-semibold uppercase tracking-wider py-2.5 px-3"
                  style={{ color: "var(--text-muted)" }}
                >
                  Amount
                </th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx) => {
                const isPositive = tx.amount >= 0;
                const amountStr = isPositive
                  ? `+${formatCurrency(tx.amount)}`
                  : `-${formatCurrency(tx.amount)}`;
                const amountColor = isPositive
                  ? "var(--color-win)"
                  : "var(--color-loss)";

                return (
                  <tr
                    key={tx.id}
                    className="transition-colors"
                    style={{
                      borderBottom: "1px solid var(--border-subtle)",
                    }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.backgroundColor =
                        "var(--bg-elevated)";
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.backgroundColor = "transparent";
                    }}
                  >
                    <td
                      className="py-3 px-3 text-xs tabular-nums"
                      style={{ color: "var(--text-muted)" }}
                    >
                      {tx.date}
                    </td>
                    <td
                      className="py-3 px-3 text-sm"
                      style={{ color: "var(--text-primary)" }}
                    >
                      {tx.description}
                    </td>
                    <td className="py-3 px-3">
                      <span
                        className="px-2 py-0.5 rounded-full text-xs font-semibold"
                        style={{
                          backgroundColor: "rgba(6, 182, 212, 0.1)",
                          color: "var(--color-accent-cyan)",
                        }}
                      >
                        {tx.category}
                      </span>
                    </td>
                    <td
                      className="py-3 px-3 text-sm tabular-nums text-right font-semibold"
                      style={{ color: amountColor }}
                    >
                      {amountStr}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
