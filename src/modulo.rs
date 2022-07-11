pub mod  modulo {
    // Los elementos de los módulos tienen visibilidad privada de forma predeterminada.
    fn funcion_privada() {
        println!("`mi_mod::funcion_privada()` llamada");
    }

    // Utiliza el modificador `pub` para anular la visibilidad predeterminada.
    pub fn funcion() {
        println!("`mi_mod::funcion()` llamada");
    }

    // Los elementos pueden acceder a otros elementos en el mismo módulo, incluso cuando
    // son privados.
    pub fn acceso_inderecto() {
        print!("`mi_mod::acceso_inderecto()` llamada,\n> ");
        funcion_privada();
    }

    // Los módulos también se pueden anidar
    pub mod anidado {
        pub fn funcion() {
            println!("`mi_mod::anidado::funcion()` llamada");
        }

        #[allow(dead_code)]
        fn funcion_privada() {
            println!("`mi_mod::anidado::funcion_privada()` llamada");
        }

        // Las funciones declaradas usando la sintaxis `pub (in ruta)` solo son visibles
        // dentro de la ruta dada. `ruta` debe ser un módulo principal o antepasado
        pub(in crate::modulo) fn funcion_publica_en_mi_mod() {
            print!("`mi_mod::anidado::funcion_publica_en_mi_mod() llamada`,\n> ");
            funcion_publica_en_anidado();
        }

        // Las funciones declaradas usando la sintaxis `pub(self)` solo son visibles
        // dentro del módulo actual, que es lo mismo que dejarlas privadas
        pub(self) fn funcion_publica_en_anidado() {
            println!("`mi_mod::anidado::funcion_publica_en_anidado()` llamada");
        }

        // Las funciones declaradas usando la sintaxis `pub(super)` solo son visibles
        // dentro del módulo padre
        pub(super) fn funcion_publica_en_super_mod() {
            println!("`mi_mod::anidado::funcion_publica_en_super_mod()` llamada");
        }
    }

    pub fn llama_funcion_publica_en_mi_mod() {
        print!("`mi_mod::llama_funcion_publica_en_mi_mod()` llamada,\n> ");
        anidado::funcion_publica_en_mi_mod();
        print!("> ");
        anidado::funcion_publica_en_super_mod();
    }

    // pub(crate) hace que las funciones sean visibles solo dentro del crate actual
    pub(crate) fn funcion_publica_en_crate() {
        println!("`mi_mod::funcion_publica_en_crate()` llamada");
    }

    // Los módulos anidados siguen las mismas reglas de visibilidad
    mod anidado_privado {
        #[allow(dead_code)]
        pub fn funcion() {
            println!("`mi_mod::anidado_privado::funcion()` llamada");
        }

        // Los elementos principales privados seguirán restringiendo la visibilidad de
        // un elemento secundario, incluso si se declara como visible dentro de un
        // ámbito más amplio.
        #[allow(dead_code)]
        pub(crate) fn funcion_restringida() {
            println!("`mi_mod::anidado_privado::funcion_restringida()` llamada");
        }
    }
}