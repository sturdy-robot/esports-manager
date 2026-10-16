import { expect, test } from "@playwright/test";
import { expectNoHorizontalScroll, expectNoViolations, startGame } from "./support";

const kairos = "Kairos, view profile";

test("the roster opens a profile from the keyboard", async ({ page }) => {
  // spec: ROST-007
  // Given the roster screen
  await startGame(page);
  // When the user tabs to a player and presses Enter
  await page.getByRole("link", { name: kairos }).locator("visible=true").focus();
  await page.keyboard.press("Enter");
  // Then that player's profile is shown, and Back returns to the roster
  await expect(page.getByRole("heading", { name: "Kairos" })).toBeVisible();
  await expect(page.getByText("Han Do-yun")).toBeVisible();
  await page.getByRole("link", { name: "Back to roster" }).click();
  await expect(page.getByRole("heading", { name: "Azure Tigers" })).toBeVisible();
});

test("the compact roster is a card list", async ({ page }, testInfo) => {
  // spec: ROST-008
  test.skip(testInfo.project.name !== "compact", "the compact layout only");
  // Given the roster screen at phone width
  await startGame(page);
  // When it is shown
  // Then players are cards, and the page doesn't scroll sideways
  await expect(page.locator("table").first()).toBeHidden();
  await expect(page.getByRole("listitem").filter({ hasText: "Kairos" })).toBeVisible();
  await expectNoHorizontalScroll(page);
});

test("the wide roster is a table", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "desktop", "the wide layout only");
  await startGame(page);
  const starters = page.getByRole("table", { name: "Starters" });
  await expect(starters).toBeVisible();
  await expect(starters.getByRole("row")).toHaveCount(6);
  await expect(starters.getByRole("columnheader", { name: "Overall" })).toBeVisible();
});

test.describe("for a Brazilian Portuguese user", () => {
  test.use({ locale: "pt-BR" });

  test("the roster and profile are translated", async ({ page }) => {
    // spec: ROST-009
    // Given the locale pt-BR
    await page.goto("/");
    await page.getByRole("link", { name: "Novo Jogo" }).click();
    await page.getByLabel("Nome do treinador").fill("Ana");
    await page.getByRole("button", { name: "Avançar" }).click();
    await page.getByRole("tab", { name: "Liga Profissional Seorabeol" }).click();
    await page.getByText("Azure Tigers").click();
    await page.getByRole("button", { name: "Avançar" }).click();
    await page.getByRole("button", { name: "Começar" }).click();
    // When the roster and a profile are shown
    await expect(page.getByRole("heading", { name: "Titulares" })).toBeVisible();
    await expect(page.getByText("Liga Profissional Seorabeol")).toBeVisible();
    await expect(page.getByText("Caçador").locator("visible=true").first()).toBeVisible();
    await page.getByRole("link", { name: "Kairos, ver perfil" }).locator("visible=true").click();
    // Then role and attribute names, headings and the country are in Portuguese
    await expect(page.getByRole("heading", { name: "Habilidades" })).toBeVisible();
    await expect(page.getByText("Mecânica")).toBeVisible();
    await expect(page.getByText("Coreia do Sul (KR)")).toBeVisible();
    await expect(page.getByText("Meio", { exact: true })).toBeVisible();
  });
});

test("the roster and profile pass the accessibility audit", async ({ page }) => {
  // spec: ROST-012
  // Given the roster and a profile
  await startGame(page);
  // When each is shown in this project's layout
  // Then axe reports no violations
  await expectNoViolations(page);
  await page.getByRole("link", { name: kairos }).locator("visible=true").click();
  await expect(page.getByRole("heading", { name: "Kairos" })).toBeVisible();
  await expectNoViolations(page);
  await expectNoHorizontalScroll(page);
});

test("hidden attributes are unknown on the profile", async ({ page }) => {
  await startGame(page);
  await page.getByRole("link", { name: kairos }).locator("visible=true").click();
  await expect(page.getByText("Consistency")).toBeVisible();
  await expect(page.getByText("unknown").first()).toBeAttached();
});
