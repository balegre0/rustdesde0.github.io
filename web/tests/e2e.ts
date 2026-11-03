// Prueba E2E con el Edge instalado (playwright-core, channel "msedge") sobre dist/.
// Uso: bun run build && bun run test        (sirve dist/ en el puerto 4333)
// Los pasos que usan el Rust Playground (Reto, problema) aceptan dos resultados: la respuesta real
// del Playground o, sin red, el mensaje de error claro de la web.
import { chromium, type Page, type Locator } from 'playwright-core';
import { mkdirSync } from 'node:fs';
import { join } from 'node:path';

const RAIZ = join(import.meta.dir, '..');
const CAPTURAS = join(RAIZ, '.capturas');
mkdirSync(CAPTURAS, { recursive: true });
const PUERTO = 4333;
const BASE = `http://localhost:${PUERTO}`;

const servidor = Bun.spawn([process.execPath, join(RAIZ, 'scripts', 'servir.ts'), String(PUERTO)], { stdout: 'pipe', stderr: 'inherit' });
await Bun.sleep(700);

const errores: string[] = [];
const fallos: string[] = [];
const notas: string[] = [];
async function paso(nombre: string, fn: () => Promise<void>) {
  try { await fn(); console.log('  ✓ ' + nombre); }
  catch (e) { const m = (e as Error).message.split('\n')[0]; fallos.push(`${nombre}: ${m}`); console.log('  ✗ ' + nombre + ' — ' + m); }
}
const cap = (n: string) => join(CAPTURAS, n);

const navegador = await chromium.launch({ channel: 'msedge', headless: true });
const ctx = await navegador.newContext({ viewport: { width: 1440, height: 900 }, colorScheme: 'dark', acceptDownloads: true });
const vigilar = (p: Page) => {
  p.on('pageerror', (e) => errores.push(`[pageerror] ${p.url()} ${e.message}`));
  p.on('console', (m) => {
    if (m.type() !== 'error') return;
    const t = m.text();
    // Sin red, el fetch al Playground falla en la consola del navegador: es el caso que probamos a propósito.
    if (/play\.rust-lang\.org|ERR_INTERNET_DISCONNECTED|ERR_NAME_NOT_RESOLVED|Failed to fetch/.test(t)) { notas.push(t); return; }
    errores.push(`[console] ${p.url()} ${t}`);
  });
};
const page = await ctx.newPage();
vigilar(page);

const bloque = (p: Page, archivo: string) => p.locator('[data-ejecutable]', { has: p.locator('.title', { hasText: new RegExp(`^${archivo.replace('.', '\\.')}$`) }) }).first();
async function hidratada(loc: Locator) {
  await loc.scrollIntoViewIfNeeded();
  await loc.page().waitForFunction((el) => { const i = el?.closest('astro-island'); return !i || !i.hasAttribute('ssr'); }, await loc.elementHandle(), { timeout: 10000 });
}

console.log('Demo de componentes');
await page.goto(`${BASE}/demo/componentes/`);
await page.waitForLoadState('networkidle');

await paso('Ejecutar: hola.rs imprime su salida', async () => {
  const b = bloque(page, 'hola.rs');
  await b.locator('[data-accion="ejecutar"]').click();
  await b.locator('[data-texto]').filter({ hasText: 'Hola desde Rust' }).waitFor({ timeout: 10000 });
  await b.locator('[data-pie]').filter({ hasText: 'código de salida 0' }).waitFor({ timeout: 5000 });
});

