# Rust desde cero + Emacs desde cero

Curso de Rust en español para empezar desde cero y llegar a dominarlo: lecciones con ejemplos que se
ejecutan en el navegador, más de mil preguntas, problemas con pruebas, quizzes y juegos.

Además incluye **Emacs desde cero**, un curso avanzado de Emacs en español con 14 módulos, 70
lecciones, 140 preguntas, 70 retos, quizzes y repaso espaciado. Vive en `/emacs/` y se construye con
el mismo sitio.

- La web está en `web/` (Astro + Starlight). Para trabajar en local: `cd web`, `bun install` y
  `bun run dev`.
- Para compilar todo: `bun run build`. Necesitas Rust 1.99.1 con el target `wasm32-wasip1`.
- Para validar el curso de Emacs: `cd web && bun run test:emacs`.
- Se publica sola en GitHub Pages con cada `push` a `main`. Los pasos están en [PUBLICAR.md](PUBLICAR.md).
