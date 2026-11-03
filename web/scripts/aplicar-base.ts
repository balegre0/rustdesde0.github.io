// Post-build: si la web se publica en una subcarpeta (BASE_PATH=/rust-desde-cero/, como en GitHub Pages),
// antepone esa base a las rutas absolutas del sitio que siguen empezando por «/» en dist/.
// Astro ya pone la base en lo suyo (recursos, enlaces de Starlight, lecciones.json); esto cubre los enlaces
// escritos a mano en el contenido, los componentes y las islas. Con BASE_PATH vacío o «/» no hace nada.
import { Glob } from "bun";
import { readdirSync } from "node:fs";
import { join } from "node:path";

const BASE = (process.env.BASE_PATH ?? "/").replace(/^\/*/, "/").replace(/\/*$/, "/");
if (BASE === "/") process.exit(0);
const DIST = join(import.meta.dir, "..", "dist");
const PREFIJO = BASE.slice(0, -1); // «/rust-desde-cero»

// Solo se reescriben rutas que apuntan a algo que existe en la raíz de dist/ (curso, ref, _astro, favicon.svg…).
const raiz = readdirSync(DIST).filter((n) => n !== "LEEME.txt");
if (raiz.includes(PREFIJO.slice(1))) throw new Error(`aplicar-base: dist/ ya contiene «${PREFIJO.slice(1)}»`);
const nombres = raiz.map((n) => n.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
const FIN = `(?=[/"'\`?#&)]|$)`;
// Tras una comilla (también escapada en HTML): "/curso/…", '/ref/…', `/problemas/${…}`, &quot;/datos/…&quot;
// (no si va tras «+»: es código que ya concatena la base, como `BASE.replace(/\/$/, "") + "/pagefind/"`).
const comillas = new RegExp(`(?<!\\+\\s*)(["'\`]|&quot;|&#34;|&#39;)/(?=(?:${nombres})${FIN})`, "g");
// url(/…) en CSS
const url = new RegExp(`(url\\(\\s*["']?)/(?=(?:${nombres})${FIN})`, "g");
// Enlaces a la portada: href="/"
const portada = /(\shref=)(["'])\/\2/g;

let archivos = 0;
let cambios = 0;
for await (const f of new Glob("**/*.{html,js,mjs,css,json,xml,webmanifest}").scan({ cwd: DIST, onlyFiles: true })) {
  const rel = f.replaceAll("\\", "/");
  if (rel.startsWith("pagefind/") || rel.startsWith("wasm/")) continue;
  const ruta = join(DIST, f);
  const antes = await Bun.file(ruta).text();
  let n = 0;
  let despues = antes.replace(comillas, (_, q) => (n++, `${q}${PREFIJO}/`));
  if (rel.endsWith(".css") || rel.endsWith(".html")) despues = despues.replace(url, (_, a) => (n++, `${a}${PREFIJO}/`));
  if (rel.endsWith(".html")) despues = despues.replace(portada, (_, a, q) => (n++, `${a}${q}${BASE}${q}`));
  if (n) {
    await Bun.write(ruta, despues);
    archivos++;
    cambios += n;
  }
}
console.log(`aplicar-base: base ${BASE} — ${cambios} rutas corregidas en ${archivos} archivos`);
