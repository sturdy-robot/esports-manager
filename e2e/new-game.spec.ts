import { expect, test } from "@playwright/test";
import { expectNoHorizontalScroll, expectNoViolations, startGame } from "./support";

test("the main menu shows the game and its version", async ({ page }) => {
  // Given the game is running
  // When the user opens it
  await page.goto("/");
  // Then the shell, the backend's version and the menu are shown
  await expect(page.getByRole("heading", { name: "eSports Manager" })).toBeVisible();
  await expect(page.getByTestId("version")).toHaveText("v0.0.0-e2e");
  await expect(page.getByRole("link", { name: "New Game" })).toBeVisible();
});

test("the wizard ends on the roster", async ({ page }) => {
  // spec: NEWG-008
  // Given the main menu
  // When the user enters a manager name, picks a team and starts
  await startGame(page);
  // Then the roster of that team is shown
  await expect(page.getByText("Seorabeol Pro League")).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Kairos, view profile" }).locator("visible=true"),
  ).toBeVisible();
});

test("the wizard works with the keyboard alone", async ({ page }) => {
  // spec: NEWG-009
  // Given the main menu
  await page.goto("/");
  // When the user completes New Game with Tab, the arrow keys, Space and Enter
  await page.getByRole("link", { name: "New Game" }).focus();
  await page.keyboard.press("Enter");
  await page.getByLabel("Manager name").focus();
  await page.keyboard.type("Ana");
  await page.keyboard.press("Enter");
  await page.getByRole("tab", { name: "Seorabeol Pro League" }).focus();
  await page.keyboard.press("Enter");
  // Tab into the team group, then move through it with the arrow keys.
  await page.keyboard.press("Tab");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("Space");
  await page.getByRole("button", { name: "Next" }).focus();
  await page.keyboard.press("Enter");
  await page.getByLabel("Save name").focus();
  await page.keyboard.press("Enter");
  // Then the roster is shown
  await expect(page.getByRole("heading", { name: "Azure Tigers" })).toBeVisible();
});

test("going back keeps what was typed", async ({ page }) => {
  // spec: NEWG-010
  // Given the wizard on step 2, with manager name "Ana"
  await page.goto("/new");
  await page.getByLabel("Manager name").fill("Ana");
  await page.getByRole("button", { name: "Next" }).click();
  await expect(page.getByText("Step 2 of 3")).toBeVisible();
  // When the user goes back to step 1
  await page.getByRole("button", { name: "Back" }).click();
  // Then the manager name still holds "Ana"
  await expect(page.getByLabel("Manager name")).toHaveValue("Ana");
});

test("the save name defaults to the team's name", async ({ page }) => {
  await page.goto("/new");
  await page.getByLabel("Manager name").fill("Ana");
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByRole("tab", { name: "Liga Pindorama" }).click();
  await page.getByText("Onça Gaming").click();
  await page.getByRole("button", { name: "Next" }).click();
  await expect(page.getByLabel("Save name")).toHaveValue("Onça Gaming");
});

test("problems from the backend are shown on the last step", async ({ page }) => {
  await page.goto("/new");
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByRole("tab", { name: "Seorabeol Pro League" }).click();
  await page.getByText("Azure Tigers").click();
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByRole("button", { name: "Start" }).click();
  await expect(page.getByRole("alert")).toContainText("Enter a manager name.");
});

test.describe("with a broken package", () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.__ESM_E2E__ = { brokenPackage: true };
    });
  });

  test("a broken package shows its report", async ({ page }) => {
    // spec: NEWG-007
    // Given a core package with a validation error
    // When a new game is started in the UI
    await page.goto("/new");
    // Then the report is shown with the error's file and path
    const alert = page.getByRole("alert");
    await expect(alert).toContainText("teams/seorabeol/azure_tigers.json");
    await expect(alert).toContainText('unknown field "reputaton"');
  });
});

test.describe("with saves", () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.__ESM_E2E__ = { saves: true };
    });
  });

  test("Load Game opens a save on its roster", async ({ page }) => {
    // spec: NEWG-011
    // Given a save in the data folder
    await page.goto("/");
    // When the user opens Load Game and picks it
    await page.getByRole("link", { name: "Load Game" }).click();
    await page.getByRole("button", { name: /Ana's Tigers/ }).click();
    // Then the roster of that save's team is shown
    await expect(page.getByRole("heading", { name: "Azure Tigers" })).toBeVisible();
  });

  test("Load Game passes the accessibility audit", async ({ page }) => {
    // spec: NEWG-013
    await page.goto("/load");
    await expect(page.getByRole("button", { name: /Ana's Tigers/ })).toBeVisible();
    await expectNoViolations(page);
    await expectNoHorizontalScroll(page);
  });
});

test("the menu and every New Game step pass the accessibility audit", async ({ page }) => {
  // spec: NEWG-013
  // Given the main menu and each New Game step
  await page.goto("/");
  await expect(page.getByTestId("version")).toBeVisible();
  // When each is shown, in this project's layout
  // Then axe reports nothing and the page doesn't scroll sideways
  await expectNoViolations(page);
  await expectNoHorizontalScroll(page);
  await page.getByRole("link", { name: "New Game" }).click();
  await page.getByLabel("Manager name").fill("Ana");
  await expectNoViolations(page);
  await page.getByRole("button", { name: "Next" }).click();
  await page.getByText("Onça Gaming").click();
  await expectNoViolations(page);
  await expectNoHorizontalScroll(page);
  await page.getByRole("button", { name: "Next" }).click();
  await expectNoViolations(page);
  await page.goto("/load");
  await expectNoViolations(page);
});

test.describe("for a Brazilian Portuguese user", () => {
  test.use({ locale: "pt-BR" });

  test("the document language follows the user's locale", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("html")).toHaveAttribute("lang", "pt-BR");
    await expect(page.getByRole("link", { name: "Novo Jogo" })).toBeVisible();
  });
});
