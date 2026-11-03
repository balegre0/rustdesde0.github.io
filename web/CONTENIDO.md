# Contrato de contenido — «Rust desde cero»

> **CAMBIO (petición del usuario, 2026-11-02 10:25):**
> 1. **Nada heredado del curso de C**: ni lecciones, ni textos, ni el flujo de aquel curso. Todo se escribe desde cero para Rust; principiante, técnico, y sin referencias a ningún otro lenguaje.
> 2. **Modelo de enseñanza tipo getcracked.io**: el sitio es ante todo una **plataforma de práctica**. Además de las lecciones (que pasan a ser los «módulos guiados»), hay: **biblioteca de preguntas** (tabla filtrable por tema/mundo, dificultad y tu tasa de acierto, con solución detallada), **problemas de código** con editor y tests, **listas** («Rust 50», «Ownership 25»…), **quizzes** cronometrados con puntuación y mejor marca, **minijuegos** cortos con tiempo, **árbol de progreso** y un panel de estadísticas propio. Ver §7.

> Lo cumplen TODOS los que escriben contenido. La plataforma implementa exactamente esto. Temario: `../investigacion/09-sintesis-y-temario.md`.

## 0. Reglas de oro

1. **Español**, tuteo, frases cortas. Público: alguien que **nunca ha programado**. Explicar todo sin perder lo técnico; el objetivo final es dominio de diseñador del lenguaje. Términos: primera aparición «término (*english*)». Vocabulario fijo: *propiedad* (ownership), *mover* (move), *préstamo compartido* (`&T`), *préstamo exclusivo* (`&mut T`), *tiempo de vida* (lifetime), *rasgo* → usamos **trait** (sin traducir), *pánico* (panic), *comportamiento indefinido* (UB).
2. **Solo Rust.** No se compara con otros lenguajes (excepción: la lección de historia puede nombrar sus influencias).
3. **Exactitud**: Rust estable **1.99.1**, **edición 2024**. Cada regla del borrow checker se explica junto con **el error que evita** (uso tras liberar, doble liberación, aliasing con mutación, carrera de datos).
4. **Toda salida o error que aparezca en el texto se obtiene compilando y ejecutando de verdad** (§4). Nunca de memoria. Los datos marcados [K] en `../investigacion/` se comprueban antes de usarlos.
5. Sin copiar texto ajeno: parafrasear; código propio.
6. Tono que normaliza el error («al borrow checker se le gana entendiéndolo, no peleando»), nunca culpa.

## 1. Árbol de archivos

```
web/
  ejemplos/<mundo>/<id>.rs           # fuente única de cada ejemplo
  ejemplos/<mundo>/<id>.json         # metadatos opcionales (§4)
  ejemplos/<mundo>/<id>.inicio.rs    # código inicial de un reto (el .rs es la solución)
  src/content/docs/curso/<NN-slug-mundo>/index.mdx
  src/content/docs/curso/<NN-slug-mundo>/<NN-slug>.mdx
  src/content/docs/ref/...            # referencia (§5)
  src/content/docs/recetas|errores|explicaciones/<slug>.mdx
```
`<mundo>` en `ejemplos/` = `m00`…`m24`. Mundos en `curso/`: `00-el-taller`, `01-la-maquina`, `02-tu-primer-programa`, `03-valores-y-tipos`, `04-expresiones-y-control`, `05-funciones-y-la-pila`, `06-ownership`, `07-prestamos`, `08-lifetimes`, `09-structs-enums-y-patrones`, `10-errores`, `11-colecciones-y-texto`, `12-modulos-crates-y-cargo`, `13-traits-y-genericos`, `14-closures-e-iteradores`, `15-punteros-inteligentes`, `16-pruebas-y-herramientas`, `17-concurrencia`, `18-async`, `19-macros`, `20-unsafe-y-ub`, `21-el-compilador-por-dentro`, `22-sistemas-y-ffi`, `23-embebido-y-tu-so`, `24-teoria-y-diseno`.

## 2. Frontmatter de una lección

```yaml
---
title: "Préstamos exclusivos: &mut"
description: "Una frase: lo que vas a conseguir."
sidebar:
  order: 3
duracion: 10            # minutos (5–15)
desde: "1.0"            # versión de Rust desde la que existe lo principal ("1.0", "1.65", "2021", "2024"…)
requiere: ["06-ownership/02-mover"]
tarjetas:
  - p: "¿Cuántos préstamos exclusivos de un valor puede haber a la vez?"
    r: "Uno, y mientras exista no puede haber préstamos compartidos."
---
```

