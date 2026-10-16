import AxeBuilder from "@axe-core/playwright";
import { expect, type Page } from "@playwright/test";

/** Starts a game through the UI: Ana, Azure Tigers, the default save name. */
export async function startGame(page: Page): Promise<void> {
  await page.goto("/");
  await page.getByRole("link", { name: "New Game" }).click();
  await page.getByLabel("Manager name").fill("Ana");
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByRole("tab", { name: "Seorabeol Pro League" }).click();
  await page.getByText("Azure Tigers").click();
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByRole("button", { name: "Start" }).click();
  await expect(page.getByRole("heading", { name: "Azure Tigers" })).toBeVisible();
}

/** axe finds nothing on the page as it is now. */
export async function expectNoViolations(page: Page): Promise<void> {
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
}

/** The page fits its viewport: nothing scrolls sideways. */
export async function expectNoHorizontalScroll(page: Page): Promise<void> {
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  expect(overflow).toBeLessThanOrEqual(0);
}
