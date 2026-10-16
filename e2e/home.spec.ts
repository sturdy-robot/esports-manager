import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("the home screen shows the game and its version", async ({ page }) => {
  // Given the game is running
  // When the user opens it
  await page.goto("/");
  // Then the shell and the backend's version are shown
  await expect(page.getByRole("heading", { name: "eSports Manager" })).toBeVisible();
  await expect(page.getByTestId("version")).toHaveText("v0.0.0-e2e");
});

test("the home screen has no detectable accessibility violations", async ({ page }) => {
  // Given the home screen has loaded
  await page.goto("/");
  await expect(page.getByTestId("version")).toBeVisible();
  // When axe audits it
  const results = await new AxeBuilder({ page }).analyze();
  // Then nothing is reported
  expect(results.violations).toEqual([]);
});

test.describe("for a Brazilian Portuguese user", () => {
  test.use({ locale: "pt-BR" });

  test("the document language follows the user's locale", async ({ page }) => {
    // Given the browser prefers pt-BR
    // When the user opens the game
    await page.goto("/");
    // Then the page is marked as Portuguese for assistive technology
    await expect(page.locator("html")).toHaveAttribute("lang", "pt-BR");
  });
});
