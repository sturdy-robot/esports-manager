import { Mail, ShieldAlert } from "lucide-react";

export interface InboxMessage {
  id: string;
  subject: string;
  category: string;
  priority: "Urgent" | "Action" | "Info";
  day: number;
  read: boolean;
}

interface InboxProps {
  messages: InboxMessage[];
  onResolveMessage?: (id: string, subject: string) => void;
  resolvingMsgId?: string | null;
}

const PRIORITY_STYLES: Record<string, { bg: string; color: string }> = {
  Urgent: { bg: "rgba(239, 68, 68, 0.15)", color: "var(--color-loss)" },
  Action: { bg: "rgba(251, 191, 36, 0.15)", color: "var(--color-warning)" },
  Info: { bg: "rgba(96, 165, 250, 0.15)", color: "var(--color-info)" },
};

export function Inbox({
  messages,
  onResolveMessage,
  resolvingMsgId,
}: InboxProps) {
  return (
    <div className="flex flex-col gap-4 animate-fade-in-up">
      <div className="flex flex-col gap-1.5">
        <span className="app-eyebrow">Comms</span>
        <h2 className="app-page-title">Inbox</h2>
        <p className="app-page-subtitle">
          Resolve urgent decisions, review team updates, and keep operations
          moving.
        </p>
      </div>

      {messages.length === 0 && (
        <div className="app-panel rounded-2xl flex flex-col items-center justify-center gap-3 py-16">
          <Mail size={48} style={{ color: "var(--text-muted)" }} />
          <p style={{ color: "var(--text-muted)" }}>No messages</p>
        </div>
      )}

      {messages.length > 0 && (
        <div className="flex flex-col gap-2">
          {messages.map((msg) => {
            const pStyle =
              PRIORITY_STYLES[msg.priority] || PRIORITY_STYLES.Info;
            return (
              <div
                key={msg.id}
                data-testid="message-row"
                data-unread={!msg.read}
                className="app-panel flex items-center gap-4 p-4 rounded-2xl transition-all glow-hover"
                style={{
                  backgroundColor: msg.read
                    ? "var(--bg-surface)"
                    : "var(--bg-elevated)",
                  borderColor: msg.read
                    ? "var(--border-subtle)"
                    : "var(--color-accent-cyan)",
                  borderLeftWidth: msg.read ? "1px" : "3px",
                }}
              >
                {/* Unread dot */}
                {!msg.read && (
                  <span
                    className="w-2 h-2 rounded-full shrink-0"
                    style={{ backgroundColor: "var(--color-accent-cyan)" }}
                  />
                )}
                {msg.read && <span className="w-2 shrink-0" />}

                {/* Subject + category */}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span
                      className="text-sm truncate"
                      style={{
                        color: "var(--text-primary)",
                        fontWeight: msg.read ? 400 : 600,
                      }}
                    >
                      {msg.subject}
                    </span>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs">
                    <span style={{ color: "var(--text-secondary)" }}>
                      {msg.category}
                    </span>
                    <span style={{ color: "var(--text-muted)" }}>·</span>
                    <span style={{ color: "var(--text-muted)" }}>
                      Day {msg.day}
                    </span>
                  </div>
                </div>

                {/* Priority badge */}
                <span
                  className="px-2 py-0.5 rounded text-xs font-semibold shrink-0"
                  style={{
                    backgroundColor: pStyle.bg,
                    color: pStyle.color,
                  }}
                >
                  {msg.priority}
                </span>

                {/* Blocking indicator for urgent */}
                {msg.priority === "Urgent" && (
                  <span title="Blocks Continue">
                    <ShieldAlert
                      size={16}
                      style={{ color: "var(--color-loss)" }}
                    />
                  </span>
                )}

                {/* Resolve Action */}
                {!msg.read &&
                  (msg.priority === "Urgent" || msg.priority === "Action") && (
                    <button
                      onClick={() =>
                        onResolveMessage &&
                        onResolveMessage(msg.id, msg.subject)
                      }
                      disabled={resolvingMsgId === msg.id}
                      className="app-button-primary ml-4 px-3 py-1.5 text-xs font-semibold rounded-xl transition-colors"
                      style={{
                        background: "var(--accent-gradient)",
                        color: "#fff",
                        opacity: resolvingMsgId === msg.id ? 0.7 : 1,
                      }}
                    >
                      {resolvingMsgId === msg.id
                        ? "Resolving..."
                        : msg.subject === "Tournament Concluded"
                          ? "View Results"
                          : "Resolve"}
                    </button>
                  )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
