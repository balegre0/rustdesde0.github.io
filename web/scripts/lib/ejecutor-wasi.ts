// Ejecutor interno de compilar-ejemplos.ts: corre un .wasm WASI con node:wasi (stdin/stdout/stderr del proceso).
// Si el programa termina con un trap (UB detectado, acceso fuera de límites…) escribe en stderr
// una última línea «@@TRAP@@ <mensaje>» y sale con código 134.
import { WASI } from "node:wasi";

const [file, ...args] = process.argv.slice(2);
const wasi = new WASI({ version: "preview1", args: ["programa", ...args], env: {}, returnOnExit: true });
const module = await WebAssembly.compile(await Bun.file(file).arrayBuffer());
const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi.wasiImport });
try {
  const code = wasi.start(instance);
  process.exitCode = code ?? 0;
} catch (e) {
  const msg = String((e as Error)?.message ?? e).replace(/\s*\(evaluating .*\)$/, "").split("\n")[0];
  process.stderr.write(`\n@@TRAP@@ ${msg}\n`);
  process.exitCode = 134;
}
