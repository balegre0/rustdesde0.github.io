// Revisión rápida del MDX antes del build: el compilador de MDX se para en el primer error y da
// posiciones engañosas. Aquí se listan TODOS los casos de los errores más comunes, con su arreglo.
import { Glob } from "bun";
import { join } from "node:path";

const DOCS = join(import.meta.dir, "..", "src", "content", "docs");
const problemas: string[] = [];
for await (const f of new Glob("**/*.mdx").scan({ cwd: DOCS })) {
  const lineas = (await Bun.file(join(DOCS, f)).text()).split(/\r?\n/);
  let enCodigo = false;
  lineas.forEach((l, i) => {
    if (/^\s*```/.test(l)) enCodigo = !enCodigo;
    if (enCodigo) return;
    // atributo JSX entre comillas dobles con \" dentro: en MDX las comillas no se escapan así
    if (/(^|\s)[a-zA-Z]+="[^"]*\\"/.test(l))
      problemas.push(`${f.replaceAll("\\", "/")}:${i + 1}: atributo con \\" dentro de comillas dobles. En MDX usa llaves: pregunta={"… \\"…\\" …"} (o comillas simples dentro).`);
    if (/<!--/.test(l))
      problemas.push(`${f.replaceAll("\\", "/")}:${i + 1}: comentario HTML <!-- -->. En MDX los comentarios se escriben {/* así */}.`);
  });
}
if (problemas.length) {
  console.error(`\n✗ revisar-contenido: ${problemas.length} problema(s) de sintaxis MDX:\n`);
  for (const p of problemas) console.error("  • " + p);
  console.error("");
  process.exit(1);
}
console.log("revisar-contenido: MDX sin problemas conocidos");
