#include <stdint.h>
typedef struct { double x; double y; } Punto;
double distancia2(const Punto *a, const Punto *b) {
    double dx = a->x - b->x, dy = a->y - b->y;
    return dx*dx + dy*dy;
}
int32_t suma_array(const int32_t *v, uintptr_t n) {
    int32_t s = 0;
    for (uintptr_t i = 0; i < n; i++) s += v[i];
    return s;
}
typedef int32_t (*Callback)(int32_t);
int32_t aplica_dos_veces(Callback f, int32_t x) { return f(f(x)); }
