// Lee y valida la biblioteca de práctica (web/CONTENIDO.md §7) y escribe src/generated/practica.json.
//   contenido/preguntas/*.yaml  (+ contenido/demo-preguntas.yaml)   → preguntas
//   contenido/listas.yaml       (+ contenido/demo-listas.yaml)      → listas
//   contenido/quizzes.yaml      (+ contenido/demo-quizzes.yaml)     → quizzes
// Debe ejecutarse después de compilar-ejemplos.ts (comprueba que cada `ej` y cada problema existen).
import { Glob } from "bun";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { parse as parseYaml } from "yaml";

const RAIZ = resolve(import.meta.dir, "..");
const CONT = join(RAIZ, "contenido");
const SALIDA = join(RAIZ, "src", "generated", "practica.json");
const TOLERANTE = process.env.RDC_TOLERANTE === "1";
const TIPOS = new Set(["opcion", "respuesta", "predice", "no-compila"]);
const DIFS = new Set(["facil", "media", "dificil"]);

const leerJson = (p: string) => (existsSync(p) ? JSON.parse(readFileSync(p, "utf8")) : {});
const ejemplos: Record<string, any> = leerJson(join(RAIZ, "src", "generated", "ejemplos.json"));
const problemas: Record<string, any> = leerJson(join(RAIZ, "src", "generated", "problemas.json"));

const errores: string[] = [];
function yaml(ruta: string): any[] {
  if (!existsSync(ruta)) return [];
  try {
    const v = parseYaml(readFileSync(ruta, "utf8").replace(/\r\n/g, "\n"));
    if (v == null) return [];
    if (!Array.isArray(v)) { errores.push(`${rel(ruta)}: debe ser una lista YAML (cada elemento empieza por «- »)`); return []; }
    return v.map((x) => ({ ...x, __archivo: rel(ruta) }));
  } catch (e) {
    errores.push(`${rel(ruta)}: YAML inválido — ${(e as Error).message.split("\n")[0]}`);
    return [];
  }
}
const rel = (p: string) => p.slice(RAIZ.length + 1).replaceAll("\\", "/");

export type Pregunta = {
  id: string; titulo: string; mundo: string | null; tema: string; dificultad: "facil" | "media" | "dificil";
  tipo: "opcion" | "respuesta" | "predice" | "no-compila"; enunciado: string; ej: string | null; codigo: string | null;
  opciones: string[]; correcta: number; respuestas: string[]; solucion: string; etiquetas: string[]; archivo: string;
};

const archivosP: string[] = [];
if (existsSync(join(CONT, "preguntas")))
  for await (const f of new Glob("*.{yaml,yml}").scan({ cwd: join(CONT, "preguntas"), onlyFiles: true })) archivosP.push(join(CONT, "preguntas", f));
archivosP.sort();
archivosP.push(join(CONT, "demo-preguntas.yaml"));

const preguntas: Pregunta[] = [];
const ids = new Set<string>();
for (const a of archivosP) {
  for (const [i, q] of yaml(a).entries()) {
    const donde = `${q.__archivo} (elemento ${i + 1}${q?.id ? `, id «${q.id}»` : ""})`;
    const antes = errores.length;
    const mal = (m: string) => errores.push(`${donde}: ${m}`);
    if (!q || typeof q !== "object") { mal("no es un objeto"); continue; }
    if (!q.id || !/^[a-z0-9][a-z0-9-]*$/.test(String(q.id))) { mal('"id" obligatorio, en minúsculas con guiones'); continue; }
    if (ids.has(q.id)) { mal(`"id" repetido`); continue; }
    ids.add(q.id);
    const tipo = String(q.tipo ?? "opcion");
    if (!TIPOS.has(tipo)) mal(`"tipo" debe ser opcion, respuesta, predice o no-compila (es «${tipo}»)`);
    const dificultad = String(q.dificultad ?? "media");
    if (!DIFS.has(dificultad)) mal(`"dificultad" debe ser facil, media o dificil`);
    if (!q.titulo) mal('falta "titulo"');
    if (!q.enunciado) mal('falta "enunciado"');
    if (!q.solucion) mal('falta "solucion" (la explicación detallada)');
    const ej = q.ej ? String(q.ej).replace(/^\/+|\.rs$/g, "") : null;
    if (ej && !ejemplos[ej]) mal(`"ej": no existe el ejemplo «ejemplos/${ej}.rs» (o no compiló)`);
    if (ej && tipo === "no-compila" && ejemplos[ej] && !ejemplos[ej].debeFallar) mal(`"ej" «${ej}» debería tener "debe_fallar": true en su .json (es una pregunta no-compila)`);
    if (tipo === "predice" && !ej && !q.codigo) mal('tipo predice: hace falta "ej" (ejemplo ejecutable) o "codigo"');
    const opciones = Array.isArray(q.opciones) ? q.opciones.map(String) : [];
    const correcta = Number(q.correcta ?? 0);
    if (tipo !== "respuesta" && (opciones.length < 2 || !Number.isInteger(correcta) || correcta < 0 || correcta >= opciones.length))
      mal('"opciones" (≥ 2) y "correcta" (índice desde 0) obligatorios para este tipo');
    const respuestas = Array.isArray(q.respuestas) ? q.respuestas.map(String) : [];
    if (tipo === "respuesta" && !respuestas.length) mal('tipo respuesta: falta "respuestas"');
    if (TOLERANTE && errores.length > antes) continue; // RDC_TOLERANTE: se omite la pregunta rota
    preguntas.push({
      id: String(q.id), titulo: String(q.titulo ?? q.id), mundo: q.mundo ? String(q.mundo) : null, tema: String(q.tema ?? "general"),
      dificultad: dificultad as Pregunta["dificultad"], tipo: tipo as Pregunta["tipo"], enunciado: String(q.enunciado ?? ""),
      ej, codigo: q.codigo ? String(q.codigo).replace(/\n+$/, "") : null, opciones, correcta, respuestas,
      solucion: String(q.solucion ?? ""), etiquetas: Array.isArray(q.etiquetas) ? q.etiquetas.map(String) : [], archivo: q.__archivo,
    });
  }
}

