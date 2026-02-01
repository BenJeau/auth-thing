import { test, expect } from "@playwright/test";

test.describe("Authentication", () => {
  test("Page redirects to login page", async ({ page }) => {
    await page.goto("http://localhost:3000");
    await expect(page).toHaveURL("http://localhost:3000/auth/login");
  });

  test("If signups are disabled, redirect to login page", async ({ page }) => {
    await page.goto("http://localhost:3001/auth/signup");
    await expect(page).toHaveURL("http://localhost:3001/auth/login");
  });
});
