---
titulo: "¿Es compatible?"
mundo: 12-modulos-crates-y-cargo
tema: dependencias
dificultad: media
---

Implementa la regla por defecto de Cargo para los requisitos de versión (`^`).

Escribe `compatible(requisito: &str, version: &str) -> bool`. Los dos textos tienen la forma `MAYOR.MENOR.PARCHE` (siempre tres números). Una versión cumple el requisito si:

1. es **mayor o igual** que el requisito, y
2. no cambia el primer número distinto de cero del requisito:
   - si MAYOR > 0: misma MAYOR (`1.4.0` acepta `1.9.2`, no `2.0.0`);
   - si MAYOR = 0 y MENOR > 0: misma MAYOR y MENOR (`0.9.1` acepta `0.9.7`, no `0.10.0`);
   - si MAYOR = 0 y MENOR = 0: exactamente la misma versión (`0.0.3` solo acepta `0.0.3`).

<details>
<summary>Pista</summary>

Convierte cada versión en una tupla `(u64, u64, u64)`: las tuplas se comparan en orden lexicográfico, así que `v >= r` funciona directamente.
</details>
