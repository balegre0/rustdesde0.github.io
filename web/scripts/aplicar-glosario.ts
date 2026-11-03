// Post-build: en cada lección de dist/curso/, marca la primera aparición de cada término del glosario
// (contenido/glosario.yaml) para que muestre su definición. Va antes de aplicar-base.ts.
import { Glob } from "bun";
import { join } from "node:path";
import { crearAnotador } from "./lib/glosario.mjs";

const DIST = join(import.meta.dir, "..", "dist");
const anotar = crearAnotador();
let paginas = 0;
let terminos = 0;
for await (const f of new Glob("curso/*/*/index.html").scan({ cwd: DIST })) {
  const leccion = f.replaceAll("\\", "/").replace(/^curso\//, "").replace(/\/index\.html$/, "");
  const ruta = join(DIST, f);
  const antes = await Bun.file(ruta).text();
  const { html } = anotar(antes, leccion);
  const n = (html.match(/class="rdc-term"/g) ?? []).length;
  if (n) {
    await Bun.write(ruta, html);
    paginas++;
    terminos += n;
  }
}
console.log(`aplicar-glosario: ${terminos} términos marcados en ${paginas} lecciones`);
