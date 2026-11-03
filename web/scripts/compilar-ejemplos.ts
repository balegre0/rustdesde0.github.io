// Compila los ejemplos y problemas de Rust y escribe los datos que consume la web.
//
//   ejemplos/<mundo>/<id>.rs         → public/wasm/<mundo>/<id>.wasm (+ <id>.release.wasm)
//   ejemplos/<mundo>/<id>.arreglo.rs → public/wasm/<mundo>/<id>.arreglo.wasm   (si el .rs debe fallar)
//   problemas/<slug>/{enunciado.md,inicio.rs,solucion.rs,pruebas.rs} → comprobación + src/generated/problemas.json
//   ⇒ src/generated/ejemplos.json
//
// Uso: bun run scripts/compilar-ejemplos.ts [--forzar]
//
// Reglas (web/CONTENIDO.md §4 y §7.2):
// - "debe_fallar": rustc --emit=metadata --error-format=json; se guardan los diagnósticos reales.
//   El build FALLA si el ejemplo compila (o si el código de error no es el esperado, cuando se indica).
// - Resto: rustc --target wasm32-wasip1 por perfil (debug: overflow-checks y debug-assertions on;
//   release: off), con -D warnings (salvo "avisos": true), y se ejecuta con node:wasi en un subproceso
//   (Bun termina el proceso en proc_exit) con el stdin del .json y tiempo límite.
// - "plataforma": "playground" | "local": no se ejecuta (se conserva el código; se comprueba que compila
//   con --emit=metadata si no usa crates externos).
// - Caché por hash (código + metadatos + flags + versión de rustc): recompilar es casi instantáneo.
import { Glob } from "bun";
import { cpus } from "node:os";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { parse as parseYaml } from "yaml";

const RAIZ = resolve(import.meta.dir, "..");
const EJEMPLOS = join(RAIZ, "ejemplos");
const PROBLEMAS = join(RAIZ, "problemas");
const SALIDA_WASM = join(RAIZ, "public", "wasm");
const GENERADO = join(RAIZ, "src", "generated", "ejemplos.json");
const GENERADO_PROB = join(RAIZ, "src", "generated", "problemas.json");
const TMP = join(RAIZ, "node_modules", ".cache", "rdc-rustc");
const EJECUTOR = join(import.meta.dir, "lib", "ejecutor-wasi.ts");
const FORZAR = process.argv.includes("--forzar");
const TOLERANTE = process.env.RDC_TOLERANTE === "1";
const LIMITE_MS = 10_000;
const MARCA_OK = "TODOS LOS CASOS SUPERADOS";
const PLATAFORMAS = new Set(["wasm", "playground", "local"]);
const PERFILES = new Set(["debug", "release"]);

const BASE_WASM = ["--edition", "2024", "--target", "wasm32-wasip1", "-C", "opt-level=s", "-C", "strip=symbols", "--crate-type", "bin"];
const FLAGS_PERFIL: Record<Perfil, string[]> = {
  debug: ["-C", "overflow-checks=on", "-C", "debug-assertions=on"],
  release: ["-C", "overflow-checks=off", "-C", "debug-assertions=off"],
};
const BASE_META = ["--edition", "2024", "--crate-type", "bin", "--emit=metadata", "--error-format=json"];

type Perfil = "debug" | "release";
type Meta = {
  stdin?: string; perfiles?: string[]; debe_fallar?: boolean | string; plataforma?: string;
  nota?: string; avisos?: boolean; flags?: string;
};
export type Diagnostico = {
  nivel: string; codigo: string | null; mensaje: string; linea: number | null; columna: number | null;
  lineaFin: number | null; columnaFin: number | null; etiqueta: string | null;
  secundarios: { linea: number; columna: number; columnaFin: number; etiqueta: string | null }[];
  notas: string[]; renderizado: string;
};
export type Ejecucion = {
  salida: string; error: string | null; exit: number | null; trap: string | null;
  panico: { mensaje: string; ubicacion: string } | null; wasm: string;
};
export type Ejemplo = {
  id: string; mundo: string; codigo: string; inicio: string | null; stdin: string;
  plataforma: "wasm" | "playground" | "local"; nota: string | null; perfiles: Perfil[];
  debeFallar: boolean; diagnosticos: Diagnostico[] | null;
  ejecuciones: Partial<Record<Perfil, Ejecucion>>;
  arreglo: { codigo: string; ejecucion: Ejecucion } | null;
  // Atajos al primer perfil (compatibilidad con <Reto>, juegos, etc.)
  salida: string | null; error: string | null; exit: number | null; trap: string | null; wasm: string | null;
  hash: string;
};
export type Problema = {
  slug: string; titulo: string; mundo: string | null; tema: string | null; dificultad: string; orden: number;
  enunciado: string; inicio: string; solucion: string; pruebas: string; salidaSolucion: string; hash: string;
};

