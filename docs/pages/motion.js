(() => {
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (reduce) {
    document.querySelectorAll(".hero, .block, footer").forEach((el) => {
      el.style.opacity = "1";
      el.style.transform = "none";
      el.style.animation = "none";
    });
  }

  document.querySelectorAll(".btn").forEach((btn) => {
    btn.addEventListener("pointerenter", () => {
      if (reduce) return;
      btn.animate(
        [{ transform: "translateY(0)" }, { transform: "translateY(-1px)" }, { transform: "translateY(0)" }],
        { duration: 220, easing: "ease-out" },
      );
    });
  });
})();
