import { modules, lessons, glossary } from './data/index.js';

/* ------------------------------------------------------------------ state */
const STORAGE = 'emacs-desde-cero-v1';
const defaultState = () => ({
  completed: {}, q: {}, ex: {}, cards: {}, theme: null,
  streak: { last: null, count: 0 },
  created: Date.now(),
});
let state = load();
let quizTimer = null;

function load() {
  try {
    const raw = localStorage.getItem(STORAGE);
    if (!raw) return defaultState();
    const parsed = JSON.parse(raw);
    return { ...defaultState(), ...parsed, streak: { ...defaultState().streak, ...(parsed.streak || {}) } };
  } catch { return defaultState(); }
}
function save() {
  try { localStorage.setItem(STORAGE, JSON.stringify(state)); } catch {}
}
function today() { return new Date().toISOString().slice(0, 10); }
function touchStreak() {
  const t = today();
  if (state.streak.last === t) return;
  const yesterday = new Date(Date.now() - 86400000).toISOString().slice(0, 10);
  state.streak.count = state.streak.last === yesterday ? state.streak.count + 1 : 1;
  state.streak.last = t;
  save();
}

/* ------------------------------------------------------------------ helpers */
const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];
const esc = (s) => String(s ?? '').replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));
const md = (s) => esc(s)
  .replace(/`([^`]+)`/g, '<code>$1</code>')
  .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
  .replace(/\[([^\]]+)\]\((https?:[^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
const lessonList = Object.values(lessons);
const totalLessons = lessonList.length;
const moduleById = Object.fromEntries(modules.map((m) => [m.id, m]));
const lessonById = Object.fromEntries(lessonList.map((l) => [l.id, l]));

function moduleLessons(m) { return m.lessons.map((id) => lessonById[id]).filter(Boolean); }
function moduleDone(m) { return moduleLessons(m).filter((l) => state.completed[l.id]).length; }
function courseDone() { return lessonList.filter((l) => state.completed[l.id]).length; }
function pct(n, d) { return d ? Math.round((n / d) * 100) : 0; }

const bank = lessonList.flatMap((l) => (l.q || []).map((q, i) => ({
  id: `${l.id}-q${i}`, lesson: l.id, lessonTitle: l.title, module: l.module,
  q: q.q, o: q.o, a: q.a, why: q.why, tags: q.tags || l.tags || [],
})));
const exercises = lessonList.filter((l) => l.ex).map((l) => ({
  id: `${l.id}-ex`, lesson: l.id, lessonTitle: l.title, module: l.module,
  ...l.ex, tags: l.tags || [],
}));

function moduleName(id) { return moduleById[id]?.title || id; }
function levelClass(level) {
  return level === 'Experto' ? 'pill-amber' : level === 'Avanzado' ? 'pill-violet' : 'pill-good';
}
function toast(msg) {
  const t = $('#toast');
  t.textContent = msg;
  t.classList.add('show');
  clearTimeout(toast._t);
  toast._t = setTimeout(() => t.classList.remove('show'), 2200);
}
function copyText(text) {
  if (navigator.clipboard?.writeText) return navigator.clipboard.writeText(text);
  const ta = document.createElement('textarea');
  ta.value = text; document.body.appendChild(ta); ta.select();
  try { document.execCommand('copy'); } catch {}
  document.body.removeChild(ta);
  return Promise.resolve();
}
function codeBlock(code, lang = 'elisp', caption = '') {
  return `<figure class="code-block">
    <pre><code data-lang="${esc(lang)}">${esc(code)}</code></pre>
    <button class="copy-btn" type="button" data-copy="${esc(code)}">Copiar</button>
    ${caption ? `<figcaption>${md(caption)}</figcaption>` : ''}
  </figure>`;
}
function qCard(q, idx, scope = 'lesson') {
  const opts = q.o.map((o, i) => `<label class="option" data-q="${esc(q.id)}" data-i="${i}">
      <input type="radio" name="${esc(q.id)}" value="${i}">
      <span>${md(o)}</span>
    </label>`).join('');
  return `<div class="question-card" data-question="${esc(q.id)}">
    <p class="q">${idx ? `${idx}. ` : ''}${md(q.q)}</p>
    <div class="options">${opts}</div>
    <div class="explain" id="ex-${esc(q.id)}">${md(q.why || '')}</div>
  </div>`;
}

/* ------------------------------------------------------------------ theme */
function applyTheme() {
  const stored = state.theme || localStorage.getItem('emacs-theme');
  if (stored === 'light' || stored === 'dark') document.documentElement.dataset.theme = stored;
  const btn = $('#theme-btn');
  if (btn) btn.textContent = document.documentElement.dataset.theme === 'light' ? '☾' : '☀';
}
$('#theme-btn')?.addEventListener('click', () => {
  const next = document.documentElement.dataset.theme === 'light' ? 'dark' : 'light';
  document.documentElement.dataset.theme = next;
  state.theme = next; localStorage.setItem('emacs-theme', next); save(); applyTheme();
});
applyTheme();

/* ------------------------------------------------------------------ toc */
function renderTOC(activeLesson) {
  const activeModule = activeLesson?.module;
  $('#toc').innerHTML = modules.map((m) => {
    const done = moduleDone(m);
    const open = m.id === activeModule;
    return `<div class="toc-module ${open ? 'open' : ''}" data-module="${m.id}">
      <button type="button" aria-expanded="${open}">
        <span class="num">${String(m.num).padStart(2, '0')}</span>
        <span>${esc(m.title)}</span>
        <span class="pill ${done === m.lessons.length ? 'pill-good' : ''}" style="font-size:.6rem">${done}/${m.lessons.length}</span>
        <span class="arrow">▶</span>
      </button>
      <ul class="toc-lessons">
        ${moduleLessons(m).map((l) => `<li><a href="#/lesson/${l.id}" class="${l.id === activeLesson?.id ? 'active' : ''}">
          ${esc(l.title)} ${state.completed[l.id] ? '<span class="done">✓</span>' : ''}
        </a></li>`).join('')}
      </ul>
    </div>`;
  }).join('');
}
$('#toc')?.addEventListener('click', (e) => {
  const btn = e.target.closest('.toc-module > button');
  if (!btn) return;
  const mod = btn.parentElement;
  mod.classList.toggle('open');
  btn.setAttribute('aria-expanded', mod.classList.contains('open'));
});

/* ------------------------------------------------------------------ router */
const main = $('#main');
function route() {
  if (quizTimer) clearInterval(quizTimer);
  const hash = location.hash.replace(/^#/, '') || '/';
  const [path, query] = hash.split('?');
  const parts = path.split('/').filter(Boolean);
  const params = new URLSearchParams(query || '');
  window.scrollTo({ top: 0, behavior: 'instant' });
  if (parts[0] === 'lesson') return renderLesson(parts[1]);
  if (parts[0] === 'module') return renderModule(parts[1]);
  if (parts[0] === 'questions') return renderQuestions(params);
  if (parts[0] === 'practice') return renderPractice(params);
  if (parts[0] === 'quiz') return parts[1] ? renderQuiz(parts[1]) : renderQuizHome();
  if (parts[0] === 'review') return renderReview();
  if (parts[0] === 'progress') return renderProgress();
  if (parts[0] === 'list') return renderList(parts[1]);
  if (parts[0] === 'glossary') return renderGlossary();
  if (parts[0] === 'about') return renderAbout();
  if (parts[0] === 'search') return renderSearch(params.get('q') || '');
  return renderHome();
}
window.addEventListener('hashchange', route);
window.addEventListener('DOMContentLoaded', route);
if (document.readyState !== 'loading') route();

function setNav(name) {
  $$('.main-nav a').forEach((a) => a.classList.toggle('active', a.dataset.nav === name));
  $('#sidebar').classList.remove('open');
  $('#menu-btn')?.setAttribute('aria-expanded', 'false');
}
function updateRing() {
  const p = pct(courseDone(), totalLessons);
  const fg = $('#ring-fg');
  if (fg) fg.style.strokeDashoffset = String(100.5 - (100.5 * p) / 100);
  $('#ring-label').textContent = `${p}%`;
}

/* ------------------------------------------------------------------ home */
function renderHome() {
  setNav('home');
  renderTOC(null);
  updateRing();
  const done = courseDone();
  const next = lessonList.find((l) => !state.completed[l.id]) || lessonList[0];
  main.innerHTML = `
    <section class="hero">
      <p class="eyebrow">Curso avanzado · español</p>
      <h1>Emacs no es un editor. <span class="grad-text">Es un entorno</span> que puedes programar.</h1>
      <p class="lede">14 módulos, ${totalLessons} lecciones y ${bank.length} preguntas para pasar de “abro Emacs y me pierdo” a escribir tus propios comandos, modos y paquetes. Sin cuentas: tu progreso vive en este navegador.</p>
      <div class="hero-actions">
        <a class="btn" href="#/lesson/${next.id}">${done ? 'Continuar' : 'Empezar'} por ${esc(next.title)} →</a>
        <a class="btn btn-ghost" href="#/questions">Practicar preguntas</a>
        <a class="btn btn-ghost" href="#/quiz">Hacer un quiz</a>
      </div>
    </section>

    <section class="stats">
      <div class="stat"><strong>${modules.length}</strong><span>módulos</span></div>
      <div class="stat"><strong>${totalLessons}</strong><span>lecciones</span></div>
      <div class="stat"><strong>${bank.length}</strong><span>preguntas</span></div>
      <div class="stat"><strong>${exercises.length}</strong><span>retos</span></div>
    </section>

    <section class="card" style="margin-bottom:1.2rem">
      <div class="meta-row"><span class="pill pill-violet">Tu progreso</span><span>${done}/${totalLessons} lecciones</span><span>${pct(done, totalLessons)}%</span></div>
      <div class="bar"><span style="width:${pct(done, totalLessons)}%"></span></div>
      <p class="small muted" style="margin:.6rem 0 0">Racha: ${state.streak.count} día(s). ${done ? 'Sigue donde lo dejaste.' : 'Empieza por el módulo 00.'}</p>
    </section>

    <section>
      <div class="page-head"><div><p class="eyebrow">Temario</p><h2>Los 14 módulos</h2></div><a class="btn btn-ghost btn-sm" href="#/glossary">Glosario</a></div>
      <div class="module-grid">
        ${modules.map((m) => {
          const total = m.lessons.length, d = moduleDone(m);
          return `<article class="module-card">
            <div class="meta-row"><span class="pill">Módulo ${String(m.num).padStart(2, '0')}</span><span>${total} lecciones</span></div>
            <h3><a href="#/module/${m.id}">${esc(m.title)}</a></h3>
            <p>${md(m.subtitle)}</p>
            <div class="bar"><span style="width:${pct(d, total)}%"></span></div>
            <p class="small muted" style="margin:.45rem 0 0">${d}/${total} · ${pct(d, total)}%</p>
          </article>`;
        }).join('')}
      </div>
    </section>

    <section class="card" style="margin-top:1.4rem">
      <p class="eyebrow">Cómo está pensado</p>
      <h2>Predice, entiende, ejecuta, explica</h2>
      <p class="muted">Cada lección tiene un reto de predicción, una explicación corta, código que puedes copiar a Emacs, una sección “bajo el capó”, una trampa típica, preguntas y un ejercicio. No se trata de memorizar teclas: se trata de entender el modelo de Emacs para que puedas deducir el resto.</p>
      <p class="small muted" style="margin-bottom:0">Este curso no ejecuta Elisp en el navegador. Los ejemplos se copian a Emacs; los quizzes y preguntas sí se corrigen aquí.</p>
    </section>`;
}

/* ------------------------------------------------------------------ module */
function renderModule(id) {
  const m = moduleById[id];
  if (!m) return renderHome();
  setNav('home'); renderTOC(null); updateRing();
  const ls = moduleLessons(m);
  main.innerHTML = `<section>
    <p class="eyebrow">Módulo ${String(m.num).padStart(2, '0')}</p>
    <h1>${esc(m.title)}</h1>
    <p class="lede">${md(m.subtitle)}</p>
    <div class="meta-row" style="margin:1rem 0"><span>${ls.length} lecciones</span><span>${moduleDone(m)} completadas</span><span>${pct(moduleDone(m), ls.length)}%</span></div>
    <div class="bar" style="margin-bottom:1.2rem"><span style="width:${pct(moduleDone(m), ls.length)}%"></span></div>
    <div class="card">
      <ol class="clean" style="padding-left:1.1rem">
        ${ls.map((l) => `<li style="margin-bottom:.55rem"><a href="#/lesson/${l.id}">${esc(l.title)}</a> <span class="pill">${l.min} min</span> ${state.completed[l.id] ? '<span class="pill pill-good">hecha</span>' : ''}<br><span class="small muted">${md(l.obj)}</span></li>`).join('')}
      </ol>
    </div>
    <div class="hero-actions" style="margin-top:1.2rem">
      <a class="btn" href="#/lesson/${ls[0].id}">Empezar módulo →</a>
      <a class="btn btn-ghost" href="#/quiz/${m.id}">Quiz del módulo</a>
    </div>
  </section>`;
}

/* ------------------------------------------------------------------ lesson */
function renderLesson(id) {
  const l = lessonById[id];
  if (!l) return renderHome();
  setNav('home'); renderTOC(l); updateRing();
  const m = moduleById[l.module];
  const predict = l.q[0];
  const checks = l.q.slice(1);
  const prev = lessonList[lessonList.indexOf(l) - 1];
  const next = lessonList[lessonList.indexOf(l) + 1];
  main.innerHTML = `<article class="lesson">
    <div class="breadcrumb"><a href="#/">Inicio</a><span>›</span><a href="#/module/${m.id}">${esc(m.title)}</a><span>›</span><span>${esc(l.title)}</span></div>
    <header class="lesson-head">
      <p class="eyebrow">Lección ${String(l.n).padStart(2, '0')} · Módulo ${String(m.num).padStart(2, '0')}</p>
      <h1>${esc(l.title)}</h1>
      <p class="lede">${md(l.obj)}</p>
      <div class="lesson-meta"><span>${l.min} min</span><span>·</span><span>${esc(l.level)}</span><span>·</span><span>${(l.tags || []).slice(0, 4).map(esc).join(' · ')}</span></div>
    </header>

    <p class="hook">${md(l.frase)}</p>

    <section class="section">
      <h2>Predice <span class="tag">antes de leer</span></h2>
      ${qCard({ ...predict, id: `${l.id}-q0` }, 0)}
    </section>

    <section class="section">
      <h2>La idea</h2>
      ${l.idea.map((p) => `<p>${md(p)}</p>`).join('')}
    </section>

    <section class="section">
      <h2>Pruébalo</h2>
      ${codeBlock(l.code.src, l.code.lang, l.code.cap)}
      <p class="small muted">Copia el bloque en Emacs: <kbd>M-:</kbd> para evaluar una expresión, o <kbd>C-x C-e</kbd> al final de una línea en un buffer Elisp. También puedes usar el buffer <code>*scratch*</code>.</p>
    </section>

    <section class="section under">
      <h3>Bajo el capó</h3>
      ${l.under.map((p) => `<p>${md(p)}</p>`).join('')}
    </section>

    <section class="section">
      <h2>Trampa típica</h2>
      <div class="trap">
        <p class="bad"><strong>❌ Mal:</strong> ${md(l.trap.bad)}</p>
        <p class="good"><strong>✅ Bien:</strong> ${md(l.trap.good)}</p>
        <p class="small muted">${md(l.trap.why)}</p>
      </div>
    </section>

    <section class="section">
      <h2>Comprueba</h2>
      ${checks.map((q, i) => qCard({ ...q, id: `${l.id}-q${i + 1}` }, i + 1)).join('')}
    </section>

    <section class="section">
      <h2>Tu turno</h2>
      <div class="exercise">
        <p><strong>Reto:</strong> ${md(l.ex.p)}</p>
        ${l.ex.start ? codeBlock(l.ex.start, 'elisp', 'Punto de partida') : ''}
        <p class="small muted">Pista: ${md(l.ex.hint)}</p>
        <button class="btn btn-ghost btn-sm" type="button" data-reveal="sol-${l.id}">Ver solución</button>
        <div class="solution" id="sol-${l.id}">${codeBlock(l.ex.sol, 'elisp', 'Una solución posible')}</div>
        <p style="margin:.8rem 0 0"><button class="btn btn-sm ${state.ex[`${l.id}-ex`] ? 'done-btn is-done' : ''}" type="button" data-exdone="${l.id}-ex">${state.ex[`${l.id}-ex`] ? '✓ Reto completado' : 'Marcar reto como hecho'}</button></p>
      </div>
    </section>

    <section class="section">
      <h2>Repaso</h2>
      ${(l.cards || []).map((c, i) => `<div class="flashcard" data-card="${l.id}-c${i}">
        <p class="front">${md(c[0])}</p>
        <p class="back">${md(c[1])}</p>
        <button class="btn btn-ghost btn-sm" type="button" data-flip>Mostrar respuesta</button>
      </div>`).join('') || '<p class="muted">Sin tarjetas en esta lección.</p>'}
    </section>

    <div class="lesson-actions">
      <button class="btn ${state.completed[l.id] ? 'done-btn is-done' : ''}" type="button" data-done="${l.id}">${state.completed[l.id] ? '✓ Lección completada' : 'Marcar lección como completada'}</button>
      <a class="btn btn-ghost" href="#/questions?m=${l.module}">Practicar preguntas del módulo</a>
    </div>
    <div class="pager">
      ${prev ? `<a href="#/lesson/${prev.id}">← ${esc(prev.title)}</a>` : '<span></span>'}
      ${next ? `<a href="#/lesson/${next.id}">${esc(next.title)} →</a>` : '<a href="#/progress">Ver progreso →</a>'}
    </div>
  </article>`;
  bindLesson(l);
}

function bindLesson(l) {
  // copy buttons
  $$('[data-copy]').forEach((b) => b.addEventListener('click', () => {
    copyText(b.dataset.copy).then(() => { b.textContent = 'Copiado'; setTimeout(() => (b.textContent = 'Copiar'), 1300); });
  }));
  // options
  $$('.option').forEach((opt) => opt.addEventListener('click', () => {
    const qid = opt.dataset.q;
    const selected = Number(opt.dataset.i);
    const question = bank.find((q) => q.id === qid) || l.q.find((_, i) => `${l.id}-q${i}` === qid);
    if (!question) return;
    const correct = question.a;
    const card = opt.closest('.question-card');
    card.querySelectorAll('.option').forEach((o) => {
      const i = Number(o.dataset.i);
      o.classList.toggle('correct', i === correct);
      o.classList.toggle('wrong', i === selected && i !== correct);
    });
    const exp = card.querySelector('.explain');
    if (exp) { exp.classList.add('show'); }
    const prev = state.q[qid];
    if (!prev) state.q[qid] = { seen: 1, correct: selected === correct ? 1 : 0 };
    if (selected === correct) touchStreak();
    save(); updateRing();
  }));
  // solution reveal
  $$('[data-reveal]').forEach((b) => b.addEventListener('click', () => {
    const el = document.getElementById(b.dataset.reveal);
    if (el) el.classList.toggle('show');
    b.textContent = el?.classList.contains('show') ? 'Ocultar solución' : 'Ver solución';
  }));
  // exercise done
  $$('[data-exdone]').forEach((b) => b.addEventListener('click', () => {
    const id = b.dataset.exdone;
    state.ex[id] = !state.ex[id];
    b.classList.toggle('is-done', state.ex[id]);
    b.textContent = state.ex[id] ? '✓ Reto completado' : 'Marcar reto como hecho';
    save(); toast(state.ex[id] ? 'Reto completado' : 'Reto desmarcado');
  }));
  // lesson done
  const doneBtn = $('[data-done]');
  if (doneBtn) doneBtn.addEventListener('click', () => {
    const id = doneBtn.dataset.done;
    state.completed[id] = !state.completed[id];
    doneBtn.classList.toggle('is-done', state.completed[id]);
    doneBtn.textContent = state.completed[id] ? '✓ Lección completada' : 'Marcar lección como completada';
    if (state.completed[id]) touchStreak();
    save(); renderTOC(l); updateRing();
    toast(state.completed[id] ? 'Lección completada' : 'Lección desmarcada');
  });
  // flashcards
  $$('[data-flip]').forEach((b) => b.addEventListener('click', () => {
    const card = b.closest('.flashcard');
    card.classList.toggle('open');
    b.textContent = card.classList.contains('open') ? 'Ocultar respuesta' : 'Mostrar respuesta';
  }));
}

/* ------------------------------------------------------------------ questions */
function renderQuestions(params) {
  setNav('questions'); renderTOC(null); updateRing();
  const m = params.get('m') || '';
  const tag = params.get('tag') || '';
  const q = (params.get('q') || '').toLowerCase();
  const tags = [...new Set(bank.flatMap((x) => x.tags))].sort();
  const rows = bank.filter((x) => (!m || x.module === m) && (!tag || x.tags.includes(tag)) && (!q || `${x.q} ${x.why}`.toLowerCase().includes(q)));
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Biblioteca</p><h1>Preguntas</h1><p class="lede">${bank.length} preguntas con solución explicada. Filtra por módulo o tema; tu tasa de acierto se guarda.</p></div></div>
    <div class="filters">
      <select id="f-module"><option value="">Todos los módulos</option>${modules.map((x) => `<option value="${x.id}" ${m === x.id ? 'selected' : ''}>${String(x.num).padStart(2, '0')} · ${esc(x.title)}</option>`).join('')}</select>
      <select id="f-tag"><option value="">Todos los temas</option>${tags.map((t) => `<option ${tag === t ? 'selected' : ''}>${esc(t)}</option>`).join('')}</select>
      <input id="f-q" placeholder="Buscar…" value="${esc(params.get('q') || '')}">
    </div>
    <p class="small muted">${rows.length} resultado(s) · acertadas ${Object.values(state.q).filter((x) => x.correct).length}/${Object.keys(state.q).length || 0}</p>
    <div>${rows.map((x) => `<details class="q-row">
      <summary>${md(x.q)} <span class="pill">${esc(moduleById[x.module]?.title || x.module)}</span></summary>
      <p class="answer">✓ ${md(x.o[x.a])}</p>
      <p class="why">${md(x.why || '')}</p>
      <p class="small muted"><a href="#/lesson/${x.lesson}">Ver lección: ${esc(x.lessonTitle)}</a></p>
    </details>`).join('') || '<div class="empty">No hay preguntas con ese filtro.</div>'}</div>
  </section>`;
  $('#f-module')?.addEventListener('change', (e) => location.hash = `#/questions?m=${e.target.value}${tag ? `&tag=${encodeURIComponent(tag)}` : ''}`);
  $('#f-tag')?.addEventListener('change', (e) => location.hash = `#/questions?${m ? `m=${m}&` : ''}tag=${encodeURIComponent(e.target.value)}`);
  $('#f-q')?.addEventListener('keydown', (e) => { if (e.key === 'Enter') location.hash = `#/questions?${m ? `m=${m}&` : ''}q=${encodeURIComponent(e.target.value)}`; });
}

