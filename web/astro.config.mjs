// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import preact from '@astrojs/preact';
import starlightSidebarTopics from 'starlight-sidebar-topics';
import starlightLinksValidator from 'starlight-links-validator';
import { topics } from './scripts/lib/barra-lateral.mjs';

// Publicación en subcarpeta (GitHub Pages: BASE_PATH=/rust-desde-cero/). En local, la raíz.
const BASE = (process.env.BASE_PATH ?? '/').replace(/^\/*/, '/').replace(/\/*$/, '/');
const SITE = process.env.SITE_URL || 'https://rust-desde-cero.example';

// Atajos de búsqueda: Starlight ya abre la búsqueda con Ctrl+K y «/»; añadimos «S» (como rustdoc).
const atajos = `
document.addEventListener('keydown', (e) => {
  if (e.defaultPrevented || e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.key !== 's' && e.key !== 'S') return;
  const t = e.target;
  if (t instanceof HTMLElement && (t.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(t.tagName) || t.closest('.cm-editor'))) return;
  const b = document.querySelector('site-search button[data-open-modal]');
  if (b) { e.preventDefault(); b.click(); }
});
// Service worker (offline). En «astro dev» no existe /sw.js: no se registra.
if ('serviceWorker' in navigator) addEventListener('load', () => {
  if (document.querySelector('script[src*="@vite/client"]')) return;
  navigator.serviceWorker.register('${BASE}sw.js', { scope: '${BASE}' }).catch(() => {});
});`;

// Páginas de práctica (src/pages/*): sin barra lateral de temas.
const PRACTICA = ['/preguntas/**', '/problemas/**', '/listas/**', '/quizzes/**', '/juegos/**', '/arbol/', '/panel/', '/preguntas/', '/problemas/', '/listas/', '/quizzes/', '/juegos/'];

export default defineConfig({
  site: SITE,
  base: BASE,
  trailingSlash: 'ignore',
  integrations: [
    preact(),
    starlight({
      title: 'Rust desde cero',
      description: 'Practica y aprende Rust en español desde cero: preguntas, problemas, quizzes y módulos guiados, todo ejecutable en el navegador.',
      defaultLocale: 'root',
      locales: { root: { label: 'Español', lang: 'es' } },
      logo: { src: './src/assets/logo.svg', alt: 'Rust desde cero', replacesTitle: false },
      favicon: '/favicon.svg',
      customCss: [
        '@fontsource/ibm-plex-sans/400.css',
        '@fontsource/ibm-plex-sans/400-italic.css',
        '@fontsource/ibm-plex-sans/600.css',
        '@fontsource/ibm-plex-mono/400.css',
        '@fontsource/ibm-plex-mono/600.css',
        './src/styles/tema.css',
        './src/styles/componentes.css',
        './src/styles/rust.css',
        './src/styles/practica.css',
      ],
      components: {
        Header: './src/overrides/Header.astro',
        PageTitle: './src/overrides/PageTitle.astro',
        Pagination: './src/overrides/Pagination.astro',
        MarkdownContent: './src/overrides/MarkdownContent.astro',
      },
      head: [
        { tag: 'meta', attrs: { name: 'theme-color', content: '#111214' } },
        { tag: 'script', content: atajos },
      ],
      lastUpdated: false,
      pagination: true,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      expressiveCode: {
        themes: ['github-dark-default', 'github-light'],
        styleOverrides: {
          codeFontFamily: "'IBM Plex Mono', ui-monospace, Consolas, monospace",
          codeFontSize: '0.9375rem',
          codeLineHeight: '1.6',
          borderRadius: '0.375rem',
          uiFontFamily: "'IBM Plex Sans', system-ui, sans-serif",
          frames: { shadowColor: 'transparent' },
        },
        defaultProps: { wrap: false },
      },
      plugins: [
        // Con base, los enlaces del contenido se corrigen después (scripts/aplicar-base.ts): se validan en la build local.
        ...(process.env.RDC_SOLO || BASE !== '/' ? [] : [starlightLinksValidator({
          errorOnRelativeLinks: false,
          exclude: ['/recetas/', '/errores/', '/explicaciones/', '/datos/**', '/repaso/', '/progreso/', ...PRACTICA],
        })]),
        starlightSidebarTopics(topics(), {
          exclude: ['/', '/**'],
          topics: {
            curso: ['/repaso', '/progreso', '/glosario', '/chuleta', '/demo', '/demo/**', '/curso/**'],
            ref: ['/ref/**'],
            recetas: ['/recetas', '/recetas/**'],
            errores: ['/errores', '/errores/**'],
            explicaciones: ['/explicaciones', '/explicaciones/**'],
          },
        }),
      ],
    }),
  ],
  vite: {
    worker: { format: 'es' },
    build: { chunkSizeWarningLimit: 4000 },
  },
});
