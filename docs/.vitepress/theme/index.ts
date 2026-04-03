import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import "./custom.css";

function initSearchAndNavFixes() {
  // Fix same-site nav links
  document.querySelectorAll(".VPNavBarMenuLink, .VPNavScreenMenuLink").forEach((link) => {
    const el = link as HTMLAnchorElement;
    if (el.href && !el.href.includes("github.com")) {
      el.removeAttribute("target");
      el.removeAttribute("rel");
    }
  });

  // Check if search button already added (SPA navigation re-triggers this)
  if (document.querySelector(".re-search-btn")) return;

  // Find insertion point — before the extras (locale + dark toggle + github)
  const navBar = document.querySelector(".VPNavBarExtra");
  if (!navBar?.parentNode) return;

  // Create search button
  const btn = document.createElement("button");
  btn.className = "re-search-btn";
  btn.setAttribute("aria-label", "Search");
  btn.setAttribute("type", "button");

  const isMac = navigator.platform?.includes("Mac");
  const iconSvg = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="11" cy="11" r="8"></circle><path d="M21 21l-4.35-4.35"></path></svg>`;

  btn.insertAdjacentHTML("beforeend", iconSvg);

  const label = document.createElement("span");
  label.className = "re-search-label";
  label.textContent = "Search";
  btn.appendChild(label);

  const kbd = document.createElement("kbd");
  kbd.textContent = isMac ? "⌘K" : "Ctrl+K";
  btn.appendChild(kbd);

  navBar.parentNode.insertBefore(btn, navBar);

  // Create modal (once)
  if (document.querySelector(".re-search-modal")) return;

  const modal = document.createElement("div");
  modal.className = "re-search-modal";
  modal.setAttribute("aria-hidden", "true");

  const backdrop = document.createElement("div");
  backdrop.className = "re-search-backdrop";
  modal.appendChild(backdrop);

  const content = document.createElement("div");
  content.className = "re-search-content";

  const container = document.createElement("div");
  container.id = "re-pagefind";
  content.appendChild(container);

  const closeBtn = document.createElement("button");
  closeBtn.className = "re-search-close";
  closeBtn.setAttribute("aria-label", "Close search");
  closeBtn.setAttribute("type", "button");
  closeBtn.textContent = "✕";
  content.appendChild(closeBtn);

  modal.appendChild(content);
  document.body.appendChild(modal);

  let loaded = false;

  function openSearch() {
    modal.setAttribute("aria-hidden", "false");
    modal.classList.add("is-open");
    document.body.style.overflow = "hidden";

    if (!loaded) {
      loaded = true;
      const css = document.createElement("link");
      css.rel = "stylesheet";
      css.href = "/_pagefind/pagefind-ui.css";
      document.head.appendChild(css);

      const script = document.createElement("script");
      script.src = "/_pagefind/pagefind-ui.js";
      script.onload = () => {
        new (window as any).PagefindUI({
          element: "#re-pagefind",
          showSubResults: true,
          showImages: false,
        });
        setTimeout(() => {
          (document.querySelector("#re-pagefind input") as HTMLInputElement)?.focus();
        }, 100);
      };
      document.head.appendChild(script);
    } else {
      setTimeout(() => {
        (document.querySelector("#re-pagefind input") as HTMLInputElement)?.focus();
      }, 50);
    }
  }

  function closeSearch() {
    modal.setAttribute("aria-hidden", "true");
    modal.classList.remove("is-open");
    document.body.style.overflow = "";
  }

  btn.addEventListener("click", openSearch);
  backdrop.addEventListener("click", closeSearch);
  closeBtn.addEventListener("click", closeSearch);

  document.addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      e.stopPropagation();
      if (modal.classList.contains("is-open")) closeSearch();
      else openSearch();
    }
    if (e.key === "Escape" && modal.classList.contains("is-open")) closeSearch();
  });
}

const theme: Theme = {
  ...DefaultTheme,
  enhanceApp({ router }) {
    if (typeof window === "undefined") return;

    // Sync theme
    const reTheme = localStorage.getItem("ralph-theme");
    if (reTheme) localStorage.setItem("vitepress-theme-appearance", reTheme);

    const observer = new MutationObserver(() => {
      const isDark = document.documentElement.classList.contains("dark");
      localStorage.setItem("ralph-theme", isDark ? "dark" : "light");
    });
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });

    // Run after page is ready and on each SPA navigation
    router.onAfterRouteChanged = () => {
      setTimeout(initSearchAndNavFixes, 50);
    };

    // Also run on initial load
    if (document.readyState === "complete") {
      setTimeout(initSearchAndNavFixes, 50);
    } else {
      window.addEventListener("load", () => setTimeout(initSearchAndNavFixes, 50));
    }
  },
};

export default theme;