/* ------------------------------------------------------------------ practice */
function renderPractice() {
  setNav('practice'); renderTOC(null); updateRing();
  const solved = exercises.filter((e) => state.ex[e.id]).length;
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Práctica</p><h1>Retos</h1><p class="lede">${exercises.length} ejercicios cortos para escribir Elisp de verdad. Intenta resolverlos en Emacs antes de mirar la solución; después marca el reto.</p></div></div>
    <div class="card" style="margin-bottom:1rem"><div class="meta-row"><span>${solved}/${exercises.length} resueltos</span><span>${pct(solved, exercises.length)}%</span></div><div class="bar"><span style="width:${pct(solved, exercises.length)}%"></span></div></div>
    ${exercises.map((e) => `<article class="practice-card" id="${e.id}">
      <div class="meta-row"><span class="pill">${esc(moduleById[e.module]?.title || e.module)}</span><span>${esc(e.lessonTitle)}</span></div>
      <h3>${md(e.p)}</h3>
      ${e.start ? codeBlock(e.start, 'elisp', 'Punto de partida') : ''}
      <p class="small muted">Pista: ${md(e.hint)}</p>
      <button class="btn btn-ghost btn-sm" type="button" data-reveal="p-${e.id}">Ver solución</button>
      <div class="solution" id="p-${e.id}">${codeBlock(e.sol, 'elisp', 'Una solución posible')}</div>
      <p style="margin:.8rem 0 0"><button class="btn btn-sm ${state.ex[e.id] ? 'done-btn is-done' : ''}" type="button" data-exdone="${e.id}">${state.ex[e.id] ? '✓ Resuelto' : 'Marcar como resuelto'}</button></p>
    </article>`).join('')}
  </section>`;
  $$('[data-copy]').forEach((b) => b.addEventListener('click', () => copyText(b.dataset.copy).then(() => { b.textContent = 'Copiado'; setTimeout(() => (b.textContent = 'Copiar'), 1300); })));
  $$('[data-reveal]').forEach((b) => b.addEventListener('click', () => {
    const el = document.getElementById(b.dataset.reveal);
    if (el) el.classList.toggle('show');
    b.textContent = el?.classList.contains('show') ? 'Ocultar solución' : 'Ver solución';
  }));
  $$('[data-exdone]').forEach((b) => b.addEventListener('click', () => {
    state.ex[b.dataset.exdone] = !state.ex[b.dataset.exdone];
    save(); location.reload();
  }));
}

/* ------------------------------------------------------------------ quiz */
const QUIZ_TIME = 8 * 60;

function renderQuizHome() {
  setNav('quiz'); renderTOC(null); updateRing();
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Quizzes</p><h1>Ponte a prueba</h1><p class="lede">8 preguntas aleatorias por módulo, 8 minutos. Al final ves la explicación de cada respuesta.</p></div></div>
    <div class="grid">
      ${modules.map((m) => `<article class="module-card">
        <div class="meta-row"><span class="pill">Módulo ${String(m.num).padStart(2, '0')}</span><span>${moduleLessons(m).length} lecciones</span></div>
        <h3>${esc(m.title)}</h3>
        <p>${md(m.subtitle)}</p>
        <a class="btn btn-sm" href="#/quiz/${m.id}">Empezar quiz →</a>
      </article>`).join('')}
    </div>
    <div class="card" style="margin-top:1.2rem">
      <h2>Quiz global</h2>
      <p class="muted">20 preguntas de todos los módulos. Para cuando ya hayas hecho varios.</p>
      <a class="btn" href="#/quiz/all">Quiz global →</a>
    </div>
  </section>`;
}