await paso('Ejecutar con stdin interactivo: saludo.rs', async () => {
  const b = bloque(page, 'saludo.rs');
  await b.locator('[data-accion="ejecutar"]').click();
  const input = b.locator('[data-entrada] input');
  await input.waitFor({ state: 'visible', timeout: 10000 });
  await b.locator('[data-texto]').filter({ hasText: '¿Cómo te llamas?' }).waitFor({ timeout: 3000 });
  await input.fill('Ada'); await input.press('Enter');
  await b.locator('[data-texto]').filter({ hasText: '¿Cuántos años tienes?' }).waitFor({ timeout: 5000 });
  await input.waitFor({ state: 'visible', timeout: 5000 });
  await input.fill('36'); await input.press('Enter');
  await b.locator('[data-texto]').filter({ hasText: 'Hola, Ada. Dentro de 10 años tendrás 46.' }).waitFor({ timeout: 5000 });
  await b.screenshot({ path: cap('ejecutar-stdin-1440.png') });
});

await paso('Pánico por overflow en debug y vuelta en release', async () => {
  const b = bloque(page, 'overflow.rs');
  await b.locator('[data-accion="ejecutar"]').click();
  const pan = b.locator('[data-panico]:not([hidden])');
  await pan.waitFor({ timeout: 10000 });
  const t = await pan.innerText();
  if (!t.includes('Pánico:') || !t.includes('attempt to add with overflow')) throw new Error('aviso de pánico incompleto: ' + t);
  if (!/desbordamiento/i.test(t)) throw new Error('falta la explicación en español: ' + t);
  await b.screenshot({ path: cap('panico-debug-1440.png') });
  await b.getByRole('radio', { name: 'release' }).click();
  await b.locator('[data-accion="ejecutar"]').click();
  await b.locator('[data-texto]').filter({ hasText: 'nivel = 0' }).waitFor({ timeout: 10000 });
  if (await b.locator('[data-panico]:not([hidden])').count()) throw new Error('en release no debería haber pánico');
  await b.screenshot({ path: cap('overflow-release-1440.png') });
});

await paso('Pánico por índice fuera de rango', async () => {
  const b = bloque(page, 'indice.rs');
  await b.locator('[data-accion="ejecutar"]').click();
  await b.locator('[data-panico]:not([hidden])', { hasText: 'index out of bounds: the len is 3 but the index is 10' }).waitFor({ timeout: 10000 });
});

await paso('NoCompila: error real E0499 y «Ver arreglo»', async () => {
  const n = page.locator('[data-nocompila]').first();
  await n.scrollIntoViewIfNeeded();
  const d = await n.locator('[data-diagnostico]').innerText();
  for (const x of ['error[E0499]', 'cannot borrow `lista` as mutable more than once at a time', 'src/main.rs:4:13', 'second mutable borrow occurs here'])
    if (!d.includes(x)) throw new Error('falta en el diagnóstico: ' + x);
  const enlace = await n.locator('.rdc-diagnostico-enlace').getAttribute('href');
  if (!/e0499/i.test(enlace ?? '')) throw new Error('enlace al error incorrecto: ' + enlace);
  await n.screenshot({ path: cap('nocompila-1440.png') });
  await n.locator('summary', { hasText: 'Ver arreglo' }).click();
  const a = n.locator('[data-ejecutable]');
  await a.locator('[data-accion="ejecutar"]').click();
  await a.locator('[data-texto]').filter({ hasText: '[1, 2, 3, 4, 5]' }).waitFor({ timeout: 10000 });
});

await paso('Prestamos: tabla con permisos perdidos tachados', async () => {
  const t = page.locator('.rdc-prestamos').first();
  await t.scrollIntoViewIfNeeded();
  if ((await t.locator('del.rdc-perm').count()) < 3) throw new Error('no hay permisos perdidos tachados');
  const sr = await t.locator('tbody tr').nth(1).innerText();
  if (!sr.includes('pierde')) throw new Error('falta la descripción accesible: ' + sr);
  await t.screenshot({ path: cap('prestamos-1440.png') });
});