const lf = (s: string) => s.replace(/\r\n/g, "\n");
const leer = (p: string) => lf(readFileSync(p, "utf8"));

/** JSON con comentarios `//` y comas finales tolerados (el contrato los usa en su ejemplo). */
function leerJson(ruta: string): Meta {
  const texto = readFileSync(ruta, "utf8");
  try { return JSON.parse(texto); } catch { /* sigue */ }
  let out = "", enCadena = false;
  for (let i = 0; i < texto.length; i++) {
    const c = texto[i];
    if (enCadena) { out += c; if (c === "\\") out += texto[++i] ?? ""; else if (c === '"') enCadena = false; }
    else if (c === '"') { enCadena = true; out += c; }
    else if (c === "/" && texto[i + 1] === "/") { while (i < texto.length && texto[i] !== "\n") i++; out += "\n"; }
    else out += c;
  }
  return JSON.parse(out.replace(/,(\s*[}\]])/g, "$1"));
}

function hashDe(...partes: string[]) {
  const h = new Bun.CryptoHasher("sha256");
  for (const p of partes) h.update(p + "\0");
  return h.digest("hex").slice(0, 16);
}
const partirFlags = (s?: string) => (s ? (s.match(/"[^"]*"|'[^']*'|\S+/g) ?? []).map((f) => f.replace(/^["']|["']$/g, "")) : []);
const indentar = (s: string) => s.split("\n").map((l) => "      " + l).join("\n");

const VERSION = Bun.spawnSync(["rustc", "-V"]).stdout.toString().trim();
if (!VERSION) { console.error("✗ compilar-ejemplos: no encuentro `rustc` en el PATH."); process.exit(1); }

/** Escribe el código en un directorio temporal como `src/main.rs` (así los mensajes dicen `src/main.rs:4:13`, como cargo). */
function preparar(clave: string, codigo: string) {
  const dir = join(TMP, clave);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(join(dir, "src", "main.rs"), codigo);
  return dir;
}

async function rustc(args: string[], cwd: string) {
  const p = Bun.spawn(["rustc", ...args], { cwd, stdout: "pipe", stderr: "pipe" });
  const [out, err, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]);
  return { out, err: lf(err), code };
}

/** Quita rutas locales (sysroot de rustup) de los mensajes: `C:\Users\…\library\core\src\x.rs` → `/rustc/library/core/src/x.rs`. */
function limpiarRutas(s: string) {
  // También `/home/…/rustlib/…` al compilar en Linux (GitHub Actions).
  return s.replace(/(?:[A-Za-z]:[\\/]|(?<![\w/])\/)[^\s:]*?rustlib[\\/]src[\\/]rust[\\/]([^\s:]+)/g, (_, r: string) => "/rustc/" + r.replaceAll("\\", "/"));
}

function parsearDiagnosticos(json: string): Diagnostico[] {
  const out: Diagnostico[] = [];
  for (const l of json.split("\n")) {
    if (!l.startsWith("{")) continue;
    let d: any;
    try { d = JSON.parse(l); } catch { continue; }
    if (d.$message_type && d.$message_type !== "diagnostic") continue;
    if (d.level !== "error" && d.level !== "warning") continue;
    if (/^aborting due to/.test(d.message)) continue;
    const prim = (d.spans ?? []).find((s: any) => s.is_primary) ?? d.spans?.[0];
    out.push({
      nivel: d.level,
      codigo: d.code?.code ?? null,
      mensaje: d.message,
      linea: prim?.line_start ?? null,
      columna: prim?.column_start ?? null,
      lineaFin: prim?.line_end ?? null,
      columnaFin: prim?.column_end ?? null,
      etiqueta: prim?.label ?? null,
      secundarios: (d.spans ?? []).filter((s: any) => s !== prim).map((s: any) => ({ linea: s.line_start, columna: s.column_start, columnaFin: s.column_end, etiqueta: s.label ?? null })),
      notas: (d.children ?? []).map((c: any) => `${c.level}: ${c.message}`),
      renderizado: limpiarRutas(lf(d.rendered ?? "")).replace(/\n+$/, ""),
    });
  }
  return out;
}