function renderQuiz(moduleId) {
  setNav('quiz'); renderTOC(null); updateRing();
  const pool = moduleId === 'all' ? bank : bank.filter((q) => q.module === moduleId);
  if (!pool.length) return renderQuizHome();
  const picked = [...pool].sort(() => Math.random() - 0.5).slice(0, Math.min(8, pool.length));
  const label = moduleId === 'all' ? 'Todo el curso' : moduleById[moduleId]?.title || moduleId;
  let remaining = QUIZ_TIME;
  let submitted = false;
  main.innerHTML = `<section class="quiz-shell">
    <div class="quiz-top">
      <div><p class="eyebrow">Quiz · ${esc(label)}</p><h1 style="font-size:1.8rem">${picked.length} preguntas</h1></div>
      <div class="timer" id="quiz-timer">08:00</div>
    </div>
    <form id="quiz-form">
      ${picked.map((q, i) => `<div class="quiz-q">
        <p class="n">PREGUNTA ${i + 1} / ${picked.length}</p>
        <p><strong>${md(q.q)}</strong></p>
        ${q.o.map((o, j) => `<label class="option"><input type="radio" name="q${i}" value="${j}" required> <span>${md(o)}</span></label>`).join('')}
      </div>`).join('')}
      <button class="btn" type="submit">Corregir quiz</button>
    </form>
    <div id="quiz-result"></div>
  </section>`;
  clearInterval(quizTimer);
  quizTimer = setInterval(() => {
    if (submitted) return;
    remaining--;
    const t = $('#quiz-timer');
    if (t) t.textContent = `${String(Math.floor(remaining / 60)).padStart(2, '0')}:${String(remaining % 60).padStart(2, '0')}`;
    if (remaining <= 0) { clearInterval(quizTimer); const f = $('#quiz-form'); if (f) { f.noValidate = true; f.requestSubmit(); } }
  }, 1000);
  $('#quiz-form').addEventListener('submit', (e) => {
    e.preventDefault(); submitted = true; clearInterval(quizTimer);
    const form = new FormData(e.target);
    let correct = 0;
    const review = picked.map((q, i) => {
      const chosen = Number(form.get(`q${i}`));
      const ok = chosen === q.a;
      if (ok) correct++;
      const qid = q.id;
      const prev = state.q[qid];
      if (!prev) state.q[qid] = { seen: 1, correct: ok ? 1 : 0 };
      return `<div class="review-item ${ok ? 'ok' : 'bad'}">
        <p><strong>${ok ? '✓' : '✗'} ${md(q.q)}</strong></p>
        <p class="small">Tu respuesta: ${md(q.o[chosen] ?? '—')}${ok ? '' : ` · Correcta: <strong>${md(q.o[q.a])}</strong>`}</p>
        <p class="small muted">${md(q.why || '')}</p>
      </div>`;
    }).join('');
    $('#quiz-result').innerHTML = `<div class="card" style="margin-top:1.2rem">
      <p class="eyebrow">Resultado</p>
      <p class="score">${correct}/${picked.length}</p>
      <p class="muted">${pct(correct, picked.length)}% · ${correct === picked.length ? 'Perfecto.' : 'Repasa las que fallaste y vuelve a intentarlo.'}</p>
      <div style="margin-top:.8rem">${review}</div>
      <a class="btn" href="#/quiz/${moduleId}">Repetir quiz</a>
      <a class="btn btn-ghost" href="#/questions?m=${moduleId === 'all' ? '' : moduleId}">Ver preguntas</a>
    </div>`;
    window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
    save(); updateRing();
  });
}