await paso('Predice: bloqueado hasta responder, luego ejecuta', async () => {
  const b = page.locator('.rdc-predice [data-ejecutable]').first();
  const btn = b.locator('[data-accion="ejecutar"]');
  if (!(await btn.isDisabled())) throw new Error('Ejecutar debería estar bloqueado');
  await hidratada(b.locator('.rdc-quiz'));
  await b.locator('.rdc-opcion[data-opcion=\"1\"]').click();
  await b.locator('.rdc-feedback-ok').waitFor({ timeout: 5000 });
  await btn.click();
  await b.locator('[data-texto]').filter({ hasText: '3' }).waitFor({ timeout: 10000 });
});

await paso('Pregunta: fallo con porqué y acierto con teclado', async () => {
  const q = page.locator('.rdc-quiz', { hasText: 'si los dos son' }).first();
  await hidratada(q);
  await q.locator('.rdc-opcion[data-opcion=\"0\"]').click();
  await q.locator('.rdc-porque').waitFor({ timeout: 3000 });
  await q.locator('.rdc-opcion[data-opcion=\"1\"]').focus();
  await page.keyboard.press('Enter');
  await q.locator('.rdc-feedback-ok').waitFor({ timeout: 3000 });
  await q.screenshot({ path: cap('pregunta-1440.png') });
});

await paso('Respuesta escrita', async () => {
  const q = page.locator('.rdc-quiz', { hasText: '17 % 5' }).first();
  await hidratada(q);
  await q.locator('input').fill('  2 ');
  await q.locator('input').press('Enter');
  await q.locator('.rdc-feedback-ok').waitFor({ timeout: 3000 });
});

await paso('Ordena (Parsons)', async () => {
  const q = page.locator('.rdc-ordena').first();
  await hidratada(q);
  for (const l of ['fn main() {', 'let x = 5;', 'println!("{x}");', '}']) await q.locator('.rdc-linea', { hasText: l }).first().click();
  await q.getByRole('button', { name: 'Comprobar' }).click();
  await q.locator('.rdc-feedback-ok').waitFor({ timeout: 3000 });
});

await paso('Bits: formar 200', async () => {
  const q = page.locator('.rdc-bits').first();
  await hidratada(q);
  for (const n of ['vale 128', 'vale 64', 'vale 8']) await q.getByRole('button', { name: n }).click();
  await q.locator('.rdc-feedback-ok').waitFor({ timeout: 3000 });
  const t = await q.locator('.rdc-bits-lectura').innerText();
  if (!t.includes('0xC8') || !t.includes('-56')) throw new Error('lectura incorrecta: ' + t);
});

await paso('Reto en el Playground: respuesta real o error claro', async () => {
  const r = page.locator('[data-reto]').first();
  await r.scrollIntoViewIfNeeded();
  await r.locator('.cm-content').waitFor({ timeout: 15000 });
  if (!(await r.innerText()).includes('play.rust-lang.org')) throw new Error('falta el aviso de envío al Playground');
  await r.locator('.cm-content').click();
  await page.keyboard.press('Control+A');
  await page.keyboard.insertText('fn doble(n: i32) -> i32 {\n    n * 2\n}\n\nfn main() {\n    println!("{}", doble(21));\n    println!("{}", doble(-4));\n}\n');
  await r.locator('[data-accion="comprobar"]').click();
  const ok = r.locator('.rdc-veredicto.rdc-feedback-ok');
  const err = r.locator('.rdc-compilacion.error', { hasText: 'No se pudo usar el Rust Playground' });
  await Promise.race([ok.waitFor({ timeout: 45000 }), err.waitFor({ timeout: 45000 })]);
  notas.push(await ok.count() ? 'Reto: superado en el Playground real' : 'Reto: sin Playground → ' + (await err.innerText()));
  await r.screenshot({ path: cap('reto-1440.png') });
});

await page.screenshot({ path: cap('demo-1440.png'), fullPage: true });

