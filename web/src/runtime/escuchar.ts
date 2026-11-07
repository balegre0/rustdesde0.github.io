// «Escuchar»: lee en voz alta una lección (o la chuleta) con las voces del propio dispositivo (Web Speech API).
// Resalta y sigue en pantalla el bloque que se está leyendo. El código no se lee símbolo a símbolo: se avisa
// de que hay un ejemplo. En las tarjetas de repaso, lee la pregunta, deja unos segundos y lee la respuesta.

type Trozo = { el: HTMLElement; texto: string; pausaAntes?: number };

const VELOCIDADES = [0.85, 1, 1.15, 1.3, 1.5];
const CLAVE_VEL = 'rdc.escuchar.velocidad';

/** Pronunciación de símbolos y palabras de Rust que la voz leería mal. */
function paraVoz(s: string) {
  return s
    .replace(/&mut\b/g, ' referencia mutable ')
    .replace(/&'static/g, ' referencia static ')
    .replace(/&(?=[\w'\[])/g, ' referencia a ')
    .replace(/->/g, ' devuelve ')
    .replace(/=>/g, ' entonces ')
    .replace(/::/g, ' ')
    .replace(/\.\.=/g, ' hasta, incluido, ')
    .replace(/(\d)\.\.(\d)/g, '$1 hasta $2')
    .replace(/\b(\w+)!(?=[\s(\[{.,;:]|$)/g, '$1 ')
    .replace(/\bprintln\b/g, 'print line')
    .replace(/\bfn\b/g, 'efe ene')
    .replace(/\bstr\b/g, 'estr')
    .replace(/\bVec\b/g, 'vec')
    .replace(/\bi32\b/g, 'i treinta y dos').replace(/\bu8\b/g, 'u ocho').replace(/\bu32\b/g, 'u treinta y dos')
    .replace(/\bi64\b/g, 'i sesenta y cuatro').replace(/\bu64\b/g, 'u sesenta y cuatro').replace(/\bf64\b/g, 'f sesenta y cuatro')
    .replace(/\busize\b/g, 'u size')
    .replace(/[`*_#|]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

/** Parte un texto largo en frases (algunas voces se cortan con textos muy largos). */
function frases(s: string): string[] {
  const out: string[] = [];
  for (const f of s.split(/(?<=[.!?:;])\s+(?=[A-ZÁÉÍÓÚÑ¿¡«"(0-9])/)) {
    if (f.length <= 220) { if (f.trim()) out.push(f); continue; }
    let resto = f;
    while (resto.length > 220) {
      const corte = Math.max(resto.lastIndexOf(', ', 220), resto.lastIndexOf(' ', 200));
      out.push(resto.slice(0, corte > 40 ? corte + 1 : 220));
      resto = resto.slice(corte > 40 ? corte + 1 : 220);
    }
    if (resto.trim()) out.push(resto);
  }
  return out;
}

const SALTAR = '[hidden], .expressive-code, pre, astro-island, .rdc-pasos, .rdc-explica, .rdc-escuchar, .rdc-term-globo, script, style, .sr-only, [aria-hidden="true"], .rdc-glosario-buscar';

/** Recorre el contenido y saca los bloques que se leen, en orden. */
function recoger(raiz: HTMLElement): Trozo[] {
  const trozos: Trozo[] = [];
  const vistos = new Set<Element>();
  const textoDe = (el: Element) => {
    const copia = el.cloneNode(true) as HTMLElement;
    copia.querySelectorAll(SALTAR).forEach((x) => x.remove());
    // Separa los bloques internos (etiqueta «En una frase» + la frase) para que la voz haga una pausa entre ellos.
    copia.querySelectorAll('p, li, div, dt, dd, h2, h3, h4, br, .rdc-chuleta-titulo, .rdc-chuleta-num').forEach((x) => {
      const t = (x.textContent ?? '').trim();
      x.after(document.createTextNode(t && !/[.!?:;…]$/.test(t) ? '. ' : ' '));
    });
    return (copia.textContent ?? '').replace(/\s+/g, ' ').replace(/\s+([.,;:])/g, '$1').replace(/\.\s*\./g, '.').trim();
  };
  const recorrer = (el: Element) => {
    for (const hijo of Array.from(el.children) as HTMLElement[]) {
      if (vistos.has(hijo) || hijo.hidden) continue;
      // Tarjetas de repaso: pregunta, pausa para pensar, respuesta.
      if (hijo.matches('.rdc-repasa')) {
        const h = hijo.querySelector('h2');
        if (h) trozos.push({ el: h as HTMLElement, texto: 'Repasa sin mirar. Intenta responder antes de oír la respuesta.' });
        for (const d of Array.from(hijo.querySelectorAll('details'))) {
          const p = d.querySelector('summary span');
          const r = d.querySelector(':scope > p');
          if (p) trozos.push({ el: d as HTMLElement, texto: 'Pregunta: ' + textoDe(p) });
          if (r) trozos.push({ el: d as HTMLElement, texto: 'Respuesta: ' + textoDe(r), pausaAntes: 4000 });
        }
        continue;
      }
      if (hijo.matches('.expressive-code, pre')) {
        trozos.push({ el: hijo, texto: 'Aquí hay un ejemplo de código en pantalla.' });
        continue;
      }
      if (hijo.matches('.rdc-pasos')) { trozos.push({ el: hijo, texto: 'Aquí hay una ejecución paso a paso: pruébala en pantalla.' }); continue; }
      if (hijo.matches('astro-island, .rdc-quiz, .rdc-predice, [data-ejecutable], .rdc-reto, .rdc-ordena')) {
        trozos.push({ el: hijo, texto: 'Aquí hay un ejercicio interactivo para hacer en pantalla.' });
        continue;
      }
      if (hijo.matches(SALTAR)) continue;
      if (hijo.matches('h1, h2, h3, h4, p, li, dt, dd, blockquote, figcaption, .rdc-clave-bloque, .rdc-antes, summary, tr')) {
        const t = textoDe(hijo);
        if (t) trozos.push({ el: hijo, texto: t });
        vistos.add(hijo);
        continue;
      }
      recorrer(hijo);
    }
  };
  recorrer(raiz);
  // Varios avisos de «ejemplo» seguidos se dicen una sola vez.
  return trozos.filter((t, i) => !(i > 0 && t.texto === trozos[i - 1].texto && t.texto.startsWith('Aquí hay')));
}

const CLAVE_VOZ = 'rdc.escuchar.voz';

/** Calidad estimada de una voz: las neuronales («Natural», Google, «Mejorada»/«Premium») suenan humanas;
 *  las clásicas del sistema (p. ej. Helena o Sabina en Windows) suenan robóticas. */
function calidad(v: SpeechSynthesisVoice) {
  const n = v.name;
  let p = 0;
  if (/natural|neural/i.test(n)) p += 100; // Microsoft Edge: Elvira, Álvaro, Dalia, Jorge… (Online, Natural)
  else if (/google/i.test(n)) p += 80; // Chrome y Android
  else if (/premium|mejorada|enhanced|siri/i.test(n)) p += 70; // Apple
  else if (/online/i.test(n)) p += 60;
  if (/^es-(es|mx)$/i.test(v.lang.replace('_', '-'))) p += 5;
  if (/desktop|helena|sabina|laura|pablo|raul|hilda/i.test(n)) p -= 40; // voces clásicas de Windows
  return p;
}

/** Voces en español del dispositivo, de mejor a peor. Espera a que el navegador las cargue (llegan tarde). */
function vocesEspanol(): Promise<SpeechSynthesisVoice[]> {
  const ya = () => speechSynthesis.getVoices().filter((v) => v.lang.toLowerCase().replace('_', '-').startsWith('es'));
  const ordenar = (vs: SpeechSynthesisVoice[]) => vs.sort((a, b) => calidad(b) - calidad(a) || a.name.localeCompare(b.name));
  if (speechSynthesis.getVoices().length) return Promise.resolve(ordenar(ya()));
  return new Promise((ok) => {
    const fin = () => ok(ordenar(ya()));
    speechSynthesis.addEventListener?.('voiceschanged', fin, { once: true });
    setTimeout(fin, 2500);
  });
}

const nombreVoz = (v: SpeechSynthesisVoice) => {
  const pais: Record<string, string> = { 'es-es': 'España', 'es-mx': 'México', 'es-us': 'EE. UU.', 'es-ar': 'Argentina', 'es-co': 'Colombia', 'es-cl': 'Chile', 'es-419': 'Latinoamérica' };
  const corto = /^google/i.test(v.name) ? 'Google' : v.name.replace(/^Microsoft\s+/i, '').replace(/\s*\(.*$/, '').replace(/\s+-\s+.*$/, '').replace(/\s+Online/i, '');
  const q = calidad(v) >= 60 ? ' · natural' : ' · básica';
  return `${corto} (${pais[v.lang.toLowerCase().replace('_', '-')] ?? v.lang})${q}`;
};

export function montarEscuchar(barra: HTMLElement) {
  const objetivo = document.querySelector<HTMLElement>(barra.dataset.escuchar || '.sl-markdown-content');
  const play = barra.querySelector<HTMLButtonElement>('[data-esc-play]')!;
  const ant = barra.querySelector<HTMLButtonElement>('[data-esc-ant]')!;
  const sig = barra.querySelector<HTMLButtonElement>('[data-esc-sig]')!;
  const parar = barra.querySelector<HTMLButtonElement>('[data-esc-parar]')!;
  const vel = barra.querySelector<HTMLButtonElement>('[data-esc-vel]')!;
  const estado = barra.querySelector<HTMLElement>('[data-esc-estado]')!;
  const probar = barra.querySelector<HTMLButtonElement>('[data-esc-probar]')!;
  if (!objetivo) return;
  if (!('speechSynthesis' in window)) {
    barra.querySelector<HTMLElement>('[data-esc-controles]')!.hidden = true;
    estado.textContent = 'Tu navegador no puede leer en voz alta. Prueba con Chrome, Edge o Safari.';
    return;
  }

  const selector = barra.querySelector<HTMLSelectElement>('[data-esc-voz]')!;
  const aviso = barra.querySelector<HTMLElement>('[data-esc-aviso]')!;
  let voces: SpeechSynthesisVoice[] = [];
  let voz: SpeechSynthesisVoice | null = null;
  let listas = false;
  const prepararVoces = vocesEspanol().then((vs) => {
    voces = vs;
    listas = true;
    let guardada = '';
    try { guardada = localStorage.getItem(CLAVE_VOZ) ?? ''; } catch { /* sin almacenamiento */ }
    voz = vs.find((v) => v.name === guardada) ?? vs[0] ?? null;
    selector.replaceChildren(...vs.map((v) => new Option(nombreVoz(v), v.name, false, v === voz)));
    selector.hidden = vs.length === 0;
    const edge = /Edg\//.test(navigator.userAgent);
    if (!vs.length) {
      aviso.innerHTML = 'Tu dispositivo no tiene ninguna voz en español, por eso suena mal. <strong>En Windows:</strong> Configuración → Hora e idioma → Voz → Agregar voces → Español. O abre la web en <strong>Microsoft Edge</strong>, que trae voces naturales.';
      aviso.hidden = false;
    } else if (voz && calidad(voz) < 60) {
      const hayNatural = vs.some((v) => calidad(v) >= 60);
      aviso.innerHTML = hayNatural
        ? 'Esta voz suena robótica: elige en la lista una marcada como «natural».'
        : edge
          ? 'Esta voz es la básica del sistema y suena robótica. Las voces naturales de Edge (Elvira, Álvaro, Dalia…) necesitan <strong>conexión a internet</strong>: conéctate y recarga la página.'
          : 'Esta voz es la básica del sistema y suena robótica. Para una voz natural, abre la web en <strong>Microsoft Edge</strong> (voces Elvira, Álvaro, Dalia…) o en <strong>Chrome</strong> (voz de Google).';
      aviso.hidden = false;
    } else aviso.hidden = true;
  });
  selector.addEventListener('change', () => {
    voz = voces.find((v) => v.name === selector.value) ?? voz;
    if (voz) fallidas.delete(voz.name);
    try { localStorage.setItem(CLAVE_VOZ, selector.value); } catch { /* sin almacenamiento */ }
    if (voz && calidad(voz) >= 60) aviso.hidden = true;
    if (sonando && !enPausa) decir(i);
  });

  let trozos: Trozo[] = [];
  let i = 0;
  let sonando = false;
  let enPausa = false;
  let temporizador = 0;
  let velocidad = 1;
  try { velocidad = Number(localStorage.getItem(CLAVE_VEL)) || 1; } catch { /* sin almacenamiento */ }
  const mostrarVel = () => { vel.textContent = `${String(velocidad).replace('.', ',')}×`; vel.setAttribute('aria-label', `Velocidad ${velocidad}`); };
  mostrarVel();

  const marcar = (el: HTMLElement | null) => {
    document.querySelectorAll('.rdc-leyendo').forEach((x) => x.classList.remove('rdc-leyendo'));
    if (!el) return;
    // Si el bloque está dentro de un desplegable cerrado («Bajo el capó»), se abre para verlo.
    const d = el.closest('details');
    if (d && !d.open && !el.matches('details')) d.open = true;
    el.classList.add('rdc-leyendo');
    const r = el.getBoundingClientRect();
    if (r.top < 80 || r.bottom > innerHeight - 40) el.scrollIntoView({ block: 'center', behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
  };
  const ponerPlay = (reproduciendo: boolean) => {
    play.textContent = reproduciendo ? '❚❚ Pausa' : (i > 0 ? '▶ Seguir' : `▶ ${barra.dataset.etiqueta ?? 'Escuchar'}`);
    play.setAttribute('aria-pressed', String(reproduciendo));
    barra.toggleAttribute('data-sonando', reproduciendo || enPausa || i > 0);
  };

  // Generación: cada vez que se corta la lectura sube, y los avisos de frases viejas se ignoran.
  let generacion = 0;
  let vigia = 0;
  // Referencias a las frases en cola: si no se guardan, el navegador puede tirarlas y no avisar al terminar.
  const vivas: SpeechSynthesisUtterance[] = [];
  const fallidas = new Set<string>();
  const ocupado = () => speechSynthesis.speaking || speechSynthesis.pending;
  const callar = () => {
    generacion++;
    clearTimeout(temporizador);
    clearTimeout(vigia);
    vivas.length = 0;
    if (ocupado()) speechSynthesis.cancel();
  };

  /** La voz actual no habla: se prueba la siguiente; si no queda ninguna, se explica qué revisar. */
  const otraVoz = (k: number, motivo: string) => {
    if (voz) fallidas.add(voz.name);
    const siguiente = voces.find((v) => !fallidas.has(v.name));
    const antes = voz ? nombreVoz(voz) : 'La voz del sistema';
    if (siguiente) {
      voz = siguiente;
      selector.value = siguiente.name;
      estado.textContent = `${antes} no responde (${motivo}). Probando con ${nombreVoz(siguiente)}…`;
      decir(k);
    } else if (voz) {
      voz = null; // último intento: la voz por defecto del navegador, pidiendo español
      estado.textContent = `Ninguna voz respondió (${motivo}). Probando con la voz por defecto…`;
      decir(k);
    } else {
      terminar(false);
      estado.textContent = '';
      aviso.innerHTML = `Tu navegador no consigue hablar (${motivo}). Revisa: <strong>1)</strong> el volumen de Windows y que esté elegido el altavoz correcto; <strong>2)</strong> que la pestaña no esté silenciada (clic derecho en la pestaña → «Activar sonido del sitio»); <strong>3)</strong> que tengas conexión a internet si usas una voz «natural». Después pulsa «Probar voz».`;
      aviso.hidden = false;
    }
  };

  const decir = (k: number) => {
    const habiaSonido = ocupado();
    callar();
    if (k >= trozos.length) { terminar(true); return; }
    i = Math.max(0, k);
    const gen = generacion;
    const t = trozos[i];
    marcar(t.el);
    estado.textContent = `${i + 1} de ${trozos.length}`;
    const partes = frases(paraVoz(t.texto));
    if (!partes.length) { setTimeout(() => { if (gen === generacion) decir(k + 1); }, 0); return; }
    const lanzar = () => {
      if (gen !== generacion) return;
      speechSynthesis.resume(); // por si el navegador dejó la síntesis en pausa (pestaña en segundo plano)
      let empezo = false;
      const t0 = performance.now();
      partes.forEach((p, j) => {
        const u = new SpeechSynthesisUtterance(p);
        u.lang = voz?.lang ?? 'es-ES';
        if (voz) u.voice = voz;
        u.rate = velocidad;
        u.onstart = () => { empezo = true; };
        u.onerror = (e) => {
          if (gen !== generacion || e.error === 'interrupted' || e.error === 'canceled') return;
          if (e.error === 'not-allowed') { terminar(false); estado.textContent = 'El navegador pide que pulses ▶ otra vez.'; return; }
          otraVoz(k, e.error);
        };
        if (j === partes.length - 1) {
          u.onend = () => {
            if (gen !== generacion || !sonando || enPausa) return;
            // Terminar «al instante» sin haber empezado a hablar: es una voz rota que no da error.
            if (!empezo && performance.now() - t0 < 250 && p.length > 20) { otraVoz(k, 'sin sonido'); return; }
            setTimeout(() => { if (gen === generacion) decir(k + 1); }, 0);
          };
        }
        vivas.push(u);
        speechSynthesis.speak(u);
      });
      // Si en 5 s no ha empezado a hablar (p. ej. una voz natural sin conexión), se prueba otra.
      vigia = window.setTimeout(() => { if (gen === generacion && !empezo && sonando && !enPausa && ocupado()) otraVoz(k, 'no empieza'); }, 5000);
    };
    // Tras cortar una lectura, se espera un poco: hablar justo después de cancel() puede no sonar en Edge.
    const espera = (t.pausaAntes ? t.pausaAntes / velocidad : 0) + (habiaSonido ? 180 : 0);
    if (espera) temporizador = window.setTimeout(lanzar, espera);
    else lanzar();
  };

  const terminar = (alFinal: boolean) => {
    callar();
    sonando = false; enPausa = false;
    if (alFinal) { i = 0; estado.textContent = 'Fin. ¡Bien hecho!'; }
    marcar(null);
    ponerPlay(false);
    if (alFinal) barra.removeAttribute('data-sonando');
  };

  const empezar = (k: number) => {
    if (!trozos.length) trozos = recoger(objetivo);
    if (/no consigue hablar/.test(aviso.textContent ?? '')) aviso.hidden = true;
    sonando = true; enPausa = false; ponerPlay(true);
    // Si las voces ya están, se habla dentro del mismo clic (algunos navegadores lo exigen).
    if (listas) decir(k);
    else prepararVoces.then(() => { if (sonando && !enPausa) decir(k); });
  };
  play.addEventListener('click', () => {
    if (sonando && !enPausa) {
      // Pausa: se corta y, al seguir, se vuelve a leer el bloque actual desde el principio (más fiable que pause()).
      enPausa = true; callar(); ponerPlay(false); return;
    }
    empezar(i);
  });
  sig.addEventListener('click', () => empezar(i + 1));
  ant.addEventListener('click', () => empezar(i - 1));
  parar.addEventListener('click', () => { i = 0; estado.textContent = ''; terminar(false); barra.removeAttribute('data-sonando'); ponerPlay(false); });
  probar.addEventListener('click', () => {
    callar();
    const gen = generacion;
    const u = new SpeechSynthesisUtterance('Hola. Así suena esta voz.');
    u.lang = voz?.lang ?? 'es-ES';
    if (voz) u.voice = voz;
    u.rate = velocidad;
    let empezo = false;
    estado.textContent = 'Probando…';
    u.onstart = () => { empezo = true; if (gen === generacion) estado.textContent = '✓ La voz está hablando. Si no oyes nada, sube el volumen o revisa que la pestaña no esté silenciada.'; };
    u.onerror = (e) => { if (gen === generacion) estado.textContent = `✗ La voz falló (${e.error}). Elige otra voz en la lista y vuelve a probar.`; };
    u.onend = () => { if (gen === generacion && !empezo) estado.textContent = '✗ La voz terminó sin hablar. Elige otra voz en la lista y vuelve a probar.'; };
    vivas.push(u);
    speechSynthesis.resume();
    speechSynthesis.speak(u);
    window.setTimeout(() => { if (gen === generacion && !empezo && estado.textContent === 'Probando…') estado.textContent = '✗ La voz no responde. Elige otra voz en la lista (o comprueba tu conexión si es «natural»).'; }, 5000);
  });
  vel.addEventListener('click', () => {
    velocidad = VELOCIDADES[(VELOCIDADES.indexOf(velocidad) + 1) % VELOCIDADES.length] ?? 1;
    try { localStorage.setItem(CLAVE_VEL, String(velocidad)); } catch { /* sin almacenamiento */ }
    mostrarVel();
    if (sonando && !enPausa) decir(i);
  });
  addEventListener('pagehide', () => speechSynthesis.cancel());
}

for (const b of document.querySelectorAll<HTMLElement>('[data-escuchar-barra]')) montarEscuchar(b);