/* ------------------------------------------------------------------ review */
function renderReview() {
  setNav('review'); renderTOC(null); updateRing();
  const cards = lessonList.flatMap((l) => (l.cards || []).map((c, i) => ({ id: `${l.id}-c${i}`, l, q: c[0], a: c[1] })));
  const now = Date.now();
  const due = cards.filter((c) => !state.cards[c.id] || state.cards[c.id].due <= now).slice(0, 20);
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Repaso espaciado</p><h1>Repaso</h1><p class="lede">${due.length} tarjeta(s) para hoy. Responde en voz alta antes de mostrar la respuesta; luego marca cuánto te costó.</p></div></div>
    ${due.length ? due.map((c) => `<div class="flashcard" id="card-${c.id}">
      <p class="front">${md(c.q)}</p>
      <p class="back">${md(c.a)}</p>
      <button class="btn btn-ghost btn-sm" type="button" data-flip>Mostrar respuesta</button>
      <div class="actions" style="display:none">
        <button class="btn btn-sm btn-ghost" type="button" data-card-rate="1" data-card-id="${c.id}">Otra vez</button>
        <button class="btn btn-sm" type="button" data-card-rate="3" data-card-id="${c.id}">Bien</button>
        <button class="btn btn-sm btn-ghost" type="button" data-card-rate="7" data-card-id="${c.id}">Fácil</button>
      </div>
      <p class="small muted" style="margin:.5rem 0 0">${esc(c.l.title)}</p>
    </div>`).join('') : '<div class="empty">No hay tarjetas pendientes. Vuelve mañana o marca más lecciones.</div>'}
  </section>`;
  $$('[data-flip]').forEach((b) => b.addEventListener('click', () => {
    const card = b.closest('.flashcard');
    card.classList.toggle('open');
    b.textContent = card.classList.contains('open') ? 'Ocultar respuesta' : 'Mostrar respuesta';
    const actions = card.querySelector('.actions');
    if (actions) actions.style.display = card.classList.contains('open') ? 'flex' : 'none';
  }));
  $$('[data-card-rate]').forEach((b) => b.addEventListener('click', () => {
    const days = Number(b.dataset.cardRate);
    state.cards[b.dataset.cardId] = { due: Date.now() + days * 86400000, days };
    save();
    $('#card-' + b.dataset.cardId)?.remove();
    toast(`Programada en ${days} día(s)`);
  }));
}

/* ------------------------------------------------------------------ progress */
function renderProgress() {
  setNav('progress'); renderTOC(null); updateRing();
  const done = courseDone();
  const seen = Object.keys(state.q).length;
  const correct = Object.values(state.q).filter((x) => x.correct).length;
  const exDone = exercises.filter((e) => state.ex[e.id]).length;
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Panel</p><h1>Tu progreso</h1><p class="lede">Todo se guarda en este navegador. Exporta el JSON si quieres llevarlo a otro equipo.</p></div></div>
    <div class="stats">
      <div class="stat"><strong>${done}/${totalLessons}</strong><span>lecciones</span></div>
      <div class="stat"><strong>${pct(done, totalLessons)}%</strong><span>curso</span></div>
      <div class="stat"><strong>${correct}/${seen || 0}</strong><span>preguntas</span></div>
      <div class="stat"><strong>${state.streak.count}</strong><span>racha</span></div>
    </div>
    <div class="card" style="margin-bottom:1rem"><div class="meta-row"><span>Retos resueltos</span><span>${exDone}/${exercises.length}</span></div><div class="bar"><span style="width:${pct(exDone, exercises.length)}%"></span></div></div>
    <div class="progress-grid">
      ${modules.map((m) => { const d = moduleDone(m); return `<div class="progress-card">
        <div class="row"><strong>${String(m.num).padStart(2, '0')} · ${esc(m.title)}</strong><span>${d}/${m.lessons.length}</span></div>
        <div class="bar"><span style="width:${pct(d, m.lessons.length)}%"></span></div>
      </div>`; }).join('')}
    </div>
    <div class="card" style="margin-top:1.2rem">
      <h2>Datos</h2>
      <p class="small muted">Exporta antes de cambiar de navegador. Importar reemplaza tu progreso actual.</p>
      <button class="btn btn-ghost btn-sm" id="export2" type="button">Exportar JSON</button>
      <button class="btn btn-ghost btn-sm" id="import2" type="button">Importar JSON</button>
      <button class="btn btn-ghost btn-sm" id="reset2" type="button">Borrar progreso</button>
    </div>
  </section>`;
  $('#export2')?.addEventListener('click', exportProgress);
  $('#import2')?.addEventListener('click', () => $('#import-file').click());
  $('#reset2')?.addEventListener('click', () => {
    if (confirm('¿Borrar todo el progreso de este navegador?')) { state = defaultState(); save(); renderProgress(); updateRing(); toast('Progreso borrado'); }
  });
}