console.log('Práctica');
await paso('Preguntas: tabla filtrable', async () => {
  await page.goto(`${BASE}/preguntas/`);
  const total = Number(await page.locator('[data-cuenta]').innerText());
  await page.selectOption('select[name="tema"]', 'prestamos');
  const n = Number(await page.locator('[data-cuenta]').innerText());
  if (!(n > 0 && n < total)) throw new Error(`filtro por tema no funciona (${n} de ${total})`);
  if (!page.url().includes('tema=prestamos')) throw new Error('el filtro no se refleja en la URL');
  await page.fill('input[name="q"]', 'E0499');
  await page.locator('tbody tr:not([hidden])', { hasText: 'Dos préstamos exclusivos' }).first().waitFor({ timeout: 3000 });
  await page.screenshot({ path: cap('preguntas-1440.png') });
});

await paso('Pregunta de la biblioteca: responder, solución y estadística', async () => {
  await page.goto(`${BASE}/preguntas/demo-dos-mut/`);
  const q = page.locator('.rdc-quiz').first();
  await hidratada(q);
  await q.locator('.rdc-opcion[data-opcion=\"0\"]').click();
  const sol = page.locator('[data-solucion-pregunta]:not([hidden])');
  await sol.waitFor({ timeout: 3000 });
  if (!(await sol.locator('[data-diagnostico]').innerText()).includes('error[E0499]')) throw new Error('la solución no muestra el error real');
  await q.locator('.rdc-opcion[data-opcion=\"1\"]').click();
  await q.locator('.rdc-feedback-ok').waitFor({ timeout: 3000 });
  await page.locator('[data-mi-estado]', { hasText: 'Fallada a la primera' }).waitFor({ timeout: 3000 });
  await page.screenshot({ path: cap('pregunta-biblioteca-1440.png'), fullPage: true });
  await page.goto(`${BASE}/preguntas/?estado=mal`);
  await page.locator('tbody tr:not([hidden]) .rdc-estado[data-estado="mal"]').first().waitFor({ timeout: 3000 });
});

await paso('Problema: editor + pruebas en el Playground (o error claro)', async () => {
  await page.goto(`${BASE}/problemas/demo-suma-pares/`);
  const p = page.locator('[data-problema]');
  await p.locator('.cm-content').waitFor({ timeout: 15000 });
  await p.locator('.cm-content').click();
  await page.keyboard.press('Control+A');
  await page.keyboard.insertText('fn suma_pares(v: &[i32]) -> i32 {\n    v.iter().filter(|n| *n % 2 == 0).sum()\n}\n');
  await p.locator('[data-accion="ejecutar"]').click();
  const res = p.locator('[data-resultado]');
  await page.waitForFunction(() => { const r = document.querySelector<HTMLElement>('[data-resultado]'); return r && !r.hidden && r.dataset.estado !== 'red' || /No se pudo/.test(r?.textContent ?? ''); }, null, { timeout: 45000 });
  const t = await res.innerText();
  notas.push(/Todos los casos superados/.test(t) ? 'Problema: superado en el Playground real' : 'Problema: ' + t.split('\n').slice(0, 2).join(' '));
  if (!/Todos los casos superados|No se pudo/.test(t)) throw new Error('resultado inesperado: ' + t);
  await page.screenshot({ path: cap('problema-1440.png') });
});

await paso('Quiz cronometrado: completar y guardar marca', async () => {
  await page.goto(`${BASE}/quizzes/demo-relampago/`);
  const z = page.locator('[data-quiz]');
  await hidratada(z);
  await z.getByRole('button', { name: 'Empezar' }).click();
  for (let i = 0; i < 4; i++) {
    const input = z.locator('input.rdc-respuesta-input');
    if (await input.count()) { await input.fill('255'); await input.press('Enter'); }
    else await z.locator('.rdc-opcion').first().click();
    await z.getByRole('button', { name: /Siguiente|Ver resultado/ }).click();
  }
  await page.locator('[data-quiz][data-fin]').waitFor({ timeout: 3000 });
  const m = await page.evaluate(() => JSON.parse(localStorage.getItem('rdc.progreso.v1') || '{}').marcas?.['quiz:demo-relampago']);
  if (!m) throw new Error('no se guardó la marca');
  await page.screenshot({ path: cap('quiz-1440.png') });
});

