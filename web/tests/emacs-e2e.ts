// Prueba E2E del curso de Emacs (sitio estático en public/emacs).
// Uso: bun run tests/emacs-e2e.ts
import { chromium, type Page } from 'playwright-core';
import { join } from 'node:path';
import { mkdirSync } from 'node:fs';

const RAIZ = join(import.meta.dir, '..');
const PUBLIC = join(RAIZ, 'public', 'emacs');
const PUERTO = 4400;
const BASE = `http://127.0.0.1:${PUERTO}`;
mkdirSync(join(RAIZ, '.capturas'), { recursive: true });

const servidor = Bun.spawn([process.platform === 'win32' ? 'python.exe' : 'python3', '-m', 'http.server', String(PUERTO)], { cwd: PUBLIC, stdout: 'inherit', stderr: 'inherit' });
for (let i = 0; i < 24; i++) {
  try { const r = await fetch(BASE + '/'); if (r.ok) break; } catch {}
  await Bun.sleep(250);
}

const errores: string[] = [];
const fallos: string[] = [];
async function paso(nombre: string, fn: () => Promise<void>) {
  try { await fn(); console.log('  OK ' + nombre); }
  catch (e) { const m = (e as Error).message.split('\n')[0]; fallos.push(`${nombre}: ${m}`); console.log('  FAIL ' + nombre + ' - ' + m); }
}
const navegador = await chromium.launch({ channel: 'msedge', headless: true });
const ctx = await navegador.newContext({ viewport: { width: 1440, height: 900 }, colorScheme: 'dark' });
const page = await ctx.newPage();
page.on('pageerror', (e) => errores.push(`[pageerror] ${e.message}`));
page.on('console', (m) => { if (m.type() === 'error') errores.push(`[console] ${m.text()}`); });

await paso('Portada carga', async () => {
  await page.goto(BASE + '/', { waitUntil: 'networkidle' });
  await page.locator('h1', { hasText: 'Emacs no es un editor' }).waitFor({ timeout: 10000 });
  const mods = await page.locator('.toc-module').count();
  if (mods !== 14) throw new Error(`módulos en TOC: ${mods}`);
});

await paso('Lección y predicción', async () => {
  await page.goto(BASE + '/#/lesson/m00-l01', { waitUntil: 'networkidle' });
  await page.locator('h1', { hasText: 'Qué es Emacs' }).waitFor({ timeout: 10000 });
  await page.locator('.question-card .option').first().click();
  await page.locator('.explain.show').first().waitFor({ timeout: 5000 });
});

await paso('Marcar lección completada', async () => {
  await page.locator('[data-done="m00-l01"]').click();
  const done = await page.evaluate(() => JSON.parse(localStorage.getItem('emacs-desde-cero-v1') || '{}').completed?.['m00-l01']);
  if (!done) throw new Error('no se guardó el progreso');
});

await paso('Preguntas y filtros', async () => {
  await page.goto(BASE + '/#/questions', { waitUntil: 'networkidle' });
  await page.locator('h1', { hasText: 'Preguntas' }).waitFor({ timeout: 10000 });
  const rows = await page.locator('.q-row').count();
  if (rows < 100) throw new Error(`preguntas visibles: ${rows}`);
});

await paso('Retos', async () => {
  await page.goto(BASE + '/#/practice', { waitUntil: 'networkidle' });
  await page.locator('h1', { hasText: 'Retos' }).waitFor({ timeout: 10000 });
  const cards = await page.locator('.practice-card').count();
  if (cards !== 70) throw new Error(`retos: ${cards}`);
});

await paso('Quiz global', async () => {
  await page.goto(BASE + '/#/quiz/all', { waitUntil: 'networkidle' });
  await page.locator('.quiz-q').first().waitFor({ timeout: 10000 });
  const qs = await page.locator('.quiz-q').count();
  if (qs !== 8) throw new Error(`preguntas de quiz: ${qs}`);
});

await paso('Progreso', async () => {
  await page.goto(BASE + '/#/progress', { waitUntil: 'networkidle' });
  await page.locator('h1', { hasText: 'Tu progreso' }).waitFor({ timeout: 10000 });
  const stats = await page.locator('.stat').count();
  if (stats < 4) throw new Error(`stats: ${stats}`);
});

await paso('Tema claro/oscuro', async () => {
  await page.goto(BASE + '/#/', { waitUntil: 'networkidle' });
  await page.locator('#theme-btn').click();
  const theme = await page.evaluate(() => document.documentElement.dataset.theme);
  if (theme !== 'light') throw new Error(`tema: ${theme}`);
});

await page.screenshot({ path: join(RAIZ, '.capturas', 'emacs-home-1440.png'), fullPage: true });
await navegador.close();
servidor.kill();

console.log('\nErrores de página/consola:', errores.length);
errores.slice(0, 10).forEach((e) => console.log(' -', e));
console.log('Fallos:', fallos.length);
fallos.forEach((f) => console.log(' -', f));
if (fallos.length || errores.length) process.exit(1);
console.log('Emacs E2E: PASS');
process.exit(0);
