export const lessons3 = [
  {
    id: 'm08-l01', module: 'm08', n: 1, title: 'Cómo completa Emacs: CAPF', min: 8, level: 'Avanzado',
    obj: 'Entender completion-at-point y las funciones que generan candidatos.',
    frase: 'Emacs no tiene “autocompletado”: tiene `completion-at-point-functions`, una lista de funciones que proponen candidatos.',
    idea: [
      '`C-M-i` (`completion-at-point`) llama a las funciones de `completion-at-point-functions` (CAPF) en orden. Cada una devuelve un rango y una lista de candidatos, o `nil` si no aplica. El modo mayor y los paquetes registran CAPFs para su lenguaje.',
      'La UI de completion (Vertico, Corfu, Company, etc.) no genera candidatos: los muestra y filtra. Entender esta separación evita confusiones: si no hay candidatos, el problema está en el CAPF o en el modo; si hay candidatos pero no ves la lista, el problema está en la UI.'
    ],
    code: { lang: 'elisp', src: '(add-hook \'completion-at-point-functions\n          #\'my-capf)\n\n(defun my-capf ()\n  (when (looking-back "\\\\bfoo" 3)\n    (list (match-beginning 0) (point)\n          \'("foobar" "foobaz"))))', cap: 'Un CAPF mínimo.' },
    under: [
      'El contrato de un CAPF es `(start end collection . props)`. `collection` puede ser una lista, un hash table o una función. `:exclusive` y `:annotation-function` controlan el comportamiento. La UI llama a `completion-in-region` con ese rango.',
      'Los CAPFs se combinan: si uno devuelve `nil`, se prueba el siguiente. `completion-at-point-functions` suele tener funciones del modo mayor y de paquetes como `cape`. `completion-styles` decide cómo se filtra (prefijo, parcial, flex, orderless).'
    ],
    trap: { bad: 'Instalar una UI de completion y esperar que aparezcan candidatos en un modo que no registra ningún CAPF.', good: 'Comprobar `C-M-i` y `M-x completion-at-point`; si no hay candidatos, agregar un CAPF (o `cape`) para ese modo.', why: 'La UI muestra candidatos; los candidatos los produce el CAPF.' },
    q: [
      { q: '¿Qué hace `C-M-i`?', o: ['Llama a completion-at-point', 'Compila', 'Cierra', 'Guarda'], a: 0, why: '`completion-at-point` pide candidatos a los CAPFs.' },
      { q: '¿Qué genera los candidatos?', o: ['La UI', 'Los CAPF', 'El tema', 'El minibuffer'], a: 1, why: 'Las funciones de `completion-at-point-functions` producen la colección.' }
    ],
    ex: { p: 'Escribí un CAPF que proponga `"alpha"` y `"beta"` después de `@@`.', start: '(defun my-capf ()\n  (when (looking-back "@@" 2)\n    (list (match-beginning 0) (point)\n          \'("alpha" "beta"))))\n(add-hook \'completion-at-point-functions #\'my-capf)', sol: '(defun my-capf ()\n  (when (looking-back "@@" 2)\n    (list (match-beginning 0) (point)\n          \'("alpha" "beta"))))\n(add-hook \'completion-at-point-functions #\'my-capf)\n;; escribe @@ y C-M-i', hint: 'El rango debe empezar después de `@@`; `match-beginning 0` ayuda.' },
    cards: [['¿Qué variable contiene los generadores de candidatos?', '`completion-at-point-functions`.'], ['¿Qué hace `completion-in-region`?', 'Muestra y filtra candidatos para un rango.']],
    tags: ['completion', 'capf', 'experto']
  },
  {
    id: 'm08-l02', module: 'm08', n: 2, title: 'Vertico, Consult, Marginalia y Orderless', min: 9, level: 'Intermedio',
    obj: 'Montar una completion vertical rápida y buscar en buffers, archivos y proyectos.',
    frase: 'Vertico muestra la lista en vertical; Consult añade comandos de búsqueda; Marginalia da contexto; Orderless filtra por palabras.',
    idea: [
      '`vertico-mode` reemplaza la lista de completion por una vertical, con contador y navegación. `marginalia-mode` agrega anotaciones (tipo, docstring, ruta) a los candidatos. `orderless` cambia el filtrado: podés escribir palabras en cualquier orden (`buf org` encuentra `org-buffer`).',
      '`consult` trae comandos que usan completion para explorar: `consult-buffer` (buffers, archivos recientes, bookmarks), `consult-line` (líneas del buffer), `consult-grep` (proyecto), `consult-imenu`, `consult-outline`, `consult-flymake`. `consult-async` permite búsquedas asíncronas.'
    ],
    code: { lang: 'elisp', src: '(use-package vertico :init (vertico-mode 1))\n(use-package marginalia :init (marginalia-mode 1))\n(use-package orderless\n  :custom (completion-styles \'(orderless basic)))\n(use-package consult\n  :bind (("C-x b" . consult-buffer)\n         ("C-s" . consult-line)))', cap: 'Configuración típica.' },
    under: [
      'Vertico es una UI de `completing-read`; no cambia los candidatos. Consult construye comandos con `completing-read` y fuentes; `consult-buffer` mezcla buffers, recientes y bookmarks. Marginalia anota con `marginalia-annotators`.',
      'Orderless divide el patrón por espacios y exige que cada parte coincida (en cualquier orden). `completion-styles` define la cadena: si `orderless` no encuentra, prueba `basic`. `consult-async` usa procesos para grep y limita resultados para no bloquear.'
    ],
    trap: { bad: 'Activar Vertico y Orderless sin ajustar `completion-styles` y concluir que “no filtra bien”.', good: 'Definir `(setq completion-styles \'(orderless basic))` y probar `M-x consult-buffer`.', why: 'La UI y el filtrado son capas distintas; sin el estilo correcto, Orderless no participa.' },
    q: [
      { q: '¿Qué hace `consult-buffer`?', o: ['Busca buffers, recientes y bookmarks', 'Borra buffers', 'Compila', 'Cierra Emacs'], a: 0, why: 'Es una fuente de completion que combina buffers y archivos recientes.' },
      { q: '¿Qué cambia Orderless?', o: ['El filtrado por palabras en cualquier orden', 'El tema', 'Las teclas', 'El kill ring'], a: 0, why: 'Orderless interpreta el patrón como palabras separadas por espacios.' }
    ],
    ex: { p: 'Activá Vertico, Marginalia y Orderless; probá `M-x consult-buffer` y `M-x consult-line`.', start: '(vertico-mode 1)\n(marginalia-mode 1)\n(setq completion-styles \'(orderless basic))\n;; M-x consult-buffer', sol: '(vertico-mode 1)\n(marginalia-mode 1)\n(setq completion-styles \'(orderless basic))\n;; M-x consult-buffer\n;; M-x consult-line', hint: 'Si no ves anotaciones, `marginalia-mode` no está activo o el candidato no tiene metadata.' },
    cards: [['¿Qué hace Marginalia?', 'Anota los candidatos con información contextual.'], ['¿Qué comando busca líneas en el buffer actual?', '`consult-line`.']],
    tags: ['completion', 'consult', 'comandos']
  },
  {
    id: 'm08-l03', module: 'm08', n: 3, title: 'Corfu, Cape y Company', min: 8, level: 'Intermedio',
    obj: 'Elegir completion en el buffer y combinarlo con fuentes de CAPF.',
    frase: 'Corfu muestra candidatos en el buffer; Cape aporta CAPFs; Company es la alternativa histórica.',
    idea: [
      '`corfu-mode` muestra una popup de completion en el punto, usando los CAPFs del modo. `corfu-popupinfo` añade documentación. `cape` proporciona CAPFs reutilizables: `cape-dabbrev`, `cape-file`, `cape-keyword`, `cape-symbol`, `cape-history`. Es la forma moderna de tener completion en casi cualquier buffer.',
      '`company-mode` es la generación anterior, muy estable y con muchos backends. Company y Corfu cumplen el mismo rol; no los actives juntos. Company tiene `company-capf`, `company-dabbrev`, `company-clang`, etc. Corfu se integra mejor con la completion nativa y es más liviano.'
    ],
    code: { lang: 'elisp', src: '(use-package corfu\n  :init (global-corfu-mode 1)\n  :custom (corfu-auto t))\n(use-package cape\n  :init (add-to-list \'completion-at-point-functions #\'cape-dabbrev)\n        (add-to-list \'completion-at-point-functions #\'cape-file))', cap: 'Corfu + Cape.' },
    under: [
      'Corfu se engancha a `completion-in-region-function` y muestra los candidatos en un child frame o en el buffer. `corfu-auto` dispara la completion al escribir; `corfu-cycle` permite circular; `corfu-separator` define el separador.',
      'Cape convierte fuentes en CAPFs. `cape-dabbrev` completa palabras del buffer; `cape-file` completa rutas; `cape-keyword` palabras clave del lenguaje. Podés combinarlos con `cape-super-capf` para unir varias fuentes.'
    ],
    trap: { bad: 'Activar Company y Corfu al mismo tiempo y pelear con dos popups.', good: 'Elegir uno: Corfu para el flujo nativo moderno, Company si ya tenés una configuración estable.', why: 'Ambos interceptan la completion en el punto; dos UIs compiten por el mismo espacio.' },
    q: [
      { q: '¿Qué hace Corfu?', o: ['Muestra completion en el buffer', 'Compila', 'Gestiona Git', 'Abre terminal'], a: 0, why: 'Corfu es una UI de completion en el punto.' },
      { q: '¿Qué aporta Cape?', o: ['CAPFs reutilizables', 'Un tema', 'Un shell', 'Un gestor de paquetes'], a: 0, why: 'Cape ofrece fuentes como dabbrev, file, keyword y symbol.' }
    ],
    ex: { p: 'Activá Corfu y agregá `cape-dabbrev` y `cape-file` a los CAPFs; probá completar una ruta.', start: '(global-corfu-mode 1)\n(add-to-list \'completion-at-point-functions #\'cape-dabbrev)\n(add-to-list \'completion-at-point-functions #\'cape-file)', sol: '(global-corfu-mode 1)\n(add-to-list \'completion-at-point-functions #\'cape-dabbrev)\n(add-to-list \'completion-at-point-functions #\'cape-file)\n;; escribe ~/ y C-M-i', hint: '`cape-file` completa rutas; `cape-dabbrev` completa palabras del buffer.' },
    cards: [['¿Qué UI es la alternativa moderna a Company?', 'Corfu.'], ['¿Qué CAPF completa rutas de archivo?', '`cape-file`.']],
    tags: ['completion', 'corfu', 'comandos']
  },
  {
    id: 'm08-l04', module: 'm08', n: 4, title: 'Embark: acciones sobre cualquier cosa', min: 8, level: 'Avanzado',
    obj: 'Usar Embark para actuar sobre el símbolo, buffer, archivo o candidato bajo el cursor.',
    frase: 'Embark convierte “lo que sea que esté en el punto” en un objeto con acciones: abrir, borrar, renombrar, exportar.',
    idea: [
      '`M-x embark-act` (por defecto `C-.`) abre un menú de acciones para el **target** bajo el cursor: un símbolo, una URL, un buffer, un archivo, una línea. `embark-dwim` (`M-.` en el mapa de embark) hace la acción más obvia: abrir un archivo, visitar una URL, encontrar una definición.',
      '`embark-export` convierte una lista de candidatos en un buffer permanente (por ejemplo, un `occur` o un `grep`); `embark-collect` la recolecta sin salir. `embark-bindings` muestra las teclas disponibles para el target. Es una capa de acciones, no un paquete de completion.'
    ],
    code: { lang: 'elisp', src: '(use-package embark\n  :bind (("C-." . embark-act)\n         ("C-;" . embark-dwim)))\n(use-package embark-consult\n  :after (embark consult))', cap: 'Embark con atajos típicos.' },
    under: [
      'Embark detecta el target con `embark-target-finders` y ofrece acciones con `embark-keymap-alist`. Cada tipo (symbol, file, url, buffer, line) tiene su keymap. `embark-act` muestra las teclas en un transient; `embark-dwim` ejecuta la principal.',
      'Embark se integra con Consult: `embark-export` sobre `consult-line` produce un `occur`; sobre `consult-grep`, un buffer de grep. `embark-collect` junta candidatos de varias fuentes en un buffer editable.'
    ],
    trap: { bad: 'Usar Embark sin saber qué target detectó y ejecutar la acción equivocada.', good: 'Pulsar `C-.` y leer el transient; `C-h` o `?` muestra las acciones disponibles.', why: 'Embark actúa sobre el target bajo el punto; si el target es otro, la acción cambia.' },
    q: [
      { q: '¿Qué hace `embark-act`?', o: ['Abre acciones para el target bajo el cursor', 'Borra el buffer', 'Compila', 'Cierra'], a: 0, why: 'Muestra un menú de acciones según el tipo de objeto.' },
      { q: '¿Qué hace `embark-export`?', o: ['Convierte candidatos en un buffer permanente', 'Exporta el proyecto', 'Guarda', 'Hace commit'], a: 0, why: 'Exporta la lista de candidatos a un buffer como occur o grep.' }
    ],
    ex: { p: 'Con `consult-line`, pulsá `C-.` sobre una línea y exportala con `embark-export`.', start: 'M-x consult-line RET patrón RET\nC-.\nembark-export', sol: 'M-x consult-line RET patrón RET\nC-.\n;; elige embark-export\n;; se abre un buffer occur', hint: 'El target de una línea de consult-line es la línea; las acciones incluyen export.' },
    cards: [['¿Qué hace `embark-dwim`?', 'Ejecuta la acción principal para el target.'], ['¿Qué paquete integra Embark con Consult?', '`embark-consult`.']],
    tags: ['completion', 'embark', 'experto']
  },
  {
    id: 'm08-l05', module: 'm08', n: 5, title: 'Which-key, transient y repeat-mode', min: 7, level: 'Intermedio',
    obj: 'Descubrir prefijos, construir menús y repetir comandos sin reescribir atajos.',
    frase: '`which-key` te muestra qué sigue después de un prefijo; `transient` construye menús como el de Magit; `repeat-mode` repite comandos.',
    idea: [
      '`which-key-mode` muestra en un popup las continuaciones posibles después de un prefijo (`C-x`, `C-c`, etc.). Es la forma de descubrir atajos sin memorizarlos. `which-key-setup-side-window-bottom` o `right` controlan dónde aparece.',
      '`transient` es la librería de menús efímeros que usa Magit. Define un `transient-define-prefix` con grupos de argumentos y acciones. `repeat-mode` permite repetir comandos como `C-x o` pulsando `o`; se configura con `repeat-map`.'
    ],
    code: { lang: 'elisp', src: '(which-key-mode 1)\n(repeat-mode 1)\n\n(transient-define-prefix my-menu ()\n  "Mi menú"\n  [["Acciones"\n    ("g" "Saludar" (lambda () (interactive) (message "hola")))\n    ("q" "Salir" transient-quit-one)]])', cap: 'Which-key, repeat y un transient mínimo.' },
    under: [
      'Which-key escucha `pre-command-hook` y, si el comando es un prefijo, programa un timer para mostrar las teclas. No bloquea: si seguís escribiendo, desaparece. `which-key-prefix-prefix` y `which-key-separator` personalizan la presentación.',
      'Transient define un estado con `transient-define-prefix`; las teclas pueden ser argumentos (`--`) o acciones. `transient-quit-one` cierra un nivel. Magit, Forge y muchos paquetes modernos lo usan; aprenderlo te permite leer sus menús.'
    ],
    trap: { bad: 'Crear decenas de atajos globales para comandos que usás una vez al mes.', good: 'Dejar que `which-key` te guíe y usar `M-x` o un transient para lo poco frecuente.', why: 'Los atajos son recursos escasos; sobrecargarlos hace que no recuerdes ninguno.' },
    q: [
      { q: '¿Qué hace `which-key-mode`?', o: ['Muestra las continuaciones de un prefijo', 'Borra buffers', 'Compila', 'Hace commit'], a: 0, why: 'Muestra un popup con las teclas posibles después de un prefijo.' },
      { q: '¿Qué librería usa Magit para sus menús?', o: ['Transient', 'Corfu', 'Vertico', 'Embark'], a: 0, why: 'Magit construye sus menús con `transient`.' }
    ],
    ex: { p: 'Activá which-key, pulsá `C-x` y esperá; después activá repeat-mode y repetí `C-x o` con `o`.', start: '(which-key-mode 1)\n(repeat-mode 1)\n;; C-x (espera)\n;; C-x o o o', sol: '(which-key-mode 1)\n(repeat-mode 1)\n;; C-x\n;; C-x o\n;; o o o', hint: '`repeat-mode` muestra un mensaje con las teclas de repetición disponibles.' },
    cards: [['¿Qué muestra which-key?', 'Las teclas que continúan un prefijo.'], ['¿Qué es un transient?', 'Un menú efímero de acciones y argumentos, como los de Magit.']],
    tags: ['completion', 'which-key', 'comandos']
  },
  {
    id: 'm09-l01', module: 'm09', n: 1, title: 'project.el, compile y grep', min: 8, level: 'Intermedio',
    obj: 'Trabajar por proyecto: archivos, compilación, búsqueda y errores.',
    frase: '`C-x p` es el prefijo de proyecto: archivos, compilación y grep sin escribir rutas.',
    idea: [
      '`C-x p p` (`project-switch-project`) cambia de proyecto; `C-x p f` (`project-find-file`) busca archivos; `C-x p d` abre Dired en la raíz; `C-x p c` (`project-compile`) compila; `C-x p g` (`project-find-regexp`) busca en el proyecto. `project.el` detecta la raíz por `.git`, `Cargo.toml`, `package.json`, etc.',
      '`M-x compile` ejecuta un comando y parsea la salida con `compilation-mode`; `M-g M-n` salta al siguiente error. `M-x recompile` repite. `M-x grep` busca en archivos; `M-x rgrep` recursivamente. `project-compile` usa la raíz del proyecto como directorio por defecto.'
    ],
    code: { lang: 'text', src: 'C-x p p   cambiar proyecto\nC-x p f   archivo\nC-x p c   compilar\nC-x p g   grep del proyecto\nM-g M-n   siguiente error\nM-x recompile', cap: 'Flujo de proyecto.' },
    under: [
      '`project-current` devuelve el proyecto actual; `project-root` su raíz. Los backends (`project-vc`, `project--vc`) detectan VCS; podés agregar marcadores con `project-vc-extra-root-markers`. `project-compile` enlaza `compile-command` al proyecto.',
      '`compilation-mode` usa `compilation-error-regexp-alist` para reconocer formatos. Si tu compilador imprime `archivo:línea:columna`, ya funciona; si no, agregá una regexp. `next-error` recorre los resultados y `C-u M-g M-n` va al anterior.'
    ],
    trap: { bad: 'Ejecutar `M-x compile` desde el directorio equivocado y obtener errores de rutas.', good: 'Usar `C-x p c` o `M-x compile` con `default-directory` en la raíz del proyecto.', why: 'El comando se ejecuta en `default-directory`; si es otro, las rutas relativas fallan.' },
    q: [
      { q: '¿Qué hace `C-x p f`?', o: ['Busca un archivo del proyecto', 'Compila', 'Cierra', 'Hace commit'], a: 0, why: '`project-find-file` busca archivos dentro de la raíz del proyecto.' },
      { q: '¿Qué comando salta al siguiente error de compilación?', o: ['`M-g M-n`', '`C-g`', '`C-x b`', '`M-x`'], a: 0, why: '`next-error` recorre los errores del buffer de compilación.' }
    ],
    ex: { p: 'En un repo, usá `C-x p p`, buscá un archivo con `C-x p f`, compilá con `C-x p c` y saltá un error.', start: 'C-x p p RET\nC-x p f\nC-x p c RET\nM-g M-n', sol: 'C-x p p RET\nC-x p f\nC-x p c RET\nM-g M-n', hint: 'Si `C-x p` no detecta el proyecto, agregá un marcador de raíz o usá `project-remember-project`.' },
    cards: [['¿Qué prefijo agrupa los comandos de proyecto?', '`C-x p`.'], ['¿Qué hace `recompile`?', 'Repite el último comando de compilación.']],
    tags: ['desarrollo', 'proyecto', 'comandos']
  },
  {
    id: 'm09-l02', module: 'm09', n: 2, title: 'Eglot y LSP', min: 9, level: 'Avanzado',
    obj: 'Conectar un language server y usar sus capacidades sin configurar un IDE entero.',
    frase: '`M-x eglot` conecta un LSP al buffer; el resto son comandos estándar de Emacs: `M-.`, `M-,`, `xref`, `flymake`.',
    idea: [
      'Eglot viene incluido desde Emacs 29. `M-x eglot` detecta el lenguaje y busca un servidor (rust-analyzer, pyright, clangd, gopls, etc.). Si no lo encuentra, `eglot-server-programs` define el comando. Al conectar, tenés definiciones (`M-.`), referencias (`M-x xref-find-references`), renombre (`M-x eglot-rename`), acciones de código (`M-x eglot-code-actions`) y formato (`M-x eglot-format`).',
      'Eglot usa `flymake` para diagnósticos y `eldoc` para documentación en la echo area. No intenta ser un IDE completo: integra el LSP con las APIs nativas de Emacs. Si preferís otra UI, `lsp-mode` es la alternativa, pero Eglot es más pequeño y ya viene incluido.'
    ],
    code: { lang: 'elisp', src: '(add-to-list \'eglot-server-programs\n             \'(rust-mode . ("rust-analyzer")))\n(add-hook \'rust-mode-hook #\'eglot-ensure)\n;; M-x eglot\n;; M-. / M-x eglot-rename / M-x eglot-code-actions', cap: 'Configuración mínima de Eglot.' },
    under: [
      'Eglot implementa el protocolo LSP sobre JSON-RPC y traduce capacidades a comandos de Emacs. `eglot-ensure` conecta automáticamente al abrir un buffer; `eglot-shutdown` desconecta. Los diagnósticos van a `flymake`; la documentación a `eldoc`; las definiciones a `xref`.',
      'El servidor debe estar instalado y en el PATH. Si Eglot no lo encuentra, `M-x eglot` muestra el error y `eglot-server-programs` permite indicar la ruta. Para proyectos grandes, el indexing puede tardar; `eglot-events-buffer` muestra el tráfico y los errores.'
    ],
    trap: { bad: 'Instalar Eglot y esperar que funcione sin el language server instalado.', good: 'Verificar que `rust-analyzer`, `pyright`, `clangd` o el servidor correspondiente esté en el PATH.', why: 'Eglot es el cliente; el servidor hace el análisis y debe existir.' },
    q: [
      { q: '¿Qué hace `M-x eglot`?', o: ['Conecta un language server', 'Compila', 'Hace commit', 'Abre Dired'], a: 0, why: 'Eglot inicia/conecta el servidor LSP para el buffer.' },
      { q: '¿Qué provee los diagnósticos en Eglot?', o: ['Flymake', 'Magit', 'Org', 'Corfu'], a: 0, why: 'Eglot integra los diagnósticos con Flymake.' }
    ],
    ex: { p: 'Abrí un archivo Rust, ejecutá `M-x eglot`, probá `M-.` y renombrá un símbolo con `M-x eglot-rename`.', start: ';; M-x eglot\n;; M-. sobre una función\n;; M-x eglot-rename', sol: ';; M-x eglot\n;; M-. sobre una función\n;; M-, para volver\n;; M-x eglot-rename', hint: 'Si falla, revisá `*EGLOT ...*` y que `rust-analyzer` esté en el PATH.' },
    cards: [['¿Qué cliente LSP viene incluido en Emacs 29+?', 'Eglot.'], ['¿Qué comando renombra un símbolo?', '`M-x eglot-rename`.']],
    tags: ['desarrollo', 'eglot', 'lsp', 'experto']
  },
  {
    id: 'm09-l03', module: 'm09', n: 3, title: 'Treesit, flymake y eldoc', min: 9, level: 'Avanzado',
    obj: 'Usar el parser incremental de treesit y las herramientas nativas de diagnóstico y documentación.',
    frase: 'Treesit da sintaxis real basada en árbol; Flymake marca errores; Eldoc muestra la firma sin abrir documentación.',
    idea: [
      '`treesit` usa gramáticas de Tree-sitter para parsear el buffer en un árbol concreto. Los modos con soporte (`rust-ts-mode`, `python-ts-mode`, `js-ts-mode`) obtienen indentación, resaltado y navegación más precisos. `M-x treesit-install-language-grammar` instala una gramática; `treesit-auto` elige automáticamente el modo -ts.',
      '`flymake-mode` ejecuta backends de diagnóstico y subraya errores/warnings; `M-x flymake-show-buffer-diagnostics` lista los del buffer; `M-x flymake-show-project-diagnostics` los del proyecto. `eldoc-mode` muestra la firma y el docstring del símbolo bajo el cursor en la echo area.'
    ],
    code: { lang: 'elisp', src: '(setq treesit-language-source-alist\n      \'((rust "https://github.com/tree-sitter/tree-sitter-rust")))\n;; M-x treesit-install-language-grammar RET rust RET\n(add-hook \'prog-mode-hook #\'flymake-mode)\n(add-hook \'prog-mode-hook #\'eldoc-mode)', cap: 'Treesit, Flymake y Eldoc.' },
    under: [
      'Treesit mantiene un árbol incremental: al editar, solo reparsea la parte afectada. Los modos -ts usan `treesit-font-lock-level` y consultas (`treesit-query-capture`) para resaltado e indentación. No todos los lenguajes tienen gramática instalada.',
      'Flymake es el marco; los backends son funciones que devuelven diagnósticos. Eglot registra un backend LSP; `flymake-grep` o `flymake-proc` pueden usar un linter externo. Eldoc consulta `eldoc-documentation-functions`; Eglot y los modos agregan fuentes.'
    ],
    trap: { bad: 'Instalar un modo -ts sin la gramática y obtener un modo fundamental silencioso.', good: 'Instalar la gramática con `treesit-install-language-grammar` y verificar `treesit-available-p`.', why: 'El modo necesita la gramática compilada; sin ella, no puede parsear.' },
    q: [
      { q: '¿Qué hace treesit?', o: ['Parseo incremental con Tree-sitter', 'Compila', 'Hace commit', 'Abre shell'], a: 0, why: 'Treesit construye un árbol sintáctico incremental para el buffer.' },
      { q: '¿Qué muestra Eldoc?', o: ['La firma/documentación del símbolo bajo el cursor', 'El log de Git', 'Los buffers', 'El clima'], a: 0, why: 'Eldoc muestra información de la API en la echo area.' }
    ],
    ex: { p: 'Instalá la gramática de Rust, abrí un archivo `.rs` en `rust-ts-mode` y activá flymake y eldoc.', start: 'M-x treesit-install-language-grammar RET rust RET\n;; abre un .rs\nM-x rust-ts-mode\nM-x flymake-mode\nM-x eldoc-mode', sol: 'M-x treesit-install-language-grammar RET rust RET\n;; abre un .rs\nM-x rust-ts-mode\nM-x flymake-mode\nM-x eldoc-mode', hint: 'Si `rust-ts-mode` no existe, tu Emacs es viejo o no tiene treesit compilado.' },
    cards: [['¿Qué comando instala una gramática?', '`M-x treesit-install-language-grammar`.'], ['¿Qué lista los diagnósticos del buffer?', '`M-x flymake-show-buffer-diagnostics`.']],
    tags: ['desarrollo', 'treesit', 'flymake', 'experto']
  },
  {
    id: 'm09-l04', module: 'm09', n: 4, title: 'Formateo, linting y tests', min: 8, level: 'Intermedio',
    obj: 'Formatear al guardar, ejecutar linters y correr tests desde Emacs.',
    frase: 'Formatear al guardar elimina discusiones; el linter marca lo que el compilador no ve; los tests se corren sin salir.',
    idea: [
      '`apheleia-mode` formatea el buffer al guardar usando la herramienta del lenguaje (rustfmt, black, prettier, gofmt). `format-all` es una alternativa. `flycheck` es un marco de linting con muchos backends; `flymake` es el nativo. Elegí uno y no los actives juntos.',
      '`M-x compile` con `cargo test`, `pytest`, `npm test` o el comando del proyecto; `M-g M-n` navega fallos. `project-compile` usa la raíz. Para tests unitarios en Elisp, `ERT`; para proyectos, el runner del lenguaje. `M-x test` no existe: usá `compile` o un paquete específico.'
    ],
    code: { lang: 'elisp', src: '(use-package apheleia\n  :init (apheleia-global-mode 1))\n(add-hook \'prog-mode-hook #\'flymake-mode)\n;; M-x compile RET cargo test RET\n;; M-g M-n', cap: 'Formateo y tests.' },
    under: [
      'Apheleia ejecuta el formateador en un proceso y reemplaza el buffer solo si el resultado es válido; si el formateador falla, no toca el archivo. `apheleia-formatters` define comandos por modo. Es más seguro que un formateador que escribe in place.',
      'Flymake/Flycheck son marcos: los backends producen diagnósticos. Flycheck tiene más backends históricos; Flymake es nativo y se integra con Eglot. Para tests, `compilation-mode` ya entiende el formato de la mayoría de runners; si no, ajustá `compilation-error-regexp-alist`.'
    ],
    trap: { bad: 'Activar Flymake y Flycheck a la vez y ver dos subrayados distintos.', good: 'Elegir uno: Flymake si usás Eglot, Flycheck si necesitás un backend específico.', why: 'Ambos marcan el buffer y compiten por los mismos overlays.' },
    q: [
      { q: '¿Qué hace Apheleia?', o: ['Formatea al guardar', 'Hace commit', 'Abre shell', 'Gestiona buffers'], a: 0, why: 'Ejecuta el formateador del lenguaje al guardar.' },
      { q: '¿Qué comando corre los tests de un proyecto?', o: ['`M-x compile` con el comando de test', '`C-x C-c`', '`M-x dired`', '`C-x b`'], a: 0, why: '`compile` ejecuta el runner y permite navegar fallos.' }
    ],
    ex: { p: 'Configurá Apheleia para Rust, corré `cargo test` con `compile` y navegá un fallo con `M-g M-n`.', start: '(apheleia-global-mode 1)\n;; M-x compile RET cargo test RET\n;; M-g M-n', sol: '(apheleia-global-mode 1)\n;; M-x compile RET cargo test RET\n;; M-g M-n', hint: 'Si el formato no se aplica, revisá `apheleia-formatters` para tu modo.' },
    cards: [['¿Qué hace `apheleia-global-mode`?', 'Formatea buffers al guardar usando el formateador adecuado.'], ['¿Qué comando navega errores?', '`M-g M-n` (`next-error`).']],
    tags: ['desarrollo', 'formato', 'testing']
  },
  {
    id: 'm09-l05', module: 'm09', n: 5, title: 'DAP, REPLs y comint', min: 9, level: 'Avanzado',
    obj: 'Depurar con DAP y usar REPLs integrados sin salir de Emacs.',
    frase: '`dap-mode` habla el Debug Adapter Protocol; `comint` es la base de los REPLs y shells interactivos.',
    idea: [
      '`dap-mode` conecta debug adapters (lldb, debugpy, delve, etc.) y ofrece breakpoints, stack frames, variables y evaluación. `M-x dap-debug` inicia una sesión; `dap-hydra` muestra las acciones; `dap-breakpoint-toggle` marca una línea; `dap-continue`, `dap-next`, `dap-step-in`. Requiere el adapter instalado y una plantilla de configuración.',
      '`comint-mode` es la infraestructura de procesos interactivos: prompt, historial, input y output. `run-python`, `run-js`, los REPLs que proveen los modos, `M-x shell` y `M-x eshell` se apoyan en comint o en variantes. Podés enviar la región al REPL con `C-c C-r` o `C-c C-c` según el modo.'
    ],
    code: { lang: 'elisp', src: '(require \'dap-mode)\n(dap-mode 1)\n(dap-auto-configure-mode 1)\n;; M-x dap-debug\n;; M-x dap-breakpoint-toggle\n;; M-x dap-hydra', cap: 'DAP mínimo.' },
    under: [
      'DAP es un protocolo JSON sobre stdin/stdout, igual que LSP. `dap-mode` lanza el adapter, abre una sesión y traduce eventos a buffers de Emacs. Las plantillas (`dap-register-debug-template`) definen programa, args, cwd y adapter.',
      'Comint mantiene un buffer de proceso, un historial y un mark de input. `comint-send-input` envía la línea; `comint-interrupt-subjob` (`C-c C-c`) interrumpe; `comint-next-input` (`M-n`) recorre el historial. Muchos modos derivan de comint y agregan atajos.'
    ],
    trap: { bad: 'Intentar depurar con `dap-mode` sin instalar el adapter o sin una plantilla válida.', good: 'Instalar el adapter (`debugpy`, `lldb-vscode`, `dlv`) y registrar una plantilla con el programa y el cwd.', why: 'DAP es solo el protocolo; el adapter hace el trabajo de depuración.' },
    q: [
      { q: '¿Qué protocolo usa dap-mode?', o: ['DAP', 'LSP', 'HTTP', 'SMTP'], a: 0, why: 'Debug Adapter Protocol.' },
      { q: '¿Qué base usan muchos REPLs en Emacs?', o: ['comint', 'Magit', 'Org', 'Dired'], a: 0, why: 'Comint implementa procesos interactivos con prompt e historial.' }
    ],
    ex: { p: 'Iniciá un REPL de Python con `run-python`, enviá una expresión y probá `M-n` para el historial.', start: 'M-x run-python\nprint(2 + 2)\nM-n', sol: 'M-x run-python\nprint(2 + 2)\nM-n', hint: 'En el REPL, `C-c C-c` interrumpe; `C-c C-z` vuelve al buffer anterior en algunos modos.' },
    cards: [['¿Qué hace `dap-breakpoint-toggle`?', 'Pone/quita un breakpoint en la línea actual.'], ['¿Qué es comint?', 'La infraestructura de procesos interactivos con prompt e historial.']],
    tags: ['desarrollo', 'dap', 'repl', 'experto']
  },
  {
    id: 'm10-l01', module: 'm10', n: 1, title: 'Formas, eval y quote', min: 8, level: 'Intermedio',
    obj: 'Leer y evaluar formas Lisp, entender quote y la diferencia entre código y datos.',
    frase: 'En Lisp todo es una forma: `(+ 1 2)` se evalúa; `(+ 1 2)` citado es una lista de tres elementos.',
    idea: [
      'Una **forma** es una expresión que Emacs Lisp puede evaluar. `(+ 1 2)` es una lista cuya cabeza es la función `+`; se evalúa a `3`. `(quote (+ 1 2))` o `\'(+ 1 2)` devuelve la lista sin evaluar. `eval` evalúa una forma; `read` convierte texto en una forma.',
      'La sintaxis mínima: números, strings, símbolos, listas y vectores. `nil` es la lista vacía y el valor falso; `t` es verdadero. Los símbolos pueden tener propiedades y funciones. Entender quote es la clave para escribir macros y para no confundir datos con código.'
    ],
    code: { lang: 'elisp', src: '(+ 1 2)          ; => 3\n\'(+ 1 2)         ; => (+ 1 2)\n(quote (+ 1 2))  ; => (+ 1 2)\n(eval \'(+ 1 2))  ; => 3\n(read "(+ 1 2)")  ; => (+ 1 2)', cap: 'Evaluar, citar y leer.' },
    under: [
      'El evaluador trata una lista así: evalúa la cabeza (debe ser una función o macro), evalúa los argumentos y aplica. Si la cabeza es un símbolo con `symbol-function`, llama a esa función. Las **formas especiales** (`quote`, `if`, `let`, `defun`) no evalúan todos sus argumentos.',
      '`read` usa el reader de Lisp y devuelve estructuras; no evalúa. `eval` evalúa en el entorno actual, con `lexical-binding` si está activo. `prin1-to-string` convierte una forma en texto legible; `format` interpola.'
    ],
    trap: { bad: 'Escribir `(+ 1 2)` dentro de una lista de datos y olvidar el quote; Emacs intenta evaluarlo.', good: 'Usar `\'(...)` para datos y `(list ...)` para construir listas evaluadas.', why: 'Sin quote, una lista se interpreta como llamada a función.' },
    q: [
      { q: '¿Qué devuelve `\'(+ 1 2)`?', o: ['3', 'La lista `(+ 1 2)`', 'Un error', '`nil`'], a: 1, why: 'Quote evita la evaluación y devuelve la lista.' },
      { q: '¿Qué hace `eval`?', o: ['Lee texto', 'Evalúa una forma', 'Imprime', 'Cierra'], a: 1, why: '`eval` evalúa una expresión Lisp en el entorno actual.' }
    ],
    ex: { p: 'Evaluá `(+ 1 2)`, `\'(+ 1 2)` y `(eval \'(+ 1 2))`. Explicá la diferencia.', start: '(+ 1 2)\n\'(+ 1 2)\n(eval \'(+ 1 2))', sol: '(+ 1 2)          ; 3\n\'(+ 1 2)         ; (+ 1 2)\n(eval \'(+ 1 2))  ; 3', hint: '`C-x C-e` evalúa la forma antes del punto; `C-j` evalúa e inserta el resultado.' },
    cards: [['¿Qué hace quote?', 'Devuelve la forma sin evaluarla.'], ['¿Qué devuelve `(read "(+ 1 2)")`?', 'La lista `(+ 1 2)`, no el número 3.']],
    tags: ['elisp', 'formas', 'comandos']
  },
  {
    id: 'm10-l02', module: 'm10', n: 2, title: 'Listas, cons y símbolos', min: 9, level: 'Intermedio',
    obj: 'Construir y recorrer listas, y entender los símbolos como objetos con nombre.',
    frase: 'Una lista Lisp es una cadena de cons cells; `car` da el primero, `cdr` el resto, `cons` agrega al frente.',
    idea: [
      '`(cons 1 2)` crea un par; `(list 1 2 3)` crea una lista propia; `\'(1 2 3)` es una lista literal. `car` devuelve el primer elemento; `cdr` el resto; `cadr` el segundo; `nth` el n-ésimo. `nil` es la lista vacía y el final de una lista propia.',
      'Un **símbolo** es un objeto con nombre (`foo`, `+`, `:clave`). Puede tener una función (`symbol-function`), un valor (`symbol-value`), una propiedad (`symbol-plist`) y una posición. Los keywords (`:foo`) se evalúan a sí mismos. `intern` crea o encuentra un símbolo; `symbol-name` lo devuelve como string.'
    ],
    code: { lang: 'elisp', src: '(cons 1 \'(2 3))   ; => (1 2 3)\n(car \'(1 2 3))    ; => 1\n(cdr \'(1 2 3))    ; => (2 3)\n(nth 1 \'(a b c))   ; => b\n(symbol-name \'foo) ; => "foo"\n(keywordp :hola)   ; => t', cap: 'Listas y símbolos.' },
    under: [
      'Una lista propia termina en `nil`; una lista impropia termina en otro valor (`(cons 1 2)`). `length` solo funciona con listas propias. `mapcar` aplica una función a cada elemento y devuelve una lista nueva; `dolist` itera por efecto; `cl-loop` es más expresivo.',
      'Los símbolos se almacenan en el **obarray**. `intern` los registra; `make-symbol` crea uno no internado (útil para macros). Las propiedades (`put`, `get`) permiten metadata sin contaminar el valor de la variable.'
    ],
    trap: { bad: 'Usar `nth` en una lista larga dentro de un bucle y obtener O(n²).', good: 'Recorrer con `dolist`/`cl-loop` o usar un vector si necesitás acceso por índice.', why: 'Las listas son enlazadas; `nth` recorre desde el inicio.' },
    q: [
      { q: '¿Qué devuelve `(car \'(1 2 3))`?', o: ['1', '(2 3)', '3', 'nil'], a: 0, why: '`car` devuelve el primer elemento.' },
      { q: '¿Qué es un keyword?', o: ['Un símbolo que se evalúa a sí mismo', 'Una función', 'Una lista', 'Un buffer'], a: 0, why: 'Los keywords (`:foo`) son símbolos autoevaluados.' }
    ],
    ex: { p: 'Construí `\'(a b c)`, obtené el segundo con `nth`, y mapeá `upcase` sobre una lista de strings.', start: '(nth 1 \'(a b c))\n(mapcar #\'upcase \'("a" "b"))', sol: '(nth 1 \'(a b c))          ; b\n(mapcar #\'upcase \'("a" "b")) ; ("A" "B")', hint: '`mapcar` devuelve una lista nueva; `mapc` solo por efecto.' },
    cards: [['¿Qué hace `cons`?', 'Crea un par con dos valores.'], ['¿Qué hace `mapcar`?', 'Aplica una función a cada elemento y devuelve una lista.']],
    tags: ['elisp', 'listas', 'comandos']
  },
  {
    id: 'm10-l03', module: 'm10', n: 3, title: 'Funciones, argumentos e interactive', min: 9, level: 'Intermedio',
    obj: 'Definir funciones, aceptar argumentos opcionales y convertirlas en comandos.',
    frase: '`defun` define una función; `(interactive)` la convierte en comando; los argumentos pueden ser opcionales o variádicos.',
    idea: [
      '`(defun nombre (args...) "docstring" body...)` define una función. `&optional` marca argumentos opcionales; `&rest` recoge el resto en una lista. `(interactive)` sin argumentos permite invocarla con `M-x`; con un string, pide entrada (`"s"` string, `"n"` número, `"f"` función, `"b"` buffer).',
      '`funcall` llama a una función con argumentos; `apply` recibe una lista de argumentos. `call-interactively` ejecuta un comando como si lo hubieras invocado, incluyendo sus argumentos interactivos. Una función sin `(interactive)` no aparece en `M-x`.'
    ],
    code: { lang: 'elisp', src: '(defun saludar (nombre &optional saludo)\n  "Saluda a NOMBRE."\n  (interactive "sNombre: ")\n  (message "%s, %s" (or saludo "Hola") nombre))\n\n(funcall #\'saludar "Ada")\n(apply #\'+ \'(1 2 3))', cap: 'Función, argumento opcional y comando.' },
    under: [
      'El string de `interactive` es una mini-especificación: cada carácter pide un tipo de dato y el resto es el prompt. `"s"` pide string, `"n"` número, `"f"` función, `"b"` buffer, `"r"` región (punto y mark). `C-h f` muestra la firma y el docstring.',
      'Las funciones son objetos; `#\'foo` es un function quote. `funcall` es la forma explícita de llamar; `apply` es útil cuando los argumentos ya están en una lista. `cl-defun` permite `&key` y `&optional` con defaults más ricos.'
    ],
    trap: { bad: 'Definir una función sin `(interactive)` y preguntarte por qué no aparece en `M-x`.', good: 'Agregar `(interactive)` y, si necesita argumentos, el string de especificación.', why: '`M-x` solo lista comandos, y un comando es una función interactiva.' },
    q: [
      { q: '¿Qué hace `(interactive)`?', o: ['Convierte la función en comando', 'La compila', 'La borra', 'La documenta'], a: 0, why: 'Marca la función como invocable desde `M-x` o una tecla.' },
      { q: '¿Qué recoge `&rest`?', o: ['Los argumentos restantes en una lista', 'El primer argumento', 'El buffer', 'El punto'], a: 0, why: '`&rest` empaqueta los argumentos que sobran en una lista.' }
    ],
    ex: { p: 'Definí `sumar-todos` con `&rest` que sume todos sus argumentos y probala con `(sumar-todos 1 2 3)`.', start: '(defun sumar-todos (&rest nums)\n  (apply #\'+ nums))', sol: '(defun sumar-todos (&rest nums)\n  "Suma todos los números."\n  (apply #\'+ nums))\n(sumar-todos 1 2 3) ; => 6', hint: '`apply` recibe la función y una lista de argumentos.' },
    cards: [['¿Qué hace `funcall`?', 'Llama a una función con argumentos evaluados.'], ['¿Qué pide `"s"` en un interactive?', 'Un string, con el prompt indicado.']],
    tags: ['elisp', 'funciones', 'comandos']
  },
  {
    id: 'm10-l04', module: 'm10', n: 4, title: 'Variables, let y scope', min: 9, level: 'Intermedio',
    obj: 'Entender variables globales, locales, dynamic vs lexical scope y defcustom.',
    frase: '`let` crea un ámbito local; `setq` asigna; `defvar` declara global; `lexical-binding` cambia las reglas del juego.',
    idea: [
      '`defvar` declara una variable global y opcionalmente su valor y docstring. `defcustom` la declara configurable, con tipo y grupo. `setq` asigna; `let` crea un binding dinámico (por defecto) o léxico (si `lexical-binding` es `t`). `let*` permite que cada binding vea los anteriores.',
      'Con **dynamic scope**, una función llamada dentro de un `let` ve la variable aunque no esté declarada localmente. Con **lexical scope**, solo ve las variables de su ámbito léxico. Los archivos modernos usan `;; -*- lexical-binding: t; -*-` en la primera línea.'
    ],
    code: { lang: 'elisp', src: '(defvar mi-contador 0 "Contador global.")\n(defcustom mi-color "azul" "Color favorito."\n  :type \'string :group \'my-group)\n\n(let ((x 1) (y 2)) (+ x y))\n(let* ((x 1) (y (+ x 1))) y) ; => 2\n(setq mi-contador (1+ mi-contador))', cap: 'Variables y ámbitos.' },
    under: [
      'El **obarray** guarda símbolos; cada símbolo tiene una celda de valor. `let` guarda el valor anterior, instala el nuevo y lo restaura al salir (`unwind-protect` garantiza la restauración). `setq` modifica el binding actual.',
      '`lexical-binding: t` hace que las closures capturen variables por referencia léxica. Sin él, las closures capturan dinámicamente y se rompen al salir del `let`. Es la razón por la que casi todo el Elisp moderno lo activa.'
    ],
    trap: { bad: 'Usar `setq` para variables locales y contaminar el ámbito global.', good: 'Usar `let` para locales y `defvar`/`defcustom` para globales; activar `lexical-binding`.', why: '`setq` asigna al binding más cercano o crea uno global; `let` delimita el ámbito.' },
    q: [
      { q: '¿Qué diferencia hay entre `let` y `let*`?', o: ['`let*` permite que cada binding vea los anteriores', 'Son iguales', '`let` es global', '`let*` borra'], a: 0, why: '`let*` evalúa los bindings en secuencia y permite referencias previas.' },
      { q: '¿Qué activa `lexical-binding: t`?', o: ['Scope léxico', 'Compilación', 'Modo modal', 'Red'], a: 0, why: 'Hace que las closures capturen variables por ámbito léxico.' }
    ],
    ex: { p: 'Definí una variable global con `defvar`, una local con `let` y comprobá que la global no cambia.', start: '(defvar x 10)\n(let ((x 20)) x)\nx', sol: '(defvar x 10)\n(let ((x 20)) x) ; 20\nx                ; 10', hint: '`let` restaura el valor anterior al salir del cuerpo.' },
    cards: [['¿Qué declara `defcustom`?', 'Una variable configurable con tipo, grupo y docstring.'], ['¿Qué hace `let*`?', 'Bindings secuenciales que pueden referirse a los anteriores.']],
    tags: ['elisp', 'variables', 'scope']
  },
  {
    id: 'm10-l05', module: 'm10', n: 5, title: 'Control de flujo, cl-lib y errores', min: 9, level: 'Avanzado',
    obj: 'Usar condicionales, bucles de cl-lib y manejo de errores con condition-case.',
    frase: '`if` y `cond` para decidir; `cl-loop` para recorrer; `condition-case` para no explotar cuando algo falla.',
    idea: [
      '`if` evalúa una condición y dos ramas; `when`/`unless` son azúcar para una sola rama; `cond` prueba pares condición/cuerpo y ejecuta el primero verdadero; `pcase` hace pattern matching. `and`/`or` cortocircuitan; `not` niega.',
      '`cl-lib` trae `cl-loop`, `cl-incf`, `cl-pushnew`, `cl-remove-if`, etc. `cl-loop for x in list sum x` es muy expresivo. Para errores, `condition-case` captura un símbolo de error y ejecuta un handler; `ignore-errors` devuelve `nil`; `unwind-protect` ejecuta limpieza pase lo que pase.'
    ],
    code: { lang: 'elisp', src: '(cond ((> x 0) "positivo")\n      ((< x 0) "negativo")\n      (t "cero"))\n\n(cl-loop for x in \'(1 2 3) sum x) ; => 6\n\n(condition-case err\n    (car 42)\n  (error (message "falló: %s" err)))\n\n(unwind-protect (foo) (bar))', cap: 'Condicionales, cl-loop y errores.' },
    under: [
      'Los errores en Elisp son símbolos (`error`, `wrong-type-argument`, `file-error`) con datos. `condition-case` captura por símbolo y recibe `(err ...)`. `signal` lanza un error; `error` es la forma común. `debug-on-error` permite entrar al depurador.',
      '`cl-loop` es un mini-lenguaje: `for`, `in`, `on`, `from`, `to`, `collect`, `sum`, `count`, `maximize`, `while`, `until`. Se expande a código eficiente. `pcase` es el pattern matching nativo y se usa en muchos paquetes modernos.'
    ],
    trap: { bad: 'Capturar `(error ...)` alrededor de todo el programa y ocultar errores reales.', good: 'Capturar solo el error esperado y volver a lanzar (`signal`) lo inesperado.', why: 'Un handler demasiado amplio convierte bugs en silencios difíciles de depurar.' },
    q: [
      { q: '¿Qué hace `condition-case`?', o: ['Captura errores por símbolo', 'Define funciones', 'Crea buffers', 'Hace commit'], a: 0, why: 'Ejecuta un handler si el cuerpo lanza el error indicado.' },
      { q: '¿Qué hace `cl-loop`?', o: ['Construye bucles expresivos', 'Cierra Emacs', 'Compila', 'Abre shell'], a: 0, why: 'Es una macro de `cl-lib` para iterar, sumar, collectar, etc.' }
    ],
    ex: { p: 'Sumá los pares de una lista con `cl-loop` y capturá un error de tipo con `condition-case`.', start: '(cl-loop for x in \'(1 2 3 4) when (cl-evenp x) sum x)\n(condition-case err (car 42) (error (message "%s" err)))', sol: '(cl-loop for x in \'(1 2 3 4) when (cl-evenp x) sum x) ; 6\n(condition-case err (car 42) (error (message "%s" err)))', hint: '`cl-evenp` está en `cl-lib`; `condition-case` recibe el símbolo `error`.' },
    cards: [['¿Qué hace `unwind-protect`?', 'Ejecuta la limpieza siempre, haya error o no.'], ['¿Qué macro hace pattern matching nativo?', '`pcase`.']],
    tags: ['elisp', 'control', 'errores', 'experto']
  },
  {
    id: 'm11-l01', module: 'm11', n: 1, title: 'Buffers, point, mark y narrowing', min: 9, level: 'Avanzado',
    obj: 'Manipular buffers y posiciones con las macros de contexto de Elisp.',
    frase: '`with-current-buffer` cambia de buffer temporalmente; `save-excursion` guarda punto y mark; `save-restriction` guarda el narrowing.',
    idea: [
      '`with-current-buffer` ejecuta un cuerpo en otro buffer sin cambiar el buffer visible. `save-excursion` guarda punto, mark y buffer actual, y los restaura al salir. `save-restriction` guarda el narrowing (la parte visible del buffer) y `widen` lo elimina. `save-excursion` + `save-restriction` es el combo para editar sin molestar al usuario.',
      '`point-min`/`point-max` son los límites del buffer accesible (respetan narrowing). `goto-char` mueve el punto; `(point)` lo lee. `insert` inserta texto en el punto; `delete-region` borra; `buffer-substring` extrae. `erase-buffer` borra todo (con cuidado).'
    ],
    code: { lang: 'elisp', src: '(with-current-buffer "*scratch*"\n  (save-excursion\n    (goto-char (point-max))\n    (insert "\\n;; fin")))\n\n(save-restriction\n  (narrow-to-region (point-min) 100)\n  (buffer-string))', cap: 'Editar otro buffer sin mover el punto visible.' },
    under: [
      'El **narrowing** limita el buffer a una región; muchas funciones respetan `point-min`/`point-max`. `save-restriction` guarda el estado y lo restaura. `widen` vuelve a ver todo. Es útil para operar en una sección sin afectar el resto.',
      'Los **markers** se ajustan al insertar/borrar. `copy-marker` crea uno; `set-marker` lo mueve. El punto y el mark son markers. `save-excursion` guarda sus posiciones y las restaura; si el texto cambió, el marker se ajusta.'
    ],
    trap: { bad: 'Modificar otro buffer sin `save-excursion` y dejar el punto del usuario en otro sitio.', good: 'Envolver la edición en `with-current-buffer` + `save-excursion` (y `save-restriction` si hay narrowing).', why: 'Las operaciones mueven punto y mark; sin guardar/restaurar, el usuario pierde su posición.' },
    q: [
      { q: '¿Qué hace `with-current-buffer`?', o: ['Ejecuta código en otro buffer', 'Cierra el buffer', 'Guarda', 'Compila'], a: 0, why: 'Cambia temporalmente el buffer actual para el cuerpo.' },
      { q: '¿Qué guarda `save-restriction`?', o: ['El narrowing', 'El kill ring', 'Las teclas', 'El tema'], a: 0, why: 'Guarda y restaura el estado de restricción del buffer.' }
    ],
    ex: { p: 'Insertá una línea al final de `*scratch*` sin cambiar el punto visible del buffer actual.', start: '(with-current-buffer "*scratch*"\n  (save-excursion\n    (goto-char (point-max))\n    (insert "\\n;; nota")))', sol: '(with-current-buffer "*scratch*"\n  (save-excursion\n    (goto-char (point-max))\n    (insert "\\n;; nota")))', hint: '`save-excursion` restaura punto y mark; `with-current-buffer` restaura el buffer.' },
    cards: [['¿Qué hace `save-excursion`?', 'Guarda punto, mark y buffer, y los restaura al salir.'], ['¿Qué hace `narrow-to-region`?', 'Limita el buffer a una región.']],
    tags: ['elisp', 'buffers', 'experto']
  },
  {
    id: 'm11-l02', module: 'm11', n: 2, title: 'Text properties, overlays y faces', min: 10, level: 'Avanzado',
    obj: 'Anotar texto con propiedades, superponer overlays y definir caras.',
    frase: 'Las text properties viajan con el texto; los overlays viven en el buffer y se reajustan; las faces definen cómo se ve.',
    idea: [
      '`put-text-property` añade una propiedad a un rango de texto; `get-text-property` la lee. Las propiedades se mueven con el texto al copiar/pegar. `add-face-text-property` aplica una cara. Son la base del resaltado de sintaxis y de los enlaces.',
      'Los **overlays** son objetos que cubren un rango y tienen propiedades (face, display, help-echo, priority). `make-overlay`, `overlay-put`, `delete-overlay`. A diferencia de las text properties, no se copian con el texto y pueden solaparse. `defface` define una cara con especificaciones para distintos temas.'
    ],
    code: { lang: 'elisp', src: '(with-temp-buffer\n  (insert "hola mundo")\n  (put-text-property 1 5 \'face \'bold)\n  (let ((ov (make-overlay 6 11)))\n    (overlay-put ov \'face \'italic)\n    (overlay-put ov \'help-echo "mundo")))\n\n(defface my-face\n  \'((t :foreground "gold"))\n  "Mi cara.")', cap: 'Text properties, overlay y face.' },
    under: [
      'Las text properties se guardan en el texto y se copian con él; los overlays se guardan en una lista del buffer y se ajustan cuando el texto cambia. Los overlays tienen `priority` para decidir cuál gana. `font-lock` usa ambas cosas para resaltar.',
      'Una **face** es un conjunto de atributos (color, peso, slant, subrayado). `defface` define especificaciones por tipo de terminal (`t`, `((class color) ...)`). `custom-set-faces` las aplica. Las caras se combinan; la prioridad y el orden importan.'
    ],
    trap: { bad: 'Usar text properties para algo que debe desaparecer al copiar el texto, o overlays para algo que debe viajar con el texto.', good: 'Text properties para enlaces/resaltado que pertenecen al texto; overlays para decoraciones temporales o interactivas.', why: 'Tienen semántica de copia y ajuste distinta.' },
    q: [
      { q: '¿Qué se copia con el texto?', o: ['Las text properties', 'Los overlays', 'Las faces', 'Los buffers'], a: 0, why: 'Las text properties forman parte del texto; los overlays no.' },
      { q: '¿Qué define una cara?', o: ['Atributos visuales', 'Una función', 'Un buffer', 'Una tecla'], a: 0, why: 'Una face agrupa color, peso, slant y otros atributos.' }
    ],
    ex: { p: 'En un buffer temporal, poné en negrita “hola” con una text property y “mundo” con un overlay en cursiva.', start: '(with-temp-buffer\n  (insert "hola mundo")\n  (put-text-property 1 5 \'face \'bold)\n  (let ((ov (make-overlay 6 11)))\n    (overlay-put ov \'face \'italic)))', sol: '(with-temp-buffer\n  (insert "hola mundo")\n  (put-text-property 1 5 \'face \'bold)\n  (let ((ov (make-overlay 6 11)))\n    (overlay-put ov \'face \'italic)))', hint: '`with-temp-buffer` evita tocar buffers reales; el overlay se borra con el buffer.' },
    cards: [['Diferencia entre text property y overlay', 'La property viaja con el texto; el overlay se ajusta al rango y no se copia.'], ['¿Qué función define una cara?', '`defface`.']],
    tags: ['elisp', 'text-properties', 'overlays', 'experto']
  },
  {
    id: 'm11-l03', module: 'm11', n: 3, title: 'Keymaps, minor modes y major modes', min: 10, level: 'Experto',
    obj: 'Crear atajos, modos menores y modos mayores derivados.',
    frase: 'Un keymap es una tabla de teclas; un modo menor añade comportamiento; un modo mayor define el mundo del buffer.',
    idea: [
      '`define-key` asocia una tecla a un comando en un keymap; `keymap-set` es la forma moderna; `kbd` convierte notación (`"C-c x"`) en eventos. `global-map` es el keymap global; cada modo mayor tiene su `foo-mode-map`; los modos menores tienen el suyo y se activan por buffer.',
      '`define-minor-mode` crea un modo ligero con variable, keymap y body. `define-derived-mode` crea un modo mayor a partir de otro (por ejemplo, `prog-mode`) y hereda keymap y hooks. `run-mode-hooks` ejecuta los hooks del modo y del padre.'
    ],
    code: { lang: 'elisp', src: '(define-key global-map (kbd "C-c e") #\'my-command)\n\n(define-minor-mode my-mode\n  "Mi modo menor."\n  :lighter " My"\n  :keymap (let ((m (make-sparse-keymap)))\n            (define-key m (kbd "C-c m") #\'my-command)\n            m))\n\n(define-derived-mode my-prog-mode prog-mode "MyProg"\n  "Modo de ejemplo."\n  (setq-local comment-start "// "))', cap: 'Keymap, minor mode y derived mode.' },
    under: [
      'Los keymaps se buscan en orden: modo menor, modo mayor, global. `emulation-mode-map-alists` permite capas. `keymap-set` reemplaza a `define-key` en código moderno; `keymap-set-after` controla prioridad. `where-is` y `C-h k` muestran la resolución final.',
      'Un modo mayor configura `font-lock`, `syntax-table`, `indent-line-function`, `imenu-generic-expression` y su keymap. `define-derived-mode` hereda todo y agrega lo propio. Los hooks se ejecutan al activar el modo; `after-change-major-mode-hook` corre después.'
    ],
    trap: { bad: 'Definir un modo mayor desde cero y olvidar `run-mode-hooks`, con lo que los hooks del usuario nunca corren.', good: 'Usar `define-derived-mode`, que ya ejecuta los hooks correctamente.', why: 'Los hooks son el punto de extensión; sin ejecutarlos, la configuración del usuario no se aplica.' },
    q: [
      { q: '¿Qué crea `define-minor-mode`?', o: ['Un modo menor', 'Un buffer', 'Un proceso', 'Una cara'], a: 0, why: 'Define una variable de modo, keymap y body para un modo ligero.' },
      { q: '¿Qué hace `define-derived-mode`?', o: ['Crea un modo mayor heredando de otro', 'Crea un buffer', 'Hace commit', 'Compila'], a: 0, why: 'Deriva un modo mayor de un padre y hereda keymap/hooks.' }
    ],
    ex: { p: 'Definí un modo menor `my-mode` con un atajo `C-c m` que muestre un mensaje.', start: '(define-minor-mode my-mode\n  "Mi modo."\n  :lighter " My"\n  :keymap (let ((m (make-sparse-keymap)))\n            (define-key m (kbd "C-c m")\n              (lambda () (interactive) (message "modo")))\n            m))', sol: '(define-minor-mode my-mode\n  "Mi modo."\n  :lighter " My"\n  :keymap (let ((m (make-sparse-keymap)))\n            (define-key m (kbd "C-c m")\n              (lambda () (interactive) (message "modo")))\n            m))\n(my-mode 1)', hint: 'El keymap del modo menor solo está activo cuando el modo está activo.' },
    cards: [['¿Qué orden de keymaps se consulta?', 'Modo menor, modo mayor, global.'], ['¿Qué función ejecuta los hooks de un modo?', '`run-mode-hooks`.']],
    tags: ['elisp', 'modos', 'keymaps', 'experto']
  },
  {
    id: 'm11-l04', module: 'm11', n: 4, title: 'Hooks, advice y carga diferida', min: 9, level: 'Experto',
    obj: 'Extender comportamiento sin modificar el código original, y cargar paquetes solo cuando hace falta.',
    frase: 'Un hook es una lista de funciones que se ejecutan en un evento; advice envuelve una función existente; `with-eval-after-load` espera a que el paquete cargue.',
    idea: [
      '`add-hook` agrega una función a un hook (`prog-mode-hook`, `after-save-hook`, `post-command-hook`). `remove-hook` la quita. Los hooks son la forma normal de extender Emacs sin parchear paquetes. `with-eval-after-load` ejecuta código cuando un feature se carga, sin obligarlo a cargarse ahora.',
      '`advice-add` envuelve una función: `:before`, `:after`, `:around`, `:override`. `:around` recibe la función original y decide si llamarla. Es potente y frágil: si la función cambia, tu advice puede romperse. Preferí hooks cuando existan; usá advice cuando no haya otra opción.'
    ],
    code: { lang: 'elisp', src: '(add-hook \'prog-mode-hook #\'display-line-numbers-mode)\n\n(with-eval-after-load \'org\n  (setq org-agenda-files \'("~/org/inbox.org")))\n\n(defun my-around (orig &rest args)\n  (message "antes")\n  (apply orig args))\n(advice-add \'save-buffer :around #\'my-around)', cap: 'Hook, carga diferida y advice.' },
    under: [
      'Los hooks son variables que contienen listas de funciones. `add-hook` agrega al final (o al principio con `append`); `run-hooks` las ejecuta. Los hooks de modo se ejecutan cada vez que se activa el modo; los de cambio, en cada evento.',
      '`advice-add` usa `advice--p` y una lista de consejos. `:around` recibe `orig` y los argumentos; `:before`/`:after` no pueden cambiar el valor de retorno. `advice-remove` deshace. `with-eval-after-load` se expande a `eval-after-load` y es la forma de configurar sin cargar.'
    ],
    trap: { bad: 'Usar `:override` para reemplazar una función y romper actualizaciones del paquete.', good: 'Usar `:around` o `:before`/`:after`, o un hook si existe.', why: '`:override` ignora la implementación original y pierde mejoras/bugfixes.' },
    q: [
      { q: '¿Qué hace `add-hook`?', o: ['Agrega una función a un hook', 'Crea un buffer', 'Compila', 'Hace commit'], a: 0, why: 'Añade la función a la lista que se ejecuta en el evento.' },
      { q: '¿Qué hace `with-eval-after-load`?', o: ['Ejecuta código cuando un feature carga', 'Cierra Emacs', 'Borra', 'Guarda'], a: 0, why: 'Difiere la configuración hasta que el paquete esté disponible.' }
    ],
    ex: { p: 'Agregá `display-line-numbers-mode` a `prog-mode-hook` y configurá Org solo cuando cargue.', start: '(add-hook \'prog-mode-hook #\'display-line-numbers-mode)\n(with-eval-after-load \'org\n  (setq org-agenda-files \'("~/org/inbox.org")))', sol: '(add-hook \'prog-mode-hook #\'display-line-numbers-mode)\n(with-eval-after-load \'org\n  (setq org-agenda-files \'("~/org/inbox.org")))', hint: '`with-eval-after-load` no carga el paquete; solo espera si se carga.' },
    cards: [['¿Qué hace `advice-add`?', 'Envuelve una función existente con código adicional.'], ['¿Qué advice puede cambiar el valor de retorno?', '`:around` (y `:override`).']],
    tags: ['elisp', 'hooks', 'advice', 'experto']
  },
  {
    id: 'm11-l05', module: 'm11', n: 5, title: 'Procesos, timers y async', min: 10, level: 'Experto',
    obj: 'Lanzar procesos, filtrar salida, usar sentinels y programar tareas.',
    frase: 'Un proceso tiene filtro y sentinel; un timer ejecuta una función después de un tiempo o cuando Emacs está idle.',
    idea: [
      '`start-process` lanza un proceso con nombre, buffer y programa. `make-process` permite `:command`, `:filter`, `:sentinel`, `:coding`. El **filtro** recibe la salida; el **sentinel** se llama cuando el proceso cambia de estado. `process-send-string` escribe al stdin; `delete-process` lo mata.',
      '`run-with-timer` ejecuta una función después de N segundos, opcionalmente repetida; `run-with-idle-timer` espera a que Emacs esté idle. `cancel-timer` lo cancela. Para tareas asíncronas, `async-start` (paquete async) ejecuta código en un Emacs hijo y devuelve el resultado por callback.'
    ],
    code: { lang: 'elisp', src: '(let ((proc (start-process "eco" "*eco*" "cat")))\n  (set-process-filter proc\n    (lambda (_p s) (message "salida: %s" s)))\n  (set-process-sentinel proc\n    (lambda (_p e) (message "estado: %s" e)))\n  (process-send-string proc "hola\\n"))\n\n(run-with-timer 5 nil (lambda () (message "5s")))\n(run-with-idle-timer 1 t (lambda () (message "idle")))', cap: 'Proceso con filtro/sentinel y timers.' },
    under: [
      'Emacs es single-threaded: los procesos y timers se atienden en el bucle de eventos. Un filtro debe ser rápido; si bloquea, la UI se congela. `accept-process-output` permite esperar sin bloquear del todo. `sit-for` y `sleep-for` también ceden el control.',
      'Los sentinels reciben el proceso y un string de estado (`"finished\n"`, `"exited abnormally"`). No cambies el buffer del proceso dentro del sentinel sin cuidado. Los timers se ejecutan en el hilo principal; para trabajo pesado, usá un proceso externo o `async-start`.'
    ],
    trap: { bad: 'Hacer trabajo pesado dentro de un filtro o timer y congelar Emacs.', good: 'Delegar a un proceso externo y procesar la salida en chunks; usar `async-start` para trabajo aislado.', why: 'El filtro corre en el hilo principal; bloquearlo bloquea toda la interfaz.' },
    q: [
      { q: '¿Qué recibe un process filter?', o: ['La salida del proceso', 'El buffer', 'La tecla', 'El tema'], a: 0, why: 'El filtro recibe el proceso y el string de salida.' },
      { q: '¿Qué hace `run-with-idle-timer`?', o: ['Ejecuta cuando Emacs está idle', 'Cierra Emacs', 'Compila', 'Hace commit'], a: 0, why: 'Programa una función para cuando no hay actividad.' }
    ],
    ex: { p: 'Lanzá `cat`, enviale `hola` y mostrá la salida con un filtro; programá un timer de 3 segundos.', start: '(let ((p (start-process "cat" "*cat*" "cat")))\n  (set-process-filter p (lambda (_ s) (message "%s" s)))\n  (process-send-string p "hola\\n"))\n(run-with-timer 3 nil (lambda () (message "listo")))', sol: '(let ((p (start-process "cat" "*cat*" "cat")))\n  (set-process-filter p (lambda (_ s) (message "%s" s)))\n  (process-send-string p "hola\\n"))\n(run-with-timer 3 nil (lambda () (message "listo")))', hint: 'En Windows, `cat` puede no existir; probá con `python -c` o `more`.' },
    cards: [['¿Qué es un process sentinel?', 'Una función llamada cuando el proceso cambia de estado.'], ['¿Qué hace `run-with-timer`?', 'Ejecuta una función después de un tiempo, opcionalmente repetida.']],
    tags: ['elisp', 'procesos', 'timers', 'experto']
  }
];