await paso('Juego «¿Compila? 60 s» (con reloj simulado)', async () => {
  const p2 = await ctx.newPage();
  vigilar(p2);
  await p2.clock.install();
  await p2.goto(`${BASE}/juegos/compila/`);
  const z = p2.locator('[data-juego]');
  await hidratada(z);
  await z.getByRole('button', { name: 'Empezar' }).click();
  await p2.locator('[data-codigo-juego]').waitFor({ timeout: 3000 });
  await p2.screenshot({ path: cap('juego-compila-1440.png') });
  for (let i = 0; i < 3; i++) { await p2.keyboard.press('c'); await p2.clock.runFor(2000); }
  await p2.clock.runFor(61000);
  await p2.locator('[data-juego][data-fin]').waitFor({ timeout: 5000 });
  const m = await p2.evaluate(() => JSON.parse(localStorage.getItem('rdc.progreso.v1') || '{}').marcas?.['juego:compila']);
  if (!m) throw new Error('no se guardó la marca del juego');
  await p2.close();
});

console.log('Lección, progreso, repaso, árbol y panel');
const indice = await (await fetch(`${BASE}/datos/lecciones.json`)).json();
const todas = indice.mundos.flatMap((m: any) => m.lecciones);
const primera = todas.find((l: any) => l.tarjetas.length > 0) ?? todas[0];

await paso(`Marcar como completada (${primera?.id})`, async () => {
  if (!primera) throw new Error('no hay lecciones en el índice');
  await page.goto(BASE + primera.href);
  await page.waitForLoadState('networkidle');
  await page.screenshot({ path: cap('leccion-1440.png') });
  const fin = page.locator('[data-fin-leccion]');
  await fin.locator('[data-marcar]').click();
  await fin.locator('[data-texto-hecho]:not([hidden])').waitFor({ timeout: 3000 });
  await fin.screenshot({ path: cap('fin-leccion-1440.png') });
});

await paso('Portada: «Continuar», racha y tarjetas para hoy', async () => {
  await page.goto(BASE + '/');
  await page.locator('[data-continuar]:not([hidden])').waitFor({ timeout: 3000 });
  await page.locator('[data-racha]', { hasText: 'Racha: 1 día' }).waitFor({ timeout: 3000 });
  if (primera?.tarjetas?.length) await page.locator('[data-repaso]:not([hidden])', { hasText: 'para hoy' }).waitFor({ timeout: 5000 });
  await page.screenshot({ path: cap('inicio-1440.png'), fullPage: true });
});

await paso('Repaso: mostrar respuesta y valorar', async () => {
  await page.goto(BASE + '/repaso/');
  const mostrar = page.getByRole('button', { name: 'Mostrar respuesta' });
  await mostrar.waitFor({ timeout: 5000 });
  await page.screenshot({ path: cap('repaso-1440.png') });
  await mostrar.click();
  await page.getByRole('button', { name: 'Lo sabía' }).click();
  const p = await page.evaluate(() => JSON.parse(localStorage.getItem('rdc.progreso.v1') || '{}'));
  if (!Object.keys(p.tarjetas ?? {}).length) throw new Error('no se guardó el estado FSRS');
});

await paso('Árbol: la lección completada aparece hecha y se navega con flechas', async () => {
  await page.goto(BASE + '/arbol/');
  await page.locator(`[data-leccion="${primera.id}"][data-estado="hecha"]`).waitFor({ timeout: 3000 });
  await page.locator(`[data-leccion="${primera.id}"]`).focus();
  await page.keyboard.press('ArrowRight');
  const foco = await page.evaluate(() => (document.activeElement as HTMLElement)?.dataset.leccion);
  if (!foco || foco === primera.id) throw new Error('las flechas no mueven el foco');
  await page.screenshot({ path: cap('arbol-1440.png') });
});

