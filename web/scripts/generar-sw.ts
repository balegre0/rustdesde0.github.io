// Post-build: genera dist/sw.js (offline). Sin Workbox: @vite-pwa/astro no funciona con Bun 1.4.4.
// - Precache de HTML, CSS, JS, fuentes, índices y Pagefind. Los .wasm de los ejemplos, al primer uso.
// - /wasm/: cache-first (se guardan al ejecutarlos por primera vez).
// - Páginas: red primero y, sin conexión, la copia guardada.
import { Glob } from "bun";
import { join } from "node:path";

const DIST = join(import.meta.dir, "..", "dist");
// Base de publicación (BASE_PATH=/rust-desde-cero/ en GitHub Pages); las rutas del precache la llevan delante.
const BASE = (process.env.BASE_PATH ?? "/").replace(/^\/*/, "/").replace(/\/*$/, "/");
const EXT = /\.(html|css|js|mjs|wasm|json|svg|woff2|png|ico|webmanifest|pagefind|pf_meta|pf_fragment|pf_index)$/;
const MAX_BYTES = 4 * 1024 * 1024;
const ESENCIAL = /^\/(index\.html|404\.html|curso\/.*|demo\/.*|(repaso|progreso|panel|arbol|preguntas|problemas|listas|quizzes|juegos|ref|errores|recetas|explicaciones)\/index\.html)$/;

const archivos: string[] = [];
let bytes = 0;
for await (const f of new Glob("**/*").scan({ cwd: DIST, onlyFiles: true })) {
  const p = "/" + f.replaceAll("\\", "/");
  // Los .wasm (≈60 KB cada uno, más de mil) se guardan al usarlos (cache-first), no en la instalación.
  if (p === "/sw.js" || p.startsWith("/wasm/") || !EXT.test(p)) continue;
  // HTML: se precargan la portada, los índices y las lecciones (curso/); el resto de páginas (miles de
  // preguntas, problemas y fichas de referencia) se guardan al visitarlas (red primero, copia sin conexión).
  if (p.endsWith(".html") && !ESENCIAL.test(p)) continue;
  const size = Bun.file(join(DIST, f)).size;
  if (size > MAX_BYTES) continue;
  bytes += size;
  archivos.push(p.endsWith("/index.html") ? p.slice(0, -"index.html".length) : p);
}
const real = (u: string) => join(DIST, u.endsWith("/") ? u + "index.html" : u);
archivos.sort();
const h = new Bun.CryptoHasher("sha256");
for (const f of archivos) {
  h.update(f);
  h.update(new Uint8Array(await Bun.file(real(f)).arrayBuffer()));
}
h.update(BASE);
const version = h.digest("hex").slice(0, 12);
const precache = archivos.map((f) => BASE + f.slice(1));

const sw = `// Generado por scripts/generar-sw.ts — no editar.
const VERSION = ${JSON.stringify(version)};
const CACHE = "rdc-" + VERSION;
const BASE = ${JSON.stringify(BASE)};
const PRECACHE = ${JSON.stringify(precache)};

self.addEventListener("install", (e) => {
  e.waitUntil((async () => {
    const c = await caches.open(CACHE);
    // De 40 en 40 para no saturar la red; un fallo puntual no impide instalar
    for (let i = 0; i < PRECACHE.length; i += 40) {
      await Promise.all(PRECACHE.slice(i, i + 40).map((u) => c.add(new Request(u, { cache: "reload" })).catch(() => {})));
    }
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (e) => {
  e.waitUntil((async () => {
    await self.clients.claim();
  })());
});

async function cacheFirst(req, nombre) {
  const c = await caches.open(nombre);
  const hit = await c.match(req, { ignoreSearch: true });
  if (hit) return hit;
  const r = await fetch(req);
  if (r.ok) c.put(req, r.clone());
  return r;
}

self.addEventListener("fetch", (e) => {
  const req = e.request;
  if (req.method !== "GET") return;
  const url = new URL(req.url);
  if (url.origin !== location.origin) return;
  if (!url.pathname.startsWith(BASE)) return;
  const ruta = url.pathname.slice(BASE.length - 1);
  if (ruta.startsWith("/wasm/") || ruta.startsWith("/_astro/") || ruta.startsWith("/pagefind/")) {
    e.respondWith(cacheFirst(req, CACHE));
    return;
  }
  if (req.mode === "navigate") {
    e.respondWith((async () => {
      try {
        const r = await fetch(req);
        if (r.ok) (await caches.open(CACHE)).put(req, r.clone());
        return r;
      } catch {
        const c = await caches.open(CACHE);
        return (await c.match(req, { ignoreSearch: true })) || (await c.match(url.pathname.endsWith("/") ? url.pathname : url.pathname + "/")) || (await c.match(BASE + "404.html")) || Response.error();
      }
    })());
    return;
  }
  e.respondWith(caches.match(req, { ignoreSearch: true }).then((r) => r || fetch(req)));
});
`;
await Bun.write(join(DIST, "sw.js"), sw);
console.log(`generar-sw: ${archivos.length} archivos en precache (${(bytes / 1024 / 1024).toFixed(1)} MB), versión ${version}`);
