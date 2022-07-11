use std::fmt;

mod modulo;
mod Vstruct;

fn main() {
   /* // Esto es un comentario doc -> con rustdoc podemos generar un documento
    println!("Hello, world!");

    //Tipo variables y constantes
    let mut x = 0; //Variable mutable
    let y = 0; //Variable inmutable, a no ser que se vuelva a declarar la variable
    let variable: String = String::from("a");
    const DATABASE: &str = "/var/lib/tracker/tracker.json";

    let tarifa: f32 = 26.90; // De 32 bytes (Con anotación de tipo explícita)
    let haystock: bool = true; // Con anotación de tipo explícita
    let pais = "Perú";

    // Tuplas
    let precios = (3.50, 4.30, 49.99, 23.50, 99.99);

    //Arrays
    let anios = [2020, 2021, 2022, 2023, 2024];
    println!("{}", x + 2);

    //Concatenar String
    let owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";
    let new_owned_string = owned_string + borrowed_string;

    let a = "a";
    let b = "b";
    let result = [a, b].join("/n");

    let a = assert_eq!(0, 0, "we are testing addition with {} and {}", 0, 0);

    let s = concat!("test", 10, 'b', true);
    println!("{}", s);


    //Condicionales
    let n = 5;

    if n < 0 {
        print!("{} es negativo", n);
    } else if n > 0 {
        print!("{} es positivo", n);
    } else {
        print!("{} es zero", n);
    }

    //Tipos de bucle
    let mut i = 0;
    loop {
        println!("{}", i);
        if i > 4 {
            break;
        }
        i += 1;
    }

    'externo: loop {
        'interno: loop {
            println!("Entró en el bucle interno");
            break 'externo;
        }
    }

    //Recogiendo datos de un loop
    let myLoop = loop {
        break 5;
    };

    for n in anios.iter() {
        println!("{}", n)
    }

    let nombres = vec!["Bob", "Frank", "Ferris"];

    // Se puede hacer uso de los métodos
    //.into_iter()
    //.iter_mut()
    for nombre in nombres.iter() {
        match nombre {
            &"Ferris" => println!("¡Hay un rustáceo entre nosotros!"),
            _ => println!("Hola {}", nombre),
        }
    }

    println!("nombres: {:?}", nombres);

    //Match funciona similar a un switch
    match n {
        // Coincide con un solo valor
        1 => println!("¡Uno!"),
        // Coincide con varios valores
        2 | 3 | 5 | 7 | 11 => println!("Este es un primo"),
        // Coincide con un rango inclusivo
        13..=19 => println!("Un adolescente"),
        // Manejar el resto de casos
        _ => println!("No es especial"),
    }

    //Llamando a una función
    println!("{}", sum(5, 10));
*/

    //struct = clase
    //trait = interface
/*
    trait MyInterface{

        fn Saludar(&self) -> ();

    }

    #[derive(Debug)]
    struct Rectangulo {
        p1: Punto,
        p2: Punto,
    }



    impl Rectangulo {
        // Este es un método de instancia
        // `&self` es azúcar sintáctica para `self: &Self`, donde `Self` es el tipo
        // del objeto llamador. En este caso, `Self` = `Rectangulo`
        fn area(&self) -> f64 {
            // `self` da acceso a los campos de la estructura a través del operador
            // de punto
            let Punto { x: x1, y: y1 } = self.p1;
            let Punto { x: x2, y: y2 } = self.p2;

            // `abs` es un método de `f64` que devuelve el valor absoluto del
            // objeto llamador
            ((x1 - x2) * (y1 - y2)).abs() // Lleva un return implicito
        }

        fn new(p1: Punto, p2: Punto) -> Rectangulo {
            Rectangulo {
                p1,
                p2
            }
        }
    }

    #[derive(Debug)]
    struct Punto {
        x: f64,
        y: f64,
    }


    impl Punto {
        fn origen() -> Punto {
            Punto { x: 0.0, y: 0.0 }
        }

        // Otro método estático, que toma dos argumentos:
        fn new(x: f64, y: f64) -> Punto {
            Punto { x, y }
        }

    }

    impl MyInterface for Punto{
        fn Saludar() -> () {
          // println!("{:?}", self);
        }
    }

    let t = Rectangulo::new(
        Punto::new(56.3, 89.3),
        Punto::new(56.3, 89.3)
    );

    impl MyInterface for Rectangulo{
        fn Saludar(&self) -> () {
            println!("{:?}", self.);
        }
    }*/

    //Rectangulo::Saludar(&t); === t.Saludar()

    let x = 5;
    let y = &x;

    println!("{}", x);


    //println!("{:?}", Punto::Saludar(&t));
/*
    impl fmt::Debug for Punto {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

            let mut resp:core::fmt::Result =  core::result::Result::Ok(());
            return resp;
        }
    }*/




/*
    let p = Rectangulo::new(Punto::origen(), Punto::new(5.4, 3.2));

    println!("{:?}", p);*/
    /*
    use crate::modulo::modulo::anidado::funcion;
    funcion();

    use crate::Vstruct::mi;

    let caja_abierta = mi::CajaAbierta{contenidos:"Cosas de rust :)"};
    println!(caja_abierta.contenidos);

    let _caja_cerrada = mi::CajaCerrada::new("información clasificada");

}

fn sum(x: i32, y: i32) -> i32 {
    return x+y;
}

fn hello() -> ! {
    //Funcion divergente no retornan nada
    panic!("Hello World");
}
*/
}