/** Ejecuta un .wasm con node:wasi en un subproceso (stdin, tiempo límite) e interpreta pánico/trap. */
async function ejecutar(wasmAbs: string, wasmRel: string, stdin: string): Promise<Ejecucion | string> {
  const run = Bun.spawn([process.execPath, EJECUTOR, wasmAbs], {
    stdin: new Blob([stdin]), stdout: "pipe", stderr: "pipe", cwd: RAIZ, timeout: LIMITE_MS, killSignal: "SIGKILL",
  });
  const [out, err, code] = await Promise.all([new Response(run.stdout).text(), new Response(run.stderr).text(), run.exited]);
  if (run.signalCode) return `la ejecución superó ${LIMITE_MS / 1000} s (¿bucle infinito o espera entrada que no está en "stdin" del .json?)`;
  let error = limpiarRutas(lf(err)).replace(/^\n+/, "");
  let trap: string | null = null;
  const m = error.match(/\n?@@TRAP@@ (.*)\n?$/);
  if (m) { trap = m[1]; error = error.slice(0, m.index); }
  // La nota de RUST_BACKTRACE no aplica en el navegador: se quita (la explicación la pone la web).
  error = error.replace(/^note: run with `RUST_BACKTRACE=1`[^\n]*\n?/m, "");
  let panico: Ejecucion["panico"] = null;
  const pm = error.match(/thread '[^']*'(?: \(\d+\))? panicked at ([^\n]+?):\n([\s\S]*?)\n?$/);
  if (pm) panico = { ubicacion: pm[1], mensaje: pm[2].trim() };
  return {
    salida: lf(out),
    error: error.trim() ? error : null,
    exit: panico ? 101 : trap ? null : code,
    trap: panico ? null : trap,
    panico,
    wasm: wasmRel,
  };
}

async function compilarWasm(clave: string, codigo: string, perfil: Perfil, extra: string[], destino: string) {
  const dir = preparar(clave, codigo);
  mkdirSync(dirname(destino), { recursive: true });
  return rustc([...BASE_WASM, ...FLAGS_PERFIL[perfil], ...extra, "src/main.rs", "-o", destino], dir);
}

