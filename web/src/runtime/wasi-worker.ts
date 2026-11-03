/// <reference lib="webworker" />
// Worker que ejecuta un .wasm WASI con stdin interactivo vía JSPI (WebAssembly.Suspending/promising).
// Basado en investigacion/pruebas/jspi/worker.js (verificado en Edge sin COOP/COEP).
//
// Protocolo:
//   principal → worker: {type:'run', url?|bytes?, stdin?: string, interactivo: boolean}
//                       {type:'stdin', text}   {type:'eof'}
//   worker → principal: {type:'out', fd, text}  {type:'need-input'}
//                       {type:'exit', code, ms, trap?}
import { WASI, File, OpenFile, ConsoleStdout, PreopenDirectory, WASIProcExit } from '@bjorn3/browser_wasi_shim';

const pendientes: Uint8Array[] = [];
let despertar: (() => void) | null = null;
const siguiente = (): Promise<Uint8Array> =>
  pendientes.length
    ? Promise.resolve(pendientes.shift()!)
    : new Promise((r) => (despertar = () => { despertar = null; r(pendientes.shift()!); }));

const enc = new TextEncoder();
const W = self as unknown as DedicatedWorkerGlobalScope;

W.onmessage = async (e: MessageEvent) => {
  const msg = e.data;
  if (msg.type === 'stdin') { pendientes.push(enc.encode(msg.text)); despertar?.(); return; }
  if (msg.type === 'eof') { pendientes.push(new Uint8Array(0)); despertar?.(); return; }
  if (msg.type !== 'run') return;

  const dec1 = new TextDecoder(), dec2 = new TextDecoder();
  const interactivo = msg.interactivo && typeof (WebAssembly as any).Suspending === 'function';
  const stdinFijo = enc.encode(msg.stdin ?? '');
  // Cada escritura se envía en el acto (si el programa entra luego en un bucle infinito, lo ya
  // escrito no se pierde). La consola del hilo principal agrupa el volcado al DOM por fotograma.
  const vaciar = () => {};
  const escribir = (fd: 1 | 2, text: string) => { if (text) W.postMessage({ type: 'out', fd, text }); };
  const fds = [
    new OpenFile(new File(interactivo ? [] : stdinFijo)),
    new ConsoleStdout((b: Uint8Array) => escribir(1, dec1.decode(b, { stream: true }))),
    new ConsoleStdout((b: Uint8Array) => escribir(2, dec2.decode(b, { stream: true }))),
    new PreopenDirectory('/', new Map()),
  ];
  const wasi = new WASI(['programa'], [], fds, { debug: false });
  let mem: WebAssembly.Memory;
  const imp: Record<string, any> = { ...wasi.wasiImport };
  let fin = false;

  if (interactivo) {
    const fdReadOriginal = wasi.wasiImport.fd_read;
    imp.fd_read = new (WebAssembly as any).Suspending(async (fd: number, iovs: number, iovsLen: number, nreadPtr: number) => {
      if (fd !== 0) return fdReadOriginal(fd, iovs, iovsLen, nreadPtr);
      let chunk: Uint8Array;
      if (fin) chunk = new Uint8Array(0);
      else {
        vaciar();
        if (!pendientes.length) W.postMessage({ type: 'need-input' });
        chunk = await siguiente();
        if (chunk.length === 0) fin = true;
      }
      const dv = new DataView(mem.buffer);
      let off = 0;
      for (let i = 0; i < iovsLen && off < chunk.length; i++) {
        const ptr = dv.getUint32(iovs + i * 8, true), len = dv.getUint32(iovs + i * 8 + 4, true);
        const n = Math.min(len, chunk.length - off);
        new Uint8Array(mem.buffer, ptr, n).set(chunk.subarray(off, off + n));
        off += n;
      }
      if (off < chunk.length) pendientes.unshift(chunk.subarray(off));
      new DataView(mem.buffer).setUint32(nreadPtr, off, true);
      return 0;
    });
  }

  const t0 = performance.now();
  const terminar = (extra: Record<string, unknown>) => {
    const r1 = dec1.decode(), r2 = dec2.decode();
    if (r1) escribir(1, r1);
    if (r2) escribir(2, r2);
    vaciar();
    W.postMessage({ type: 'exit', ms: performance.now() - t0, ...extra });
  };
  try {
    const fuente = msg.bytes
      ? await WebAssembly.instantiate(msg.bytes, { wasi_snapshot_preview1: imp })
      : await WebAssembly.instantiateStreaming(fetch(msg.url), { wasi_snapshot_preview1: imp });
    const instance = (fuente as WebAssembly.WebAssemblyInstantiatedSource).instance;
    mem = instance.exports.memory as WebAssembly.Memory;
    (wasi as any).inst = instance;
    const start = instance.exports._start as () => void;
    if (interactivo) await (WebAssembly as any).promising(start)();
    else start();
    terminar({ code: 0 });
  } catch (err: any) {
    if (err instanceof WASIProcExit) terminar({ code: err.code });
    else terminar({ code: null, trap: String(err?.message ?? err) });
  }
};