## 3. Estructura de una lección (8–15 min)

1. *(sin encabezado)* 1–2 frases: qué vas a conseguir.
2. `## Predice` — `<Predice>`, `<Pregunta>` o `<NoCompila>` de calentamiento («¿por qué no compila?» es mejor que «¿compila?»).
3. `## La idea` — explicación mínima (máx. 3 párrafos sin algo interactivo o un ejemplo).
4. `## Pruébalo` — `<Ejecutar>` con comentarios-subobjetivo (`// 1. …`).
5. `## Bajo el capó` — `<BajoElCapo>`: memoria (pila/heap con direcciones y tamaños), permisos, MIR/asm, la Reference. Siempre presente.
6. `## Trampa típica` — `<Trampa>` (❌ mal / ✅ bien) o `<NoCompila>` con el error real.
7. `## Tu turno` — `<Reto>` u `<Ordena>`.
8. `## Comprueba` — 2–4 preguntas con distractores basados en malentendidos reales (`../investigacion/02-dificultades-y-errores.md`).
9. Opcional: `<Historia>`, `## Para saber más` (referencia interna + máx. 2 externos).

## 4. Componentes (importar desde `~/components`)

```mdx
import { Ejecutar, NoCompila, Prestamos, Predice, Pregunta, Respuesta, Ordena, Reto, BajoElCapo, Trampa, Historia, Bits } from '~/components';
```

| Componente | Props | Qué hace |
|---|---|---|
| `<Ejecutar ej="m06/mover" />` | `ej`, `titulo?` | Código de `ejemplos/m06/mover.rs` resaltado + ▶ que lo ejecuta en el navegador (wasm). stdin interactivo. Si hay perfiles debug/release muestra ambas salidas. |
| `<NoCompila ej="m07/dos-mut" />` | `ej`, `explicacion?` | Para ejemplos con `"debe_fallar": true`: muestra el código y **el error real de rustc** (código E0xxx, mensaje, línea marcada) capturado en build, con enlace a `/errores/E0xxx/` y botón «Ver arreglo» si existe `<id>.arreglo.rs` (que sí compila y se ejecuta). |
| `<Prestamos lineas={[...]} />` | `lineas`: array de `{ codigo: "let r = &v;", permisos: { v: "R-", r: "RO" }, nota?: "…" }` | Tabla de permisos línea a línea (R = leer, W = escribir, O = propiedad; `-` = perdido), estilo Aquascope. Los datos los escribe el autor y deben coincidir con lo que acepta/rechaza el compilador (verificado). |
| `<Predice ej="…" opciones={[…]} correcta={1} explicacion="…" porque={[…]} />` | | Elegir qué imprime (o qué error da) antes de ejecutar. |
| `<Pregunta pregunta="…" opciones={[…]} correcta={0} explicacion="…" porque={[…]} />` | | Opción múltiple (admite `**negrita**` y `` `código` ``). |
| `<Respuesta pregunta="…" respuestas={["…"]} explicacion="…" />` | | Respuesta escrita. |
| `<Ordena lineas={[…]} distractores={[…]} explicacion="…" />` | | Parsons. |
| `<Reto ej="m03/suma" enunciado="…" />` | requiere `suma.inicio.rs` + `suma.rs` | Editor (CodeMirror, Rust). Se ejecuta en el **Rust Playground** (aviso: el código se envía a play.rust-lang.org); superado si la salida coincide con la de la solución. «Ver solución» tras 3 intentos. |
| `<BajoElCapo>`, `<Trampa titulo>`, `<Historia titulo>` | | Bloques plegable / aviso / historia. |
| `<Bits objetivo={200} />` | | 8 interruptores. |

### Metadatos `ejemplos/<mundo>/<id>.json` (opcionales)

```json
{
  "stdin": "21\n",
  "perfiles": ["debug", "release"],   // por defecto ["debug"]; para overflow, mostrar ambos
  "debe_fallar": true,                 // no debe compilar: se captura el error real (E0xxx)
  "plataforma": "wasm",                // wasm (por defecto) | playground (hilos, red, crates) | local
  "nota": "En el navegador usize mide 4 bytes."
}
```
Límites del navegador (verificados): `wasm32-wasip1` (`usize` = 4 bytes), **sin hilos** (`thread::spawn` → pánico «Unsupported»), sin red, sin crates externos (solo `std`). Un pánico muestra su mensaje real. Antes de leer stdin: `print!` + `std::io::stdout().flush()`.

### Verificar cada ejemplo ANTES de escribir su salida

