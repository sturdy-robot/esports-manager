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
      className="flex flex-col gap-2 p-4 rounded-lg border glow-hover transition-all"
      style={{
        backgroundColor: "var(--bg-surface)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="flex items-center justify-between">
        <span
          className="text-xs font-semibold uppercase tracking-wider"
          style={{ color: "var(--text-muted)" }}
        >
          {label}
        </span>
        <span style={{ color }}>{icon}</span>
      </div>
      <span
        className="text-2xl font-bold font-mono tracking-tight"
        style={{ color: "var(--text-primary)" }}
      >
        {value}
      </span>
    </div>
  );
}

export function Finances({ balance, income, expenses, transactions }: FinancesProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      <h2
        className="text-lg font-bold"
        style={{ color: "var(--text-primary)" }}
      >
        Finances
      </h2>

      {/* Summary cards */}
      <div className="grid grid-cols-3 gap-4">
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
        <div className="flex flex-col items-center justify-center gap-3 py-16">
          <Receipt size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No transactions yet</p>
        </div>
      )}

      {transactions.length > 0 && (
        <div
          className="rounded-lg border overflow-hidden"
          style={{
            backgroundColor: "var(--bg-surface)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <table className="w-full">
            <thead>
              <tr
                style={{
                  borderBottom: "1px solid var(--border-subtle)",
                  backgroundColor: "var(--bg-elevated)",
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
                      className="py-3 px-3 text-xs font-mono"
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
                        className="px-2 py-0.5 rounded text-xs font-semibold"
                        style={{
                          backgroundColor: "rgba(6, 182, 212, 0.1)",
                          color: "var(--color-accent-cyan)",
                        }}
                      >
                        {tx.category}
                      </span>
                    </td>
                    <td
                      className="py-3 px-3 text-sm font-mono tabular-nums text-right font-semibold"
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