/* ------------------------------------------------------------------ lists */
const LISTS = {
  'emacs-50': { title: 'Emacs 50', desc: 'Cincuenta preguntas que recorren todo el curso.', filter: (q, i) => i < 50 },
  'elisp-25': { title: 'Elisp 25', desc: 'El núcleo de Emacs Lisp: formas, listas, funciones, scope, hooks y advice.', filter: (q) => ['m10', 'm11'].includes(q.module) },
  'org-20': { title: 'Org 20', desc: 'Org-mode: estructura, TODO, agenda, tablas, babel y export.', filter: (q) => q.module === 'm06' },
  'magit-20': { title: 'Magit 20', desc: 'Git sin salir de Emacs: staging, commits, diff, rebase y conflictos.', filter: (q) => q.module === 'm07' },
  'entrevista-40': { title: 'Entrevista 40', desc: 'Cuarenta preguntas difíciles de todo el temario.', filter: (q) => q.tags.includes('entrevista') || q.tags.includes('experto') },
  'comandos-30': { title: 'Comandos 30', desc: 'Los atajos y comandos que más se usan.', filter: (q) => q.tags.includes('comandos') },
};
function renderList(id) {
  const list = LISTS[id];
  if (!list) return renderHome();
  setNav('questions'); renderTOC(null); updateRing();
  const rows = bank.filter((q, i) => list.filter(q, i));
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Lista</p><h1>${esc(list.title)}</h1><p class="lede">${md(list.desc)}</p></div></div>
    <p class="small muted">${rows.length} preguntas.</p>
    <div>${rows.map((x) => `<details class="q-row">
      <summary>${md(x.q)} <span class="pill">${esc(moduleById[x.module]?.title || x.module)}</span></summary>
      <p class="answer">✓ ${md(x.o[x.a])}</p>
      <p class="why">${md(x.why || '')}</p>
      <p class="small muted"><a href="#/lesson/${x.lesson}">Ver lección: ${esc(x.lessonTitle)}</a></p>
    </details>`).join('')}</div>
  </section>`;
}

/* ------------------------------------------------------------------ glossary / about / search */
function renderGlossary() {
  setNav('home'); renderTOC(null); updateRing();
  const terms = [...glossary].sort((a, b) => a.term.localeCompare(b.term, 'es'));
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Referencia</p><h1>Glosario</h1><p class="lede">Términos de Emacs y Elisp, con la traducción y la idea en una frase.</p></div></div>
    <div class="grid">${terms.map((t) => `<div class="card"><h3>${esc(t.term)}</h3><p class="small muted" style="margin-bottom:0">${md(t.def)}</p></div>`).join('')}</div>
  </section>`;
}
function renderAbout() {
  setNav('home'); renderTOC(null); updateRing();
  main.innerHTML = `<section class="lesson">
    <p class="eyebrow">Cómo usar</p><h1>Cómo está construido este curso</h1>
    <p class="lede">No es una lista de atajos. Es un recorrido por el modelo de Emacs para que puedas deducir comandos, escribir Elisp y diseñar tu propio entorno.</p>
    <h2>Qué vas a encontrar</h2>
    <ul><li><strong>14 módulos y ${totalLessons} lecciones</strong>, de 6 a 15 minutos.</li><li><strong>${bank.length} preguntas</strong> con explicación y seguimiento de aciertos.</li><li><strong>${exercises.length} retos</strong> para escribir código y comparar con una solución.</li><li><strong>Quizzes</strong> cronometrados y repaso espaciado.</li></ul>
    <h2>Qué necesitas</h2>
    <p>Emacs instalado (recomendado 29.1 o superior). El curso usa Elisp estándar y menciona paquetes populares, pero no te obliga a instalar un pack de inicio. Copia los ejemplos en <code>*scratch*</code> y evalúalos con <code>C-x C-e</code> o <code>M-:</code>.</p>
    <h2>Qué no hace</h2>
    <p>No ejecuta Elisp en el navegador: sería inseguro y limitado. La corrección automática está en las preguntas y quizzes; los retos se comparan con una solución. La práctica real ocurre en tu Emacs.</p>
    <h2>Método</h2>
    <p>Predice → lee la idea → copia y ejecuta → entiende el “bajo el capó” → evita la trampa → responde → escribe tu versión. Si no escribes, no aprendes Emacs.</p>
  </section>`;
}
function renderSearch(q) {
  setNav('home'); renderTOC(null); updateRing();
  const term = q.toLowerCase();
  const ls = term ? lessonList.filter((l) => `${l.title} ${l.obj} ${l.idea.join(' ')} ${(l.tags || []).join(' ')}`.toLowerCase().includes(term)) : [];
  const qs = term ? bank.filter((x) => `${x.q} ${x.why}`.toLowerCase().includes(term)).slice(0, 30) : [];
  main.innerHTML = `<section>
    <div class="page-head"><div><p class="eyebrow">Buscar</p><h1>“${esc(q)}”</h1><p class="lede">${ls.length} lección(es) y ${qs.length} pregunta(s).</p></div></div>
    ${ls.length ? `<h2>Lecciones</h2><div class="grid">${ls.map((l) => `<a class="card" href="#/lesson/${l.id}" style="color:inherit;text-decoration:none"><h3>${esc(l.title)}</h3><p class="small muted" style="margin:0">${md(l.obj)}</p></a>`).join('')}</div>` : ''}
    ${qs.length ? `<h2 style="margin-top:1.4rem">Preguntas</h2>${qs.map((x) => `<details class="q-row"><summary>${md(x.q)}</summary><p class="answer">✓ ${md(x.o[x.a])}</p><p class="why">${md(x.why || '')}</p></details>`).join('')}` : ''}
    ${!ls.length && !qs.length ? '<div class="empty">Sin resultados. Prueba con “buffers”, “org”, “magit”, “hooks” o “lifetimes”.</div>' : ''}
  </section>`;
}

