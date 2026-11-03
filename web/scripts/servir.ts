// Servidor estático mínimo de dist/ (para probar el build). Uso: bun run scripts/servir.ts [puerto]
// Con BASE_PATH=/rust-desde-cero/ sirve dist/ en esa subcarpeta, como GitHub Pages.
import { join, normalize } from "node:path";
import { existsSync, statSync } from "node:fs";

const DIST = join(import.meta.dir, "..", "dist");
const port = Number(process.argv[2] ?? process.env.PORT ?? 4322);
const BASE = (process.env.BASE_PATH ?? "/").replace(/^\/*/, "/").replace(/\/*$/, "/");
const tipos: Record<string, string> = { ".wasm": "application/wasm", ".js": "text/javascript", ".mjs": "text/javascript", ".json": "application/json" };

const server = Bun.serve({
  port,
  async fetch(req) {
    const url = new URL(req.url);
    if (url.pathname + "/" === BASE) return Response.redirect(BASE, 301);
    const fuera = !url.pathname.startsWith(BASE);
    url.pathname = "/" + url.pathname.slice(BASE.length);
    let p = normalize(decodeURIComponent(url.pathname)).replace(/^([\/])+/, "");
    let f = join(DIST, p);
    if (!f.startsWith(DIST)) return new Response("no", { status: 403 });
    if (existsSync(f) && statSync(f).isDirectory()) f = join(f, "index.html");
    else if (!existsSync(f) && existsSync(f + ".html")) f += ".html";
    if (fuera || !existsSync(f)) return new Response(Bun.file(join(DIST, "404.html")), { status: 404, headers: { "Content-Type": "text/html; charset=utf-8" } });
    const ext = f.slice(f.lastIndexOf("."));
    const headers: Record<string, string> = {};
    if (tipos[ext]) headers["Content-Type"] = tipos[ext];
    if (ext === ".html") headers["Cache-Control"] = "no-cache";
    return new Response(Bun.file(f), { headers });
  },
});
console.log(`Sirviendo dist/ en http://localhost:${server.port}${BASE}`);
