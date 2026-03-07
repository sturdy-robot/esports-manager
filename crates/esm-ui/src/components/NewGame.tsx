import { useState } from "react";
import { ArrowLeft, ChevronRight } from "lucide-react";

export interface ManagerFormData {
  firstName: string;
  lastName: string;
  nickname: string;
  nationality: string;
  esportType: EsportType;
}

interface NewGameProps {
  onBack: () => void;
  onStart: (data: ManagerFormData) => void;
}

type EsportType = "Moba" | "Rts" | "Fps";

const esportOptions: { value: EsportType; label: string }[] = [
  { value: "Moba", label: "MOBA" },
  { value: "Rts", label: "RTS" },
  { value: "Fps", label: "FPS" },
];

interface FormFieldProps {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

function FormField({ id, label, value, onChange, placeholder }: FormFieldProps) {
  return (
    <div className="flex flex-col gap-1.5">
      <label
        htmlFor={id}
        className="text-xs font-semibold uppercase tracking-wider"
        style={{ color: "var(--text-muted)" }}
      >
        {label}
      </label>
      <input
        id={id}
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="px-3 py-2 rounded-md text-sm border outline-none transition-colors"
        style={{
          backgroundColor: "var(--bg-elevated)",
          borderColor: "var(--border-subtle)",
          color: "var(--text-primary)",
        }}
        onFocus={(e) => {
          e.currentTarget.style.borderColor = "var(--color-accent-cyan)";
        }}
        onBlur={(e) => {
          e.currentTarget.style.borderColor = "var(--border-subtle)";
        }}
      />
    </div>
  );
}

export function NewGame({ onBack, onStart }: NewGameProps) {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [nickname, setNickname] = useState("");
  const [nationality, setNationality] = useState("");
  const [esportType, setEsportType] = useState<EsportType>("Moba");

  const isValid =
    firstName.trim().length > 0 &&
    lastName.trim().length > 0 &&
    nickname.trim().length > 0 &&
    nationality.trim().length > 0;

  const handleContinue = () => {
    if (isValid) {
      onStart({
        firstName: firstName.trim(),
        lastName: lastName.trim(),
        nickname: nickname.trim(),
        nationality: nationality.trim(),
        esportType,
      });
    }
  };

  return (
    <div
      className="flex flex-col min-h-screen w-full animate-fade-in-up"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Header */}
      <header
        className="flex items-center gap-3 h-14 px-6 border-b shrink-0"
        style={{
          borderColor: "var(--border-subtle)",
          backgroundColor: "var(--bg-surface)",
        }}
      >
        <button
          onClick={onBack}
          className="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium cursor-pointer border transition-colors"
          style={{
            borderColor: "var(--border-subtle)",
            backgroundColor: "transparent",
            color: "var(--text-secondary)",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = "var(--color-accent-cyan)";
            e.currentTarget.style.color = "var(--text-primary)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = "var(--border-subtle)";
            e.currentTarget.style.color = "var(--text-secondary)";
          }}
        >
          <ArrowLeft size={16} />
          Back
        </button>
        <h1
          className="text-xl font-bold"
          style={{ color: "var(--text-primary)" }}
        >
          Manager Creation
        </h1>
      </header>

      {/* Form */}
      <main className="flex-1 flex justify-center py-12 px-6">
        <div className="w-full max-w-lg flex flex-col gap-6">
          {/* Identity */}
          <div
            className="p-6 rounded-lg border"
            style={{
              backgroundColor: "var(--bg-surface)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <h2
              className="text-sm font-semibold uppercase tracking-wider mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              Identity
            </h2>
            <div className="grid grid-cols-2 gap-4">
              <FormField
                id="first-name"
                label="First Name"
                value={firstName}
                onChange={setFirstName}
                placeholder="Kim"
              />
              <FormField
                id="last-name"
                label="Last Name"
                value={lastName}
                onChange={setLastName}
                placeholder="Jeong-gyun"
              />
            </div>
            <div className="grid grid-cols-2 gap-4 mt-4">
              <FormField
                id="nickname"
                label="Nickname"
                value={nickname}
                onChange={setNickname}
                placeholder="kkOma"
              />
              <FormField
                id="nationality"
                label="Nationality"
                value={nationality}
                onChange={setNationality}
                placeholder="KR"
              />
            </div>
          </div>

          {/* eSport Type */}
          <div
            className="p-6 rounded-lg border"
            style={{
              backgroundColor: "var(--bg-surface)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <h2
              className="text-sm font-semibold uppercase tracking-wider mb-4"
              style={{ color: "var(--text-muted)" }}
            >
              <label htmlFor="esport-type">Esport Type</label>
            </h2>
            <div id="esport-type" className="flex gap-3" role="radiogroup" aria-label="Esport Type">
              {esportOptions.map((opt) => {
                const selected = esportType === opt.value;
                return (
                  <button
                    key={opt.value}
                    type="button"
                    role="radio"
                    aria-checked={selected}
                    onClick={() => setEsportType(opt.value)}
                    className="flex-1 px-4 py-3 rounded-md text-sm font-semibold cursor-pointer border transition-all"
                    style={{
                      backgroundColor: selected
                        ? "var(--bg-elevated)"
                        : "transparent",
                      borderColor: selected
                        ? "var(--color-accent-cyan)"
                        : "var(--border-subtle)",
                      color: selected
                        ? "var(--text-primary)"
                        : "var(--text-secondary)",
                      boxShadow: selected ? "var(--accent-glow)" : "none",
                    }}
                  >
                    {opt.label}
                  </button>
                );
              })}
            </div>
          </div>

          {/* Continue */}
          <button
            onClick={handleContinue}
            disabled={!isValid}
            className="flex items-center justify-center gap-2 w-full px-6 py-3 rounded-lg text-sm font-semibold cursor-pointer border-none transition-all"
            style={{
              background: isValid
                ? "linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))"
                : "var(--bg-elevated)",
              color: isValid ? "#fff" : "var(--text-muted)",
              opacity: isValid ? 1 : 0.6,
              cursor: isValid ? "pointer" : "not-allowed",
            }}
          >
            Continue
            <ChevronRight size={16} />
          </button>
        </div>
      </main>
    </div>
  );
}
