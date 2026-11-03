// Ejecuta un .wasm WASI (compilado con `zig cc -target wasm32-wasi`) y muestra su salida real.
// Uso: bun run scripts/correr-wasm.ts programa.wasm [args...] < entrada.txt
import { WASI } from "node:wasi";

const [file, ...args] = process.argv.slice(2);
if (!file) {
  console.error("uso: bun run scripts/correr-wasm.ts programa.wasm [args...]");
  process.exit(2);
}
const wasi = new WASI({ version: "preview1", args: [file, ...args], env: {}, returnOnExit: true });
const module = await WebAssembly.compile(await Bun.file(file).arrayBuffer());
const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi.wasiImport });
const code = wasi.start(instance);
process.exitCode = code ?? 0;