const lista = (v: unknown) => (Array.isArray(v) ? v.map(String) : []);
const listas = [...yaml(join(CONT, "listas.yaml")), ...yaml(join(CONT, "demo-listas.yaml"))].map((l, i) => {
  const donde = `${l.__archivo} (lista ${i + 1}${l?.id ? ` «${l.id}»` : ""})`;
  if (!l.id || !l.titulo) errores.push(`${donde}: faltan "id" o "titulo"`);
  const ps = lista(l.preguntas), pr = lista(l.problemas);
  for (const p of ps) if (!ids.has(p)) errores.push(`${donde}: la pregunta «${p}» no existe`);
  for (const p of pr) if (!problemas[p]) errores.push(`${donde}: el problema «${p}» no existe en problemas/`);
  return { id: String(l.id), titulo: String(l.titulo ?? l.id), descripcion: String(l.descripcion ?? ""), preguntas: ps, problemas: pr };
});

const quizzes = [...yaml(join(CONT, "quizzes.yaml")), ...yaml(join(CONT, "demo-quizzes.yaml"))].map((q, i) => {
  const donde = `${q.__archivo} (quiz ${i + 1}${q?.id ? ` «${q.id}»` : ""})`;
  if (!q.id || !q.titulo) errores.push(`${donde}: faltan "id" o "titulo"`);
  let ps = lista(q.preguntas);
  if (!ps.length && q.filtro) {
    const f = q.filtro;
    ps = preguntas.filter((p) => (!f.tema || p.tema === f.tema) && (!f.mundo || p.mundo === f.mundo) && (!f.dificultad || p.dificultad === f.dificultad)).map((p) => p.id).slice(0, Number(f.n ?? 10));
  }
  for (const p of ps) if (!ids.has(p)) errores.push(`${donde}: la pregunta «${p}» no existe`);
  if (!ps.length) errores.push(`${donde}: no tiene preguntas`);
  return { id: String(q.id), titulo: String(q.titulo ?? q.id), descripcion: String(q.descripcion ?? ""), tiempo: Number(q.tiempo ?? 300), preguntas: ps };
});

if (errores.length) {
  console.error(`\n✗ generar-practica: ${errores.length} problema(s):\n`);
  for (const e of errores) console.error("  • " + e);
  if (!TOLERANTE) { console.error("\nEl build se detiene (ver web/CONTENIDO.md §7)."); process.exit(1); }
}
mkdirSync(dirname(SALIDA), { recursive: true });
const nuevo = JSON.stringify({ preguntas, listas, quizzes }, null, 1);
if (!existsSync(SALIDA) || readFileSync(SALIDA, "utf8") !== nuevo) writeFileSync(SALIDA, nuevo);
console.log(`generar-practica: ${preguntas.length} pregunta(s), ${Object.keys(problemas).length} problema(s), ${listas.length} lista(s), ${quizzes.length} quiz(zes)`);
