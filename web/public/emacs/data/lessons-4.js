export const lessons4 = [
  {
    id: 'm12-l01', module: 'm12', n: 1, title: 'package.el, MELPA y use-package', min: 9, level: 'Intermedio',
    obj: 'Instalar paquetes desde repositorios y declarar la configuración de forma reproducible.',
    frase: '`package.el` instala; MELPA provee; `use-package` declara intención, no solo código suelto.',
    idea: [
      '`package-initialize` activa el sistema de paquetes. `M-x package-refresh-contents` actualiza el índice; `M-x package-install` instala; `M-x package-list-packages` muestra el catálogo. `package-archives` define repositorios; MELPA es el más grande, GNU ELPA el oficial, NonGNU ELPA el intermedio.',
      '`use-package` es una macro de configuración: agrupa `:ensure`, `:init`, `:config`, `:bind`, `:hook`, `:custom`, `:defer`. No instala por sí sola (necesita `:ensure t` y `package.el` o straight/elpaca), pero hace que la configuración sea legible y diferible.'
    ],
    code: { lang: 'elisp', src: '(require \'package)\n(add-to-list \'package-archives\n  \'("melpa" . "https://melpa.org/packages/") t)\n(package-initialize)\n\n(use-package magit\n  :ensure t\n  :bind ("C-x g" . magit-status))\n\n(use-package org\n  :ensure t\n  :custom (org-agenda-files \'("~/org/inbox.org")))', cap: 'package.el + use-package.' },
    under: [
      '`package.el` descarga tarballs, compila y activa paquetes en `package-user-dir` (normalmente `~/.emacs.d/elpa`). `package-selected-packages` registra lo instalado; `package-autoremove` limpia dependencias huérfanas. `package-check-signature` verifica firmas GPG.',
      '`use-package` expande a `require`/`autoload` y configuración. `:defer t` evita cargar hasta que se use; `:hook` agrega a un hook; `:bind` crea autoloads. `use-package-always-ensure` hace `:ensure t` implícito. `C-h f use-package` lista todas las palabras clave.'
    ],
    trap: { bad: 'Poner `(require \'paquete)` de todo al inicio y arrancar Emacs en 8 segundos.', good: 'Usar `:defer`, `:hook` y autoloads para cargar solo cuando hace falta.', why: 'Cada `require` carga código y dependencias; diferir reduce el tiempo de arranque.' },
    q: [
      { q: '¿Qué hace `M-x package-refresh-contents`?', o: ['Actualiza el índice de paquetes', 'Instala Emacs', 'Borra paquetes', 'Compila'], a: 0, why: 'Descarga la lista de paquetes disponibles de los archivos configurados.' },
      { q: '¿Qué hace `:defer t` en use-package?', o: ['Difiere la carga', 'Instala', 'Borra', 'Compila'], a: 0, why: 'Evita cargar el paquete hasta que se use.' }
    ],
    ex: { p: 'Agregá MELPA, instalá `magit` con `use-package`, enlazá `C-x g` y verificá que no se cargue al inicio.', start: '(use-package magit\n  :ensure t\n  :bind ("C-x g" . magit-status))', sol: '(require \'package)\n(add-to-list \'package-archives\n  \'("melpa" . "https://melpa.org/packages/") t)\n(package-initialize)\n(use-package magit\n  :ensure t\n  :bind ("C-x g" . magit-status))', hint: '`:bind` crea un autoload; Magit se carga al pulsar `C-x g`.' },
    cards: [['¿Qué es MELPA?', 'Un repositorio grande de paquetes de Emacs.'], ['¿Qué hace `:ensure t`?', 'Instala el paquete si no está instalado.']],
    tags: ['paquetes', 'use-package', 'comandos']
  },
  {
    id: 'm12-l02', module: 'm12', n: 2, title: 'Straight, Elpaca y reproducibilidad', min: 9, level: 'Avanzado',
    obj: 'Gestionar paquetes desde Git con versiones fijas y configuración reproducible.',
    frase: '`straight.el` clona repos Git y fija commits; `elpaca` hace lo mismo de forma asíncrona. La reproducibilidad es el objetivo.',
    idea: [
      '`straight.el` instala paquetes clonando sus repositorios en `straight/repos` y registra versiones en `straight/versions/default.el`. `straight-use-package` reemplaza a `use-package` (o se integra). Podés fijar un commit con `:pin`, usar `:recipe` para recetas personalizadas y `straight-freeze-versions` para guardar el lockfile.',
      '`elpaca` es una alternativa moderna que instala de forma asíncrona y no bloquea el arranque. `elpaca` usa `elpaca-...` y un lockfile. `package.el` es suficiente para muchos; straight/elpaca son para quienes quieren control de versiones y recetas.'
    ],
    code: { lang: 'elisp', src: '(straight-use-package \'magit)\n(straight-use-package\n \'(foo :host github :repo "user/foo" :pin "abc123"))\n(straight-freeze-versions)\n;; M-x straight-freeze-versions', cap: 'Straight con pin de commit.' },
    under: [
      'Straight construye un grafo de dependencias y activa paquetes desde el repositorio clonado, no desde un tarball. Los builds se hacen con `straight-build-compile`; los archivos compilados van a `straight/build`. El lockfile permite recrear el estado exacto.',
      'Elpaca usa un modelo de “recipe” y un lockfile, y ejecuta la instalación en un proceso aparte para no bloquear el inicio. La contrapartida es más complejidad y menos madurez que `package.el`. Si no necesitás pins, no lo uses.'
    ],
    trap: { bad: 'Clonar paquetes desde `master` sin fijar commits y romper la configuración en cada actualización.', good: 'Fijar versiones en el lockfile (`straight-freeze-versions` o el lock de Elpaca) y actualizar deliberadamente.', why: 'El código en master cambia; sin pin, tu configuración no es reproducible.' },
    q: [
      { q: '¿Qué hace `straight-freeze-versions`?', o: ['Guarda las versiones actuales en un lockfile', 'Borra paquetes', 'Compila', 'Hace commit'], a: 0, why: 'Registra los commits actuales para reproducir el entorno.' },
      { q: '¿Qué ventaja tiene Elpaca?', o: ['Instalación asíncrona', 'Más paquetes', 'Mejor tema', 'Compilación nativa'], a: 0, why: 'Instala sin bloquear el arranque.' }
    ],
    ex: { p: 'Instalá un paquete con straight fijando un commit y guardá el lockfile.', start: '(straight-use-package\n \'(foo :host github :repo "user/foo" :pin "abc123"))\n(straight-freeze-versions)', sol: '(straight-use-package\n \'(foo :host github :repo "user/foo" :pin "abc123"))\n(straight-freeze-versions)', hint: 'El repo y el commit deben existir; `:pin` acepta un hash o un tag.' },
    cards: [['¿Qué es un lockfile?', 'Un archivo que fija versiones/commits para reproducir el entorno.'], ['¿Qué hace `:pin`?', 'Fija un paquete a un commit o tag concreto.']],
    tags: ['paquetes', 'straight', 'experto']
  },
  {
    id: 'm12-l03', module: 'm12', n: 3, title: 'early-init, init y lexical-binding', min: 9, level: 'Avanzado',
    obj: 'Organizar la configuración en archivos con el orden correcto y activar scope léxico.',
    frase: '`early-init.el` corre antes de la UI y los paquetes; `init.el` corre después; `lexical-binding` debe estar en la primera línea.',
    idea: [
      '`early-init.el` se carga antes de inicializar el sistema de paquetes y la interfaz. Es el lugar para desactivar `menu-bar-mode`, `tool-bar-mode`, `scroll-bar-mode`, ajustar `gc-cons-threshold` y `package-enable-at-startup`. `init.el` se carga después y es el lugar de la configuración normal.',
      'La primera línea `;; -*- lexical-binding: t; -*-` activa scope léxico en ese archivo. Sin ella, las closures capturan dinámicamente y muchos paquetes modernos se comportan mal. Es la recomendación por defecto en todo archivo `.el` nuevo.'
    ],
    code: { lang: 'elisp', src: ';; early-init.el\n(setq gc-cons-threshold (* 100 1024 1024))\n(menu-bar-mode -1)\n(tool-bar-mode -1)\n(scroll-bar-mode -1)\n\n;; init.el\n;; -*- lexical-binding: t; -*-\n(setq inhibit-startup-screen t)\n(add-hook \'prog-mode-hook #\'display-line-numbers-mode)', cap: 'Separación early-init/init y lexical-binding.' },
    under: [
      'El orden de carga: `early-init.el`, inicialización de paquetes, `init.el`, `after-init-hook`, `emacs-startup-hook`. `early-init` es el único lugar donde podés evitar que se carguen la UI y los paquetes del sitio. `package-enable-at-startup` se debe poner ahí, no en `init.el`.',
      '`lexical-binding` es una variable de archivo, no global. Se activa con la línea de modo o con `-*- lexical-binding: t; -*-`. Afecta cómo se compilan las closures y cómo se resuelven las variables libres. Es la diferencia entre un closure que funciona y uno que captura la variable equivocada.'
    ],
    trap: { bad: 'Poner `(setq package-enable-at-startup nil)` en `init.el` y preguntarte por qué los paquetes ya se cargaron.', good: 'Ponerlo en `early-init.el`, que corre antes de la inicialización de paquetes.', why: 'Cuando `init.el` corre, `package.el` ya hizo su trabajo.' },
    q: [
      { q: '¿Qué archivo corre primero?', o: ['`early-init.el`', '`init.el`', '`after-init.el`', '`custom.el`'], a: 0, why: '`early-init.el` se carga antes de la UI y los paquetes.' },
      { q: '¿Qué activa `lexical-binding: t`?', o: ['Scope léxico en el archivo', 'Compilación nativa', 'Modo modal', 'Red'], a: 0, why: 'Hace que las closures capturen variables léxicamente.' }
    ],
    ex: { p: 'Creá un `early-init.el` que desactive la toolbar y un `init.el` con `lexical-binding` y números de línea en prog-mode.', start: ';; early-init.el\n(tool-bar-mode -1)\n\n;; init.el\n;; -*- lexical-binding: t; -*-\n(add-hook \'prog-mode-hook #\'display-line-numbers-mode)', sol: ';; early-init.el\n(tool-bar-mode -1)\n(menu-bar-mode -1)\n\n;; init.el\n;; -*- lexical-binding: t; -*-\n(add-hook \'prog-mode-hook #\'display-line-numbers-mode)', hint: '`emacs -Q` ignora ambos; usá `emacs --debug-init` para ver errores.' },
    cards: [['¿Qué va en early-init?', 'UI, GC, package-enable-at-startup, cosas previas a paquetes.'], ['¿Qué hace `--debug-init`?', 'Muestra el backtrace si init falla.']],
    tags: ['configuracion', 'early-init', 'experto']
  },
  {
    id: 'm12-l04', module: 'm12', n: 4, title: 'Byte-compile y native-comp', min: 9, level: 'Avanzado',
    obj: 'Compilar Elisp a bytecode y a código nativo, y leer los warnings.',
    frase: 'El bytecode se interpreta más rápido; native-comp lo compila a máquina. Los warnings te avisan de errores futuros.',
    idea: [
      '`byte-compile-file` compila un `.el` a `.elc`; `batch-byte-compile` compila en lote; `byte-compile-directory` hace un directorio. Emacs carga `.elc` si es más nuevo que `.el`. Los warnings (`free variable`, `unused lexical argument`, `docstring`) no impiden cargar, pero suelen indicar bugs.',
      '`native-comp` (Emacs 28+) compila a código nativo con libgccjit. `native-compile` compila una función/archivo; `native-comp-available-p` dice si está disponible; `native-comp-jit-compilation` controla la compilación en segundo plano. Los `.eln` se guardan en `native-comp-eln-load-path`.'
    ],
    code: { lang: 'elisp', src: '(byte-compile-file "~/mi-paquete.el")\n(batch-byte-compile)\n(native-compile "~/mi-paquete.el")\n(native-comp-available-p)\n;; M-x byte-compile-file\n;; M-x native-compile-async', cap: 'Bytecode y native-comp.' },
    under: [
      'El bytecode es un vector de instrucciones para la VM de Emacs; se interpreta más rápido que el árbol Lisp, pero no es código máquina. Native-comp genera código máquina y puede ser 2–5× más rápido en cargas intensivas, a costa de compilación y memoria.',
      'Los warnings de byte-compile son estáticos: no ejecutan el código. `(setq byte-compile-error-on-warn t)` los convierte en errores para CI. `checkdoc` revisa docstrings y convenciones. `package-lint` valida un paquete antes de publicarlo.'
    ],
    trap: { bad: 'Ignorar warnings de “free variable” y descubrir en runtime que la variable no existe.', good: 'Compilar con `byte-compile-error-on-warn` en CI y corregir cada warning.', why: 'Una variable libre suele ser un typo o una dependencia no declarada.' },
    q: [
      { q: '¿Qué genera `byte-compile-file`?', o: ['Un `.elc`', 'Un `.eln`', 'Un `.so`', 'Un `.md`'], a: 0, why: 'Compila a bytecode en un archivo `.elc`.' },
      { q: '¿Qué hace native-comp?', o: ['Compila a código máquina', 'Solo formatea', 'Gestiona paquetes', 'Hace commit'], a: 0, why: 'Genera código nativo (`.eln`) usando libgccjit.' }
    ],
    ex: { p: 'Compilá un archivo `.el` con `byte-compile-file` y revisá los warnings en `*Compile-Log*`.', start: '(byte-compile-file "~/prueba.el")', sol: '(byte-compile-file "~/prueba.el")\n;; C-h e para ver *Compile-Log*\n;; corrige los warnings', hint: 'Si el archivo no existe, creá uno con una función simple.' },
    cards: [['¿Qué diferencia hay entre `.elc` y `.eln`?', '`.elc` es bytecode; `.eln` es código nativo.'], ['¿Qué comando compila en lote?', '`batch-byte-compile`.']],
    tags: ['paquetes', 'compilacion', 'experto']
  },
  {
    id: 'm12-l05', module: 'm12', n: 5, title: 'Publicar un paquete', min: 9, level: 'Experto',
    obj: 'Preparar un paquete para MELPA: metadatos, docstrings, tests y licencia.',
    frase: 'Un paquete publicable tiene header correcto, dependencias declaradas, docstrings, tests y una licencia.',
    idea: [
      'El header de un paquete incluye `;;; foo.el --- Descripción -*- lexical-binding: t; -*-`, `Author`, `Version`, `Package-Requires`, `Keywords` y `URL`. El archivo debe terminar con `(provide \'foo)` y `;;; foo.el ends here`. `package-lint` y `checkdoc` validan.',
      'Para MELPA, el paquete se envía como PR al repo `melpa/melpa` con una recipe. Para GNU ELPA, el proceso es más estricto (copyright assignment). Para uso personal, alcanza con `package-install-file`. Los tests con ERT y un README claro ayudan a que lo acepten.'
    ],
    code: { lang: 'elisp', src: ';;; mi-paquete.el --- Hace algo útil -*- lexical-binding: t; -*-\n;; Author: Tu Nombre <tu@email>\n;; Version: 0.1.0\n;; Package-Requires: ((emacs "29.1"))\n;; Keywords: tools\n;; URL: https://example.com/mi-paquete\n\n;;; Commentary:\n;; Descripción.\n\n;;; Code:\n(defun mi-paquete-hola () (interactive) (message "hola"))\n(provide \'mi-paquete)\n;;; mi-paquete.el ends here', cap: 'Header mínimo de paquete.' },
    under: [
      'MELPA construye cada paquete desde su repositorio y ejecuta `package-build`. La recipe indica `:fetcher`, `:repo`, `:files`. Si el paquete depende de otro, `Package-Requires` lo declara. `package-lint` verifica el header y las convenciones.',
      'GNU ELPA requiere asignación de copyright a la FSF para contribuciones significativas. NonGNU ELPA es más flexible. Para un paquete interno, no necesitás publicar: `package-install-file` o straight con `:local-repo` alcanzan.'
    ],
    trap: { bad: 'Publicar un paquete sin `Package-Requires` y romper en Emacs viejos o sin dependencias.', good: 'Declarar la versión mínima de Emacs y cada dependencia; probar con `emacs -Q` y `package-lint`.', why: 'El usuario instala el paquete en un entorno distinto; las dependencias deben ser explícitas.' },
    q: [
      { q: '¿Qué declara `Package-Requires`?', o: ['Dependencias y versión mínima', 'El tema', 'La licencia', 'Los tests'], a: 0, why: 'Lista paquetes y versión de Emacs necesarios.' },
      { q: '¿Qué herramienta valida el header?', o: ['`package-lint`', 'Magit', 'Org', 'Corfu'], a: 0, why: '`package-lint` revisa metadatos y convenciones.' }
    ],
    ex: { p: 'Creá un paquete mínimo con header, `provide` y un comando; compilalo y pasale `package-lint`.', start: ';;; mi-paquete.el --- Demo -*- lexical-binding: t; -*-\n;; Version: 0.1.0\n;; Package-Requires: ((emacs "29.1"))\n;;; Code:\n(defun mi-paquete-hola () (interactive) (message "hola"))\n(provide \'mi-paquete)\n;;; mi-paquete.el ends here', sol: ';;; mi-paquete.el --- Demo -*- lexical-binding: t; -*-\n;; Author: Tu Nombre\n;; Version: 0.1.0\n;; Package-Requires: ((emacs "29.1"))\n;; Keywords: tools\n;;; Commentary:\n;; Demo.\n;;; Code:\n(defun mi-paquete-hola () (interactive) (message "hola"))\n(provide \'mi-paquete)\n;;; mi-paquete.el ends here', hint: '`M-x package-lint-current-buffer` y `M-x checkdoc`.' },
    cards: [['¿Qué función debe tener todo paquete?', '`(provide \'nombre)`.'], ['¿Qué revisa `checkdoc`?', 'Docstrings y convenciones de documentación.']],
    tags: ['paquetes', 'publicacion', 'experto']
  },
  {
    id: 'm13-l01', module: 'm13', n: 1, title: 'Startup y benchmark', min: 8, level: 'Avanzado',
    obj: 'Medir el arranque, encontrar cuellos de botella y comparar alternativas con benchmark.',
    frase: 'No optimices por intuición: `emacs-init-time` y `benchmark-run` te dan números.',
    idea: [
      '`(emacs-init-time)` devuelve el tiempo de arranque. `emacs --debug-init` muestra errores; `emacs --profile-init`? No existe: usá `M-x profiler-start` durante el arranque o `esup`/`benchmark-init` para un informe por paquete. `M-x load-file` y `M-x eval-buffer` permiten probar cambios sin reiniciar.',
      '`benchmark-run` ejecuta una forma N veces y devuelve tiempo, GCs y tiempo de GC. `benchmark-run-compiled` usa bytecode. `M-x benchmark` (paquete) compara expresiones. Para medir una función, repetila muchas veces y restá el overhead.'
    ],
    code: { lang: 'elisp', src: '(emacs-init-time)\n(benchmark-run 1000 (+ 1 2))\n(benchmark-run-compiled 1000 (cl-loop for i below 1000 sum i))\n;; M-x profiler-start RET cpu RET\n;; ... usar Emacs ...\n;; M-x profiler-report', cap: 'Medir arranque y operaciones.' },
    under: [
      'El arranque carga `early-init`, paquetes, `init` y hooks. Los mayores costos suelen ser `require` de paquetes, `package-initialize`, fuentes y temas. `use-package :defer` y autoloads reducen el trabajo inicial.',
      '`benchmark-run` devuelve `(segundos gc-count gc-time)`. Si el tiempo de GC es alto, ajustá `gc-cons-threshold` o reducí allocations. `profiler-start` muestrea el CPU y genera un árbol de funciones; `profiler-report` lo muestra.'
    ],
    trap: { bad: 'Subir `gc-cons-threshold` a 1 GB y nunca bajarlo, consumiendo memoria sin control.', good: 'Subirlo durante el arranque y restaurarlo después con `emacs-startup-hook`.', why: 'Un umbral alto reduce GCs pero aumenta el pico de memoria; no es gratis.' },
    q: [
      { q: '¿Qué mide `emacs-init-time`?', o: ['El tiempo de arranque', 'El uso de CPU', 'La memoria', 'Los buffers'], a: 0, why: 'Devuelve el tiempo que tardó Emacs en inicializarse.' },
      { q: '¿Qué devuelve `benchmark-run`?', o: ['Tiempo, GCs y tiempo de GC', 'Solo tiempo', 'Memoria', 'Un buffer'], a: 0, why: 'Devuelve una lista con segundos, número de GCs y tiempo de GC.' }
    ],
    ex: { p: 'Medí `(emacs-init-time)`, compará un loop con `benchmark-run` y mirá el profiler.', start: '(emacs-init-time)\n(benchmark-run 10000 (cl-loop for i below 1000 sum i))', sol: '(emacs-init-time)\n(benchmark-run 10000 (cl-loop for i below 1000 sum i))\n;; M-x profiler-start RET cpu RET\n;; M-x profiler-report', hint: 'Si el tiempo de GC domina, probá `(setq gc-cons-threshold (* 64 1024 1024))`.' },
    cards: [['¿Qué hace `benchmark-run-compiled`?', 'Mide código bytecode, no interpretado.'], ['¿Dónde ves el informe del profiler?', 'En `*CPU-Profiler-Report*` tras `profiler-report`.']],
    tags: ['rendimiento', 'benchmark', 'experto']
  },
  {
    id: 'm13-l02', module: 'm13', n: 2, title: 'Profiler, memoria y GC', min: 9, level: 'Experto',
    obj: 'Encontrar funciones costosas, entender el GC y reducir allocations.',
    frase: 'El profiler muestrea; el GC libera; la memoria se mide. Optimizar sin medir es adivinar.',
    idea: [
      '`profiler-start` con `cpu`, `mem` o `cpu+mem`; `profiler-report` muestra un árbol con porcentajes; `profiler-report-find-entry` busca una función. `M-x profiler-stop` termina. Para el arranque, `M-x profiler-start` antes de cargar y `profiler-report` después.',
      '`garbage-collect` fuerza una recolección y devuelve estadísticas. `memory-info` da total/free. `gc-cons-threshold` controla cuándo se dispara; `gc-cons-percentage` lo ajusta por tamaño. `M-x memory-report` (Emacs 29+) muestra uso por buffer y variable.'
    ],
    code: { lang: 'elisp', src: '(profiler-start \'cpu)\n;; usar Emacs\n(profiler-stop)\n(profiler-report)\n\n(garbage-collect)\n(memory-info)\n(setq gc-cons-threshold (* 64 1024 1024))', cap: 'Profiler, GC y memoria.' },
    under: [
      'El profiler de CPU muestrea el stack cada cierto intervalo; no instrumenta cada función. Por eso el costo es bajo y el resultado es estadístico. `profiler-report` agrega por función y muestra el porcentaje del tiempo total.',
      'El GC de Emacs es mark-and-sweep y no compacta. Los objetos viven hasta que no hay referencias. Las allocations frecuentes (strings temporales, listas) disparan GCs. Reutilizar buffers, usar `StringBuilder`/`concat` con cuidado y evitar `format` en bucles ayuda.'
    ],
    trap: { bad: 'Optimizar una función que no aparece en el profiler porque el costo real está en otra parte.', good: 'Mirar el informe completo, ordenar por porcentaje y medir antes/después con `benchmark-run`.', why: 'La intuición sobre el cuello de botella suele ser incorrecta.' },
    q: [
      { q: '¿Qué hace `profiler-start`?', o: ['Empieza a muestrear', 'Borra', 'Compila', 'Hace commit'], a: 0, why: 'Inicia el profiler de CPU/memoria.' },
      { q: '¿Qué controla `gc-cons-threshold`?', o: ['Cuándo se dispara el GC', 'El tema', 'Las teclas', 'Los buffers'], a: 0, why: 'Es el número de bytes asignados antes de recolectar.' }
    ],
    ex: { p: 'Perfilá una operación lenta, encontrá la función más costosa y medí una mejora con `benchmark-run`.', start: '(profiler-start \'cpu)\n;; ejecuta la operación\n(profiler-stop)\n(profiler-report)', sol: '(profiler-start \'cpu)\n;; ejecuta la operación\n(profiler-stop)\n(profiler-report)\n;; benchmark-run antes/después', hint: 'Si el reporte es enorme, `profiler-report-find-entry` busca una función.' },
    cards: [['¿Qué tipos de profiler hay?', 'CPU, memoria y cpu+mem.'], ['¿Qué devuelve `garbage-collect`?', 'Estadísticas de la recolección.']],
    tags: ['rendimiento', 'profiler', 'gc', 'experto']
  },
  {
    id: 'm13-l03', module: 'm13', n: 3, title: 'ERT, buttercup y CI', min: 9, level: 'Experto',
    obj: 'Escribir tests de Elisp, correrlos en batch y integrarlos en CI.',
    frase: '`ert-deftest` define un test; `should` afirma; `ert-run-tests-batch-and-exit` lo corre sin abrir Emacs.',
    idea: [
      '`ert-deftest nombre ()` define un test; `(should cond)` afirma; `(should-error form)` espera un error; `(should-not cond)` niega. `M-x ert-run-tests-buffer` corre los del buffer; `M-x ert` muestra la lista. `ert-run-tests-batch-and-exit` es para CI.',
      '`buttercup` es una alternativa con `describe`/`it` y spies. `ert` viene incluido y es suficiente para la mayoría. Para CI: `emacs -Q --batch -l ert -l mi-paquete.el -f ert-run-tests-batch-and-exit`. Si un test falla, el exit code es distinto de 0.'
    ],
    code: { lang: 'elisp', src: '(require \'ert)\n\n(ert-deftest mi-suma ()\n  (should (= (+ 1 2) 3)))\n\n(ert-deftest mi-error ()\n  (should-error (car 42)))\n\n;; batch:\n;; emacs -Q --batch -l ert -l mi-paquete.el \\\n;;   -f ert-run-tests-batch-and-exit', cap: 'Tests con ERT y ejecución batch.' },
    under: [
      'ERT agrupa tests en suites y selectores. `ert-run-tests` acepta un selector (`t` para todos, `"regexp"` para nombres). Los resultados se reportan con tiempos y backtraces. `ert-results` permite re-ejecutar un test con `r`.',
      'En CI, `--batch` no carga la UI ni tu init; `-Q` evita configuración. Cargá el paquete y sus dependencias explícitamente. `ert-run-tests-batch-and-exit` sale con 0 si todo pasa y 1 si algo falla, lo que GitHub Actions interpreta.'
    ],
    trap: { bad: 'Correr tests solo en tu Emacs interactivo y no en `-Q --batch`, donde faltan paquetes o variables.', good: 'Probar en batch con las dependencias declaradas y un `emacs -Q` limpio.', why: 'El entorno interactivo tiene estado que el CI no tiene; los tests deben ser reproducibles.' },
    q: [
      { q: '¿Qué define un test en ERT?', o: ['`ert-deftest`', '`defun`', '`define-key`', '`add-hook`'], a: 0, why: '`ert-deftest` declara un test y su body.' },
      { q: '¿Qué corre los tests en batch?', o: ['`ert-run-tests-batch-and-exit`', '`ert-run-tests-buffer`', '`M-x ert`', '`compile`'], a: 0, why: 'Es la función para ejecutar sin UI y salir con código de estado.' }
    ],
    ex: { p: 'Escribí un test para una función `sumar` y corrélo con `ert-run-tests-buffer` y en batch.', start: '(require \'ert)\n(defun sumar (a b) (+ a b))\n(ert-deftest sumar-test () (should (= (sumar 2 3) 5)))', sol: '(require \'ert)\n(defun sumar (a b) (+ a b))\n(ert-deftest sumar-test () (should (= (sumar 2 3) 5)))\n;; M-x ert-run-tests-buffer\n;; emacs -Q --batch -l ert -l test.el -f ert-run-tests-batch-and-exit', hint: 'En batch, cargá el archivo que define `sumar` antes de los tests.' },
    cards: [['¿Qué hace `should-error`?', 'Afirma que una forma lanza un error.'], ['¿Qué comando corre tests en batch y sale?', '`ert-run-tests-batch-and-exit`.']],
    tags: ['testing', 'ert', 'experto']
  },
  {
    id: 'm13-l04', module: 'm13', n: 4, title: 'Debugging: edebug y trace', min: 9, level: 'Experto',
    obj: 'Instrumentar funciones, entrar paso a paso y rastrear llamadas.',
    frase: '`C-u C-M-x` instrumenta una función con Edebug; `M-x trace-function` muestra cada llamada y su resultado.',
    idea: [
      'Poné el cursor dentro de una `defun` y pulsá `C-u C-M-x` (`eval-defun` con prefijo) para instrumentarla con Edebug. Al llamarla, Edebug se detiene en cada forma: `SPC` paso, `n` siguiente, `i` entrar, `o` salir, `g` go, `q` salir. `edebug-defun` es la función.',
      '`M-x trace-function` pide una función y un buffer; cada llamada y retorno se registra. `M-x untrace-function` lo desactiva. `debug-on-error` entra al depurador cuando hay un error; `M-x toggle-debug-on-error`. `debug-on-entry` rompe al entrar a una función.'
    ],
    code: { lang: 'elisp', src: '(defun mi-funcion (x)\n  (let ((y (* x 2)))\n    (+ y 1)))\n\n;; C-u C-M-x dentro de la defun\n;; SPC / n / i / o / g / q\n\n(trace-function #\'mi-funcion)\n(untrace-function #\'mi-funcion)', cap: 'Edebug y trace.' },
    under: [
      'Edebug instrumenta la forma y guarda información de cada paso. Podés evaluar expresiones en el contexto (`e`), ver la profundidad (`d`) y saltar (`g`). Es más lento que el código normal, pero invaluable para entender el flujo.',
      '`trace-function` usa `advice` para imprimir entrada/salida. `debug-on-error` muestra el backtrace y permite evaluar en el frame. `M-x debug-on-entry` es útil para funciones que se llaman desde muchos lados. `backtrace` muestra la pila actual.'
    ],
    trap: { bad: 'Instrumentar una función que se llama miles de veces por segundo y congelar Emacs en Edebug.', good: 'Usar `trace-function` con un filtro o instrumentar solo el caso que importa; desactivar Edebug al terminar.', why: 'Edebug se detiene en cada paso; en un hot path es inusable.' },
    q: [
      { q: '¿Qué hace `C-u C-M-x` dentro de una defun?', o: ['Instrumenta con Edebug', 'Compila', 'Borra', 'Hace commit'], a: 0, why: 'Evalúa la defun en modo Edebug.' },
      { q: '¿Qué hace `trace-function`?', o: ['Registra llamadas y retornos', 'Borra la función', 'Compila', 'Abre shell'], a: 0, why: 'Traza las llamadas a la función en un buffer.' }
    ],
    ex: { p: 'Instrumentá una función con Edebug, recorréla con `SPC` y después traceala con `trace-function`.', start: '(defun doble (x) (* x 2))\n;; C-u C-M-x\n;; M-x trace-function RET doble RET\n(doble 21)', sol: '(defun doble (x) (* x 2))\n;; C-u C-M-x\n;; SPC SPC g\n;; M-x trace-function RET doble RET\n(doble 21)', hint: 'Si Edebug no se activa, el cursor debe estar dentro de la defun al pulsar `C-u C-M-x`.' },
    cards: [['¿Qué hace `debug-on-error`?', 'Entra al depurador cuando ocurre un error.'], ['¿Qué tecla sale de Edebug?', '`q` (`top-level`).']],
    tags: ['debugging', 'edebug', 'experto']
  },
  {
    id: 'm13-l05', module: 'm13', n: 5, title: 'Bytecode, dump y contribuir', min: 10, level: 'Experto',
    obj: 'Entender qué ejecuta Emacs, cómo se empaqueta y cómo contribuir sin romper nada.',
    frase: 'Emacs ejecuta bytecode o código nativo; el dump precarga el estado; contribuir empieza por reproducir y leer el código.',
    idea: [
      'Una función compilada es un objeto bytecode; `(symbol-function \'foo)` lo muestra; `(disassemble \'foo)` lo desensambla. `byte-code-function-p` comprueba. Native-comp genera `.eln` y `comp-function-p`. `dump-emacs` crea una imagen precargada (usado al construir Emacs).',
      'Para contribuir: reproducir el bug con `emacs -Q`, leer el código en `lisp/`, escribir un test ERT, seguir `CONTRIBUTING.md`, firmar el copyright assignment si es para Emacs core, y enviar a `bug-gnu-emacs` o al tracker. Para paquetes, el flujo es PR en GitHub/MELPA.'
    ],
    code: { lang: 'elisp', src: '(symbol-function \'car)\n(byte-code-function-p (symbol-function \'car))\n(disassemble \'mi-funcion)\n(native-comp-available-p)\n;; M-x report-emacs-bug', cap: 'Inspeccionar bytecode y reportar.' },
    under: [
      'El bytecode usa un stack y un vector de constantes; `disassemble` muestra las instrucciones. Native-comp produce código máquina y se integra en el mismo modelo de funciones. `dump-emacs` guarda el estado en una imagen para arrancar más rápido, pero no es algo que uses a diario.',
      'Contribuir a Emacs core requiere entender el ciclo de release, las convenciones de estilo y los tests. `make check` corre la suite. `M-x report-emacs-bug` genera un correo con la versión, configuración y backtrace. Para paquetes, `package-lint`, `checkdoc` y ERT son el mínimo.'
    ],
    trap: { bad: 'Reportar un bug sin reproducirlo con `emacs -Q` y sin incluir el backtrace.', good: 'Reproducir con `-Q`, adjuntar versión, pasos, backtrace y configuración mínima.', why: 'Sin reproducción, el maintainer no puede confirmar ni arreglar el problema.' },
    q: [
      { q: '¿Qué muestra `disassemble`?', o: ['El bytecode de una función', 'El tema', 'Los buffers', 'El kill ring'], a: 0, why: 'Desensambla la función a instrucciones de bytecode.' },
      { q: '¿Qué comando reporta un bug de Emacs?', o: ['`M-x report-emacs-bug`', '`M-x compile`', '`M-x dired`', '`M-x shell`'], a: 0, why: 'Abre un buffer de correo con la información de diagnóstico.' }
    ],
    ex: { p: 'Desensamblá una función simple con `disassemble` y generá un reporte con `M-x report-emacs-bug`.', start: '(defun cuadrado (x) (* x x))\n(disassemble \'cuadrado)\n;; M-x report-emacs-bug', sol: '(defun cuadrado (x) (* x x))\n(disassemble \'cuadrado)\n;; M-x report-emacs-bug', hint: 'Si la función no está compilada, `disassemble` muestra la forma Lisp.' },
    cards: [['¿Qué es un `.eln`?', 'Código nativo compilado por native-comp.'], ['¿Qué comando abre un reporte de bug?', '`M-x report-emacs-bug`.']],
    tags: ['internals', 'bytecode', 'contribuir', 'experto']
  }
];
