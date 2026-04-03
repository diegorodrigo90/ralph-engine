import DefaultTheme from "vitepress/theme";
import "./custom.css";

export default {
  ...DefaultTheme,
  enhanceApp() {
    if (typeof window !== "undefined") {
      // Sync ralph-theme → VitePress on page load
      const reTheme = localStorage.getItem("ralph-theme");
      if (reTheme) {
        localStorage.setItem("vitepress-theme-appearance", reTheme);
      }

      // Sync VitePress theme changes back to ralph-theme
      const observer = new MutationObserver(() => {
        const isDark = document.documentElement.classList.contains("dark");
        localStorage.setItem("ralph-theme", isDark ? "dark" : "light");
      });
      observer.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["class"],
      });

      // Fix same-site nav links: remove target="_blank" for Home/Plugins
      // VitePress treats all absolute URLs as external, but in production
      // Home and Plugins are on the same domain.
      requestAnimationFrame(() => {
        document.querySelectorAll(".VPNavBarMenuLink, .VPNavScreenMenuLink").forEach((link) => {
          const el = link as HTMLAnchorElement;
          if (el.href && !el.href.includes("github.com")) {
            el.removeAttribute("target");
            el.removeAttribute("rel");
          }
        });
      });
    }
  },
};
