// PhantomData<*const ()> quita Send y Sync: el tipo queda atado a su hilo.
use std::marker::PhantomData;

struct SoloEsteHilo {
    id: u32,
    _no_send: PhantomData<*const ()>,
}

fn exige_send<T: Send>(_: T) {}

fn main() {
    let t = SoloEsteHilo { id: 1, _no_send: PhantomData };
    let _ = t.id;
    exige_send(t);
}