/* ------------------------------------------------------------------ export / import */
function exportProgress() {
  const blob = new Blob([JSON.stringify(state, null, 2)], { type: 'application/json' });
  const a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = 'emacs-desde-cero-progreso.json';
  a.click();
  URL.revokeObjectURL(a.href);
  toast('Progreso exportado');
}
$('#export-btn')?.addEventListener('click', exportProgress);
$('#import-btn')?.addEventListener('click', () => $('#import-file').click());
$('#import-file')?.addEventListener('change', async (e) => {
  const file = e.target.files?.[0];
  if (!file) return;
  try {
    const data = JSON.parse(await file.text());
    state = { ...defaultState(), ...data };
    save(); route(); updateRing(); toast('Progreso importado');
  } catch { toast('Archivo inválido'); }
  e.target.value = '';
});

/* ------------------------------------------------------------------ global chrome */
$('#menu-btn')?.addEventListener('click', () => {
  const sb = $('#sidebar');
  sb.classList.toggle('open');
  $('#menu-btn').setAttribute('aria-expanded', String(sb.classList.contains('open')));
});
$('#search')?.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    const q = e.target.value.trim();
    if (q) location.hash = `#/search?q=${encodeURIComponent(q)}`;
  }
});
document.addEventListener('keydown', (e) => {
  if (e.key === '/' && !/^(INPUT|TEXTAREA|SELECT)$/.test(e.target.tagName)) {
    e.preventDefault(); $('#search')?.focus();
  }
  if (e.key === 'Escape') $('#sidebar')?.classList.remove('open');
});
$$('.main-nav a').forEach((a) => a.addEventListener('click', () => $('#sidebar')?.classList.remove('open')));
updateRing();