await paso('Panel: racha, preguntas y marcas', async () => {
  await page.goto(BASE + '/panel/');
  const t = await page.locator('.rdc-panel').innerText();
  if (!/Racha\s*1/.test(t.replace(/\n/g, ' '))) throw new Error('racha incorrecta: ' + t.slice(0, 120));
  if (!t.includes('Préstamos')) throw new Error('falta el acierto por tema');
  await page.screenshot({ path: cap('panel-1440.png'), fullPage: true });
});

await paso('Progreso: exportación', async () => {
  await page.goto(BASE + '/progreso/');
  const [descarga] = await Promise.all([page.waitForEvent('download'), page.getByRole('button', { name: /Exportar/ }).click()]);
  const ruta = cap('progreso-exportado.json');
  await descarga.saveAs(ruta);
  const j = JSON.parse(await Bun.file(ruta).text());
  if (j.v !== 1 || !j.preguntas) throw new Error('exportación incompleta');
});

await paso('Búsqueda con la tecla S', async () => {
  await page.goto(BASE + '/demo/componentes/');
  await page.keyboard.press('s');
  await page.locator('dialog[open]').waitFor({ timeout: 3000 });
  await page.keyboard.press('Escape');
});

await paso('Capturas en modo claro', async () => {
  const claro = await navegador.newContext({ viewport: { width: 1440, height: 900 }, colorScheme: 'light' });
  const p = await claro.newPage();
  vigilar(p);
  for (const [ruta, nombre] of [['/', 'inicio'], ['/demo/componentes/', 'demo'], ['/preguntas/', 'preguntas'], ['/problemas/demo-suma-pares/', 'problema']] as const) {
    await p.goto(BASE + ruta);
    await p.waitForLoadState('networkidle');
    await p.screenshot({ path: cap(`${nombre}-claro-1440.png`) });
  }
  await claro.close();
});

await paso('Capturas a 390 px sin desbordamiento horizontal', async () => {
  const movil = await navegador.newContext({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 2, isMobile: true, hasTouch: true, colorScheme: 'dark' });
  const p = await movil.newPage();
  vigilar(p);
  for (const [ruta, nombre] of [['/', 'inicio'], ['/demo/componentes/', 'demo'], ['/preguntas/', 'preguntas'], ['/preguntas/demo-dos-mut/', 'pregunta'], ['/problemas/demo-suma-pares/', 'problema'], ['/juegos/compila/', 'juego'], ['/arbol/', 'arbol'], ['/panel/', 'panel'], [primera?.href ?? '/curso/', 'leccion'], ['/repaso/', 'repaso']] as const) {
    await p.goto(BASE + ruta);
    await p.waitForLoadState('networkidle');
    await p.screenshot({ path: cap(`${nombre}-390.png`), fullPage: nombre !== 'demo' && nombre !== 'preguntas' });
    const ancho = await p.evaluate(() => document.documentElement.scrollWidth);
    if (ancho > 392) throw new Error(`${ruta}: desbordamiento horizontal (${ancho}px)`);
  }
  await p.goto(BASE + '/demo/componentes/');
  const b = bloque(p, 'hola.rs');
  await b.locator('[data-accion="ejecutar"]').tap();
  await b.locator('[data-texto]').filter({ hasText: 'Hola desde Rust' }).waitFor({ timeout: 10000 });
  await b.screenshot({ path: cap('ejecutar-390.png') });
  await movil.close();
});

await navegador.close();
servidor.kill();

for (const n of notas) console.log('  · ' + n);
console.log(`\nErrores de consola: ${errores.length}`);
for (const e of errores) console.log('  ' + e);
console.log(fallos.length ? `\n${fallos.length} paso(s) fallaron:\n  ${fallos.join('\n  ')}` : '\nTodo OK.');
process.exit(fallos.length || errores.length ? 1 : 0);
