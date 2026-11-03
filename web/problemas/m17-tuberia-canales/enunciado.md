---
titulo: "Una tubería con canales"
mundo: 17-concurrencia
tema: canales
dificultad: media
---

Monta una tubería de dos etapas con canales de `std::sync::mpsc`:

1. La etapa 1 recibe números por `rx1`, se queda con los **pares**, los multiplica por 10 y los envía por `tx2`.
2. Cuando `rx1` se cierra, la etapa 1 tiene que **cerrar** `tx2` para que quien lee el resultado sepa que ha terminado.

Escribe `etapa(rx1: Receiver<u32>, tx2: Sender<u32>)` y `tuberia(entrada: &[u32]) -> Vec<u32>`, que crea los dos canales, envía la entrada, cierra el primer emisor, ejecuta la etapa y recoge todo lo que sale.

(En un programa real, cada etapa iría en su hilo; aquí se ejecuta todo en orden, porque en el navegador no hay hilos.)

<details>
<summary>Pista</summary>

`for x in rx1 { … }` termina cuando todos los `Sender` del canal se han destruido. Al recibir `tx2` por valor, la etapa lo destruye al terminar, y eso cierra el segundo canal. Para cerrar el primero, `drop(tx1)` antes de llamar a la etapa.
</details>