```bash
# ejecutable (debug = con comprobación de overflow):
rustc --edition 2024 --target wasm32-wasip1 -C opt-level=s -C strip=symbols -C debug-assertions=on -C overflow-checks=on ejemplos/m03/x.rs -o /tmp/x.wasm
bun run web/scripts/correr-wasm.ts /tmp/x.wasm < entrada.txt
# release: igual pero con -C overflow-checks=off -C debug-assertions=off
# debe fallar: capturar el error real
rustc --edition 2024 --crate-type bin --emit=metadata --error-format=human ejemplos/m07/dos-mut.rs -o /tmp/x.rmeta
```
(No hay `link.exe` en esta máquina: para comprobar que algo compila en nativo usar `--emit=metadata`; para ejecutar, wasm.)

## 5. Referencia (`src/content/docs/ref/`), estilo rustdoc

```
ref/lenguaje/palabras-clave/<kw>.mdx      # todas las palabras clave (estrictas, reservadas, débiles)
ref/lenguaje/<concepto>.mdx               # tipos, patrones, expresiones, layout, varianza, ...
ref/std/<modulo>/index.mdx                # módulos de std (y core/alloc cuando difieran)
ref/std/<modulo>/<item>.mdx               # tipos, traits, funciones, macros importantes
ref/ub.mdx                                # la lista de comportamiento indefinido de la Reference
ref/ediciones.mdx                         # 2015, 2018, 2021, 2024
ref/versiones.mdx
errores/E0xxx.mdx                         # índice de errores del compilador explicados
```
Ficha: Sinopsis (firma exacta), Qué hace, Ejemplo (`<Ejecutar>` verificado), Pánicos / Errores, Seguridad (para `unsafe fn`: requisitos), Rendimiento (si aplica), Desde (versión), Ver también, Aprende esto en.

## 7. Práctica estilo getcracked (nuevo)

### 7.1 Preguntas — `web/contenido/preguntas/<mundo>.yaml` (lista)

```yaml
- id: m07-dos-prestamos-mut          # único en todo el sitio
  titulo: "Dos préstamos exclusivos"   # corto y con gancho
  mundo: 07-prestamos
  tema: prestamos                      # etiqueta de filtro (ownership, prestamos, lifetimes, traits, ...)
  dificultad: facil                    # facil | media | dificil
  tipo: no-compila                     # opcion | respuesta | predice | no-compila
  enunciado: "¿Por qué no compila este programa?"   # markdown
  ej: m07/dos-mut                      # opcional: ejemplo de ejemplos/ (para predice / no-compila se muestra su código)
  codigo: |                            # alternativa a ej para código corto no ejecutable
    let a = 1;
  opciones: ["…", "…", "…", "…"]      # tipos opcion/predice/no-compila
  correcta: 1
  respuestas: ["42"]                   # tipo respuesta
  solucion: |                          # explicación detallada (markdown): por qué es la correcta y por qué fallan las demás
    …
  etiquetas: ["E0499", "&mut"]
```
Cada mundo aporta **30–60 preguntas** (más fáciles en los primeros mundos). Todo código y toda salida/error citados, verificados como en §4.

### 7.2 Problemas de código — `web/problemas/<slug>/`

```
enunciado.md     # frontmatter: titulo, mundo, tema, dificultad; cuerpo: enunciado, ejemplos de entrada/salida, pistas (plegadas)
inicio.rs        # lo que ve el alumno en el editor (firma de la función + todo!())
solucion.rs      # solución de referencia (mismo contenido que inicio.rs pero completo)
pruebas.rs       # fn main() que llama a la función del alumno con casos y imprime «OK: …» / «FALLO: …» y al final «TODOS LOS CASOS SUPERADOS»
```
La plataforma concatena el código del alumno + `pruebas.rs` y lo ejecuta (Playground, con aviso; o wasm precompilado si existe un compilador local en el futuro). Superado = aparece «TODOS LOS CASOS SUPERADOS». Verificar antes: `solucion.rs + pruebas.rs` compila a wasm32-wasip1 y pasa; `inicio.rs + pruebas.rs` compila (con `todo!()`) y falla. Cada mundo (desde el 03) aporta **3–8 problemas** de dificultad creciente.

### 7.3 Listas, quizzes y juegos — `web/contenido/listas.yaml`, `web/contenido/quizzes.yaml`
Las escribe el coordinador al final con los ids de preguntas/problemas. Los juegos los programa la plataforma a partir de la biblioteca (p. ej. «¿Compila? en 60 segundos» con las preguntas `no-compila`).
