# Emacs desde cero — curso avanzado

Curso de Emacs en español, integrado en el mismo sitio que **Rust desde cero**, en la ruta `/emacs/`.

## Qué incluye

- **14 módulos**, de “qué es Emacs” a bytecode, GC y contribución.
- **70 lecciones** de 6–15 minutos, con:
  - Predicción antes de leer.
  - Explicación conceptual.
  - Código Elisp para copiar a Emacs.
  - Sección “Bajo el capó”.
  - Trampa típica.
  - Preguntas de comprobación.
  - Reto con solución.
  - Tarjetas de repaso.
- **140 preguntas** con solución explicada y tasa de acierto.
- **70 retos** de Elisp.
- **140 tarjetas** de repaso espaciado.
- **Quizzes** cronometrados por módulo y global.
- **Listas** curadas: Emacs 50, Elisp 25, Org 20, Magit 20, Entrevista 40, Comandos 30.
- **Progreso local**: sin cuentas, exportable/importable en JSON.
- Tema oscuro/claro, búsqueda, responsive y navegación por teclado.

## Cómo se usa

El curso es una aplicación estática dentro de `web/public/emacs/`. Astro la copia a `dist/emacs/` en cada build.

```bash
cd web
bun install
bun run dev        # abre el sitio en localhost:4331; ve a /emacs/
bun run build      # genera dist/ con /emacs/ incluido
```

También podés abrir `web/public/emacs/index.html` con un servidor estático. No funciona con `file://` porque usa módulos ES.

## Método

1. **Predice**: te comprometés con una respuesta antes de leer.
2. **La idea**: explicación corta y precisa.
3. **Pruébalo**: copiás el código a `*scratch*` y lo evaluás con `C-x C-e` o `M-:`.
4. **Bajo el capó**: el modelo mental, no solo el atajo.
5. **Trampa típica**: el error que comete casi todo el mundo.
6. **Comprueba**: recuperación activa.
7. **Tu turno**: escribís tu versión y comparás.
8. **Repaso**: tarjetas espaciadas.

El curso **no ejecuta Elisp en el navegador** (sería inseguro y limitado). La corrección automática está en preguntas y quizzes; los retos se comparan con una solución. La práctica real ocurre en tu Emacs.

## Estructura

```
web/public/emacs/
  index.html          # shell de la SPA
  styles.css          # diseño, temas, responsive
  app.js              # router, lecciones, preguntas, quizzes, progreso
  data/
    modules.js        # 14 módulos y sus lecciones
    lessons-1.js      # m00–m03 (20 lecciones)
    lessons-2.js      # m04–m07 (20)
    lessons-3.js      # m08–m11 (20)
    lessons-4.js      # m12–m13 (10)
    glossary.js       # 52 términos
    index.js          # combina y exporta
  README.md
```

## Añadir una lección

1. Agregá el objeto en el archivo `lessons-*.js` correspondiente.
2. Agregá su `id` al array `lessons` del módulo en `data/modules.js`.
3. Ejecutá `bun run test:emacs`.

Esquema mínimo:

```js
{
  id: 'm06-l06', module: 'm06', n: 6, title: '...', min: 8, level: 'Avanzado',
  obj: '...', frase: '...',
  idea: ['...', '...'],
  code: { lang: 'elisp', src: '...', cap: '...' },
  under: ['...'],
  trap: { bad: '...', good: '...', why: '...' },
  q: [ { q: '...', o: ['...', '...'], a: 0, why: '...' }, { q: '...', o: ['...', '...'], a: 1, why: '...' } ],
  ex: { p: '...', start: '...', sol: '...', hint: '...' },
  cards: [ ['...', '...'], ['...', '...'] ],
  tags: ['org', 'comandos']
}
```

Los `q` se convierten automáticamente en el banco de preguntas. Los `ex` se convierten en retos. Las `cards` alimentan el repaso espaciado.

## Verificación

```bash
cd web
bun run test:emacs
```

Esto valida estructura y datos, y ejecuta una prueba E2E en Edge sobre la portada, una lección, preguntas, retos, quiz, progreso y tema.

## Nota sobre precisión

El contenido está escrito para Emacs 29.1+ y evita deliberadamente afirmaciones dependientes de una plataforma cuando no aplican. Ejemplos:

- El **daemon** (`emacs --daemon`) no está soportado oficialmente en Windows; ahí se usa `M-x server-start` + `emacsclient`.
- Los atajos de terceros (Embark, Corfu, Vertico, etc.) se muestran como configuración de ejemplo, no como defaults universales.
- Los ejemplos de procesos que usan `cat`/`sleep` se marcan como dependientes del sistema; en Windows pueden requerir otro comando.

Si encontrás una inexactitud, corregí el dato y agregá una pregunta que la cubra.
