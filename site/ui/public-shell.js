const root = document.documentElement;
const storageKey = "re-theme";
const themeToggle = document.querySelector("[data-theme-toggle]");
const menuToggle = document.querySelector("[data-menu-toggle]");
const menuPanel = document.querySelector("[data-menu-panel]");
const prefersDark = window.matchMedia("(prefers-color-scheme: dark)");

function applyTheme(theme, persist = false) {
  root.dataset.theme = theme;
  root.style.colorScheme = theme;

  if (persist) {
    localStorage.setItem(storageKey, theme);
  }

  const themeMeta = document.querySelector('meta[name="theme-color"]');
  if (themeMeta) {
    themeMeta.setAttribute("content", theme === "dark" ? "#0f172a" : "#5a67d8");
  }

  if (themeToggle) {
    themeToggle.dataset.theme = theme;
  }
}

function detectTheme() {
  const storedTheme = localStorage.getItem(storageKey);

  if (storedTheme === "light" || storedTheme === "dark") {
    return storedTheme;
  }

  return prefersDark.matches ? "dark" : "light";
}

function setMenuState(open) {
  if (!menuPanel || !menuToggle) {
    return;
  }

  menuPanel.dataset.open = open ? "true" : "false";
  menuToggle.setAttribute("aria-expanded", String(open));
  root.classList.toggle("menu-open", open);
}

applyTheme(detectTheme());
setMenuState(false);

if (themeToggle) {
  themeToggle.addEventListener("click", () => {
    const currentTheme = root.dataset.theme === "dark" ? "dark" : "light";
    applyTheme(currentTheme === "dark" ? "light" : "dark", true);
  });
}

if (menuToggle) {
  menuToggle.addEventListener("click", () => {
    setMenuState(menuPanel?.dataset.open !== "true");
  });
}

document.addEventListener("click", (event) => {
  if (!menuPanel || !menuToggle) {
    return;
  }

  const target = event.target;
  if (!(target instanceof Node)) {
    return;
  }

  if (menuPanel.contains(target) || menuToggle.contains(target)) {
    return;
  }

  setMenuState(false);
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    setMenuState(false);
  }
});

window.addEventListener("resize", () => {
  if (window.innerWidth >= 980) {
    setMenuState(false);
  }
});

prefersDark.addEventListener("change", (event) => {
  const storedTheme = localStorage.getItem(storageKey);

  if (storedTheme === "light" || storedTheme === "dark") {
    return;
  }

  applyTheme(event.matches ? "dark" : "light");
});