async function main() {
  const t0 = performance.now();
  mkdirSync(EJEMPLOS, { recursive: true });
  mkdirSync(TMP, { recursive: true });

  let previo: Record<string, Ejemplo> = {};
  let previoProb: Record<string, Problema> = {};
  if (!FORZAR) {
    try { previo = JSON.parse(readFileSync(GENERADO, "utf8")); } catch { /* nada */ }
    try { previoProb = JSON.parse(readFileSync(GENERADO_PROB, "utf8")); } catch { /* nada */ }
  }

  const archivos: string[] = [];
  for await (const f of new Glob("**/*.rs").scan({ cwd: EJEMPLOS, onlyFiles: true })) {
    const rel = f.replaceAll("\\", "/");
    if (/\.(inicio|arreglo)\.rs$/.test(rel)) continue;
    archivos.push(rel);
  }
  archivos.sort();

  const errores: string[] = [];
  const avisos: string[] = [];
  const resultado: Record<string, Ejemplo> = {};
  let compilados = 0, reutilizados = 0;

  async function procesar(rel: string) {
    const id = rel.slice(0, -3);
    const partes = id.split("/");
    const mundo = partes.length > 1 ? partes[0] : "";
    const ruta = (suf: string) => join(EJEMPLOS, id + suf);
    const codigo = leer(ruta(".rs"));
    const inicio = existsSync(ruta(".inicio.rs")) ? leer(ruta(".inicio.rs")) : null;
    const arregloCod = existsSync(ruta(".arreglo.rs")) ? leer(ruta(".arreglo.rs")) : null;
    let meta: Meta = {};
    if (existsSync(ruta(".json"))) {
      try { meta = leerJson(ruta(".json")); }
      catch (e) { errores.push(`ejemplos/${id}.json: JSON inválido (${(e as Error).message})`); return; }
    }
    const plataforma = (meta.plataforma ?? "wasm") as Ejemplo["plataforma"];
    if (!PLATAFORMAS.has(plataforma)) { errores.push(`ejemplos/${id}.json: "plataforma" debe ser wasm, playground o local (es "${meta.plataforma}")`); return; }
    const perfiles = (meta.perfiles ?? ["debug"]) as Perfil[];
    if (!Array.isArray(perfiles) || !perfiles.length || perfiles.some((p) => !PERFILES.has(p))) { errores.push(`ejemplos/${id}.json: "perfiles" debe ser una lista con "debug" y/o "release"`); return; }
    const debeFallar = meta.debe_fallar === true || typeof meta.debe_fallar === "string";
    const codigoEsperado = typeof meta.debe_fallar === "string" ? meta.debe_fallar : null;
    const stdin = lf(meta.stdin ?? "");
    const extra = [...(meta.avisos ? [] : ["-D", "warnings"]), ...partirFlags(meta.flags)];
    const hash = hashDe(VERSION, codigo, JSON.stringify(meta), extra.join(" "), inicio ?? "", arregloCod ?? "", "v3");
    const wasmRel = (p: Perfil, suf = "") => `wasm/${id}${suf}${p === "release" ? ".release" : ""}.wasm`;

    const base: Ejemplo = {
      id, mundo, codigo, inicio, stdin, plataforma, nota: meta.nota ?? null, perfiles, debeFallar,
      diagnosticos: null, ejecuciones: {}, arreglo: null, salida: null, error: null, exit: null, trap: null, wasm: null, hash,
    };

    const ant = previo[id];
    const wasmsOk = (e: Ejemplo) => [...Object.values(e.ejecuciones), e.arreglo?.ejecucion].every((x) => !x || existsSync(join(RAIZ, "public", x.wasm)));
    if (ant && ant.hash === hash && wasmsOk(ant)) { resultado[id] = { ...ant, codigo, inicio }; reutilizados++; return; }

    const clave = hashDe(id);
    if (debeFallar) {
      const dir = preparar(clave + "-m", codigo);
      const r = await rustc([...BASE_META, "src/main.rs", "-o", join(dir, "x.rmeta")], dir);
      if (r.code === 0) { errores.push(`ejemplos/${rel}: tiene "debe_fallar" pero COMPILA. Si debía fallar, revisa el código; si no, quita "debe_fallar" del .json.`); return; }
      const diags = parsearDiagnosticos(r.err).filter((d) => d.nivel === "error");
      if (!diags.length) { errores.push(`ejemplos/${rel}: rustc falló sin diagnósticos legibles:\n${indentar(r.err)}`); return; }
      if (codigoEsperado && !diags.some((d) => d.codigo === codigoEsperado))
        errores.push(`ejemplos/${rel}: se esperaba el error ${codigoEsperado} y rustc dio ${diags.map((d) => d.codigo ?? "(sin código)").join(", ")}`);
      base.diagnosticos = diags;
      if (arregloCod !== null) {
        const dest = join(RAIZ, "public", wasmRel("debug", ".arreglo"));
        const c = await compilarWasm(clave + "-a", arregloCod, "debug", extra, dest);
        if (c.code !== 0) { errores.push(`ejemplos/${id}.arreglo.rs: NO COMPILA (el arreglo debe compilar sin avisos)\n${indentar(c.err.trim())}`); return; }
        const e = await ejecutar(dest, wasmRel("debug", ".arreglo"), stdin);
        if (typeof e === "string") { errores.push(`ejemplos/${id}.arreglo.rs: ${e}`); return; }
        base.arreglo = { codigo: arregloCod, ejecucion: e };
      }
      resultado[id] = base;
      compilados++;
      return;
    }

    if (arregloCod !== null) avisos.push(`ejemplos/${id}.arreglo.rs: solo se usa si ${id}.rs tiene "debe_fallar"; se ignora.`);

    if (plataforma !== "wasm") {
      // No se ejecuta: se comprueba que compila (en nativo, solo metadatos) salvo que use crates externos.
      if (!/^\s*(use|extern crate)\s+(?!std|core|alloc|crate|self|super)\w+/m.test(codigo)) {
        const dir = preparar(clave + "-m", codigo);
        // Si el .json pide otro --crate-type (lib, staticlib…), sustituye al "bin" de BASE_META.
        const meta0 = extra.includes("--crate-type") ? BASE_META.filter((x, i, a) => x !== "--crate-type" && a[i - 1] !== "--crate-type") : BASE_META;
        const r = await rustc([...meta0, ...extra, "src/main.rs", "-o", join(dir, "x.rmeta")], dir);
        if (r.code !== 0) {
          const d = parsearDiagnosticos(r.err).map((x) => x.renderizado).join("\n");
          errores.push(`ejemplos/${rel}: NO COMPILA (plataforma "${plataforma}")\n${indentar(d || r.err)}`);
          return;
        }
      }
      resultado[id] = base;
      compilados++;
      return;
    }

    for (const p of perfiles) {
      const rel2 = wasmRel(p);
      const dest = join(RAIZ, "public", rel2);
      const c = await compilarWasm(clave + "-" + p, codigo, p, extra, dest);
      if (c.code !== 0) {
        const esAviso = /warnings?\]|#\[deny\(warnings\)\]|-D warnings/.test(c.err);
        errores.push(`ejemplos/${rel}: ${esAviso ? 'rustc da avisos (warnings). Corrige el código o, si el aviso es a propósito, pon "avisos": true en el .json' : "NO COMPILA"} (perfil ${p})\n${indentar(c.err.trim())}`);
        return;
      }
      const e = await ejecutar(dest, rel2, stdin);
      if (typeof e === "string") { errores.push(`ejemplos/${rel}: ${e}`); return; }
      if (e.trap) avisos.push(`ejemplos/${rel}: terminó con un trap de wasm (${e.trap}) sin mensaje de pánico`);
      base.ejecuciones[p] = e;
    }
    const pr = base.ejecuciones[perfiles[0]]!;
    Object.assign(base, { salida: pr.salida, error: pr.error, exit: pr.exit, trap: pr.trap, wasm: pr.wasm });
    resultado[id] = base;
    compilados++;
  }

  // ---------- Problemas ----------
  const problemas: Record<string, Problema> = {};
  const slugs: string[] = [];
  if (existsSync(PROBLEMAS)) for await (const f of new Glob("*/enunciado.md").scan({ cwd: PROBLEMAS, onlyFiles: true })) slugs.push(f.replaceAll("\\", "/").split("/")[0]);
  slugs.sort();

  async function procesarProblema(slug: string) {
    const d = join(PROBLEMAS, slug);
    const faltan = ["enunciado.md", "inicio.rs", "solucion.rs", "pruebas.rs"].filter((f) => !existsSync(join(d, f)));
    if (faltan.length) { errores.push(`problemas/${slug}: faltan ${faltan.join(", ")}`); return; }
    const md = leer(join(d, "enunciado.md"));
    const fm = md.match(/^---\n([\s\S]*?)\n---\n?/);
    let datos: any = {};
    try { datos = fm ? parseYaml(fm[1]) ?? {} : {}; } catch (e) { errores.push(`problemas/${slug}/enunciado.md: frontmatter YAML inválido (${(e as Error).message})`); return; }
    if (!datos.titulo) errores.push(`problemas/${slug}/enunciado.md: falta "titulo" en el frontmatter`);
    const dif = datos.dificultad ?? "media";
    if (!["facil", "media", "dificil"].includes(dif)) errores.push(`problemas/${slug}/enunciado.md: "dificultad" debe ser facil, media o dificil`);
    const inicio = leer(join(d, "inicio.rs")), solucion = leer(join(d, "solucion.rs")), pruebas = leer(join(d, "pruebas.rs"));
    const hash = hashDe(VERSION, md, inicio, solucion, pruebas, "p2");
    const meta = {
      slug, titulo: String(datos.titulo ?? slug), mundo: datos.mundo ? String(datos.mundo) : null, tema: datos.tema ? String(datos.tema) : null,
      dificultad: dif, orden: Number(datos.orden ?? 0), enunciado: fm ? md.slice(fm[0].length) : md, inicio, solucion, pruebas, hash,
    };
    const ant = previoProb[slug];
    if (ant && ant.hash === hash) { problemas[slug] = ant; reutilizados++; return; }
    const clave = "p-" + hashDe(slug);
    const dest = join(TMP, clave, "sol.wasm");
    const c = await compilarWasm(clave + "-s", solucion + "\n" + pruebas, "debug", [], dest);
    if (c.code !== 0) { errores.push(`problemas/${slug}: solucion.rs + pruebas.rs NO COMPILA\n${indentar(c.err.trim())}`); return; }
    const e = await ejecutar(dest, "", "");
    if (typeof e === "string") { errores.push(`problemas/${slug}: solucion.rs + pruebas.rs: ${e}`); return; }
    if (!e.salida.includes(MARCA_OK)) { errores.push(`problemas/${slug}: solucion.rs + pruebas.rs no imprime «${MARCA_OK}»:\n${indentar((e.salida + (e.error ?? "")).trim())}`); return; }
    const ci = await compilarWasm(clave + "-i", inicio + "\n" + pruebas, "debug", ["-A", "warnings"], join(TMP, clave, "ini.wasm"));
    if (ci.code !== 0) { errores.push(`problemas/${slug}: inicio.rs + pruebas.rs NO COMPILA (debe compilar, con todo!() donde toque)\n${indentar(ci.err.trim())}`); return; }
    const ei = await ejecutar(join(TMP, clave, "ini.wasm"), "", "");
    if (typeof ei !== "string" && ei.salida.includes(MARCA_OK)) { errores.push(`problemas/${slug}: ¡inicio.rs ya supera las pruebas! El código inicial no debe resolver el problema.`); return; }
    problemas[slug] = { ...meta, salidaSolucion: e.salida };
    compilados++;
  }

  const cola: (() => Promise<void>)[] = [...archivos.map((a) => () => procesar(a)), ...slugs.map((s) => () => procesarProblema(s))];
  const n = Math.max(2, Math.min(cpus().length, 12));
  await Promise.all(Array.from({ length: n }, async () => { while (cola.length) await cola.shift()!(); }));

  for await (const f of new Glob("**/*.{inicio,arreglo}.rs").scan({ cwd: EJEMPLOS, onlyFiles: true })) {
    const rel = f.replaceAll("\\", "/");
    const sol = rel.replace(/\.(inicio|arreglo)\.rs$/, ".rs");
    if (!existsSync(join(EJEMPLOS, sol))) errores.push(`ejemplos/${rel}: falta ${sol}`);
  }

  for (const a of avisos) console.warn("aviso: " + a);
  if (errores.length) {
    console.error(`\n✗ compilar-ejemplos: ${errores.length} problema(s):\n`);
    for (const e of errores) console.error("  • " + e + "\n");
    if (!TOLERANTE) {
      console.error("El build se detiene. Arregla los ejemplos de arriba (ver web/CONTENIDO.md §4 y §7.2).");
      process.exit(1);
    }
    console.error("RDC_TOLERANTE=1: se sigue sin esos ejemplos (solo para depurar la plataforma; el build normal fallaría).");
  }

  mkdirSync(dirname(GENERADO), { recursive: true });
  const escribir = (ruta: string, obj: Record<string, unknown>) => {
    const nuevo = JSON.stringify(Object.fromEntries(Object.keys(obj).sort().map((k) => [k, obj[k]])), null, 1);
    if (!existsSync(ruta) || readFileSync(ruta, "utf8") !== nuevo) writeFileSync(ruta, nuevo);
  };
  escribir(GENERADO, resultado);
  escribir(GENERADO_PROB, problemas);
  console.log(
    `compilar-ejemplos: ${archivos.length} ejemplo(s) y ${slugs.length} problema(s) · ${compilados} compilado(s) · ${reutilizados} sin cambios · ` +
      `${(performance.now() - t0).toFixed(0)} ms (${VERSION}) → ${relative(RAIZ, GENERADO)}`,
  );
}

await main();
