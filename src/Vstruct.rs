pub(crate) mod mi {
    // Una estructura pública con un campo público de tipo genérico `T`
    pub struct CajaAbierta<T> {
        pub contenidos: T,
    }

    // Una estructura pública con un campo privado de tipo genérico `T`
    #[allow(dead_code)]
    pub struct CajaCerrada<T> {
        contenidos: T,
    }

    impl<T> CajaCerrada<T> {
        // Un método constructor público
        pub fn new(contenidos: T) -> CajaCerrada<T> {
            CajaCerrada {
                contenidos: contenidos,
            }
        }
    }
}