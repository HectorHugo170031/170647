##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoria computacional
##### Profesor: González Ibarra Juan Carlos
##### Alumno: Rodríguez Abella David Ulises 
##### Matricula: 170647
##### Carrera: Ingeniería en Tecnologías de la Información
## Programación palindromo en Rust 
### ¿Qué es un palindromo?
¿Qué es un palíndromo? Muy sencillo: una palabra o frase que se lee igual de izquierda a derecha que de derecha a izquierda. 
Para que lo entiendas mejor, aquí tienes un ejemplo clásico, el que te enseñaban hace años en el cole para explicártelo, 
y que recoge el diccionario de la RAE.
  
### Objetivos
 1. Desarrollar un programa que resuelva un palindromo utilizando una estructura de datos que en este caso es una lista


### Palindromo
Se utilizo el siguiente palindromo
```rust  
	let palabra: Vec<char>;
	palabra = vec!['A','N','I','T','A','L','A','V','A','L','A','T','I','N','A'];
```
## Source Code

```rust
//Libreria para utilizar las listas 
use std::collections::LinkedList;
fn main() {
  //Se define la variable que va a contener la palabra o palindromo
  let palabra: Vec<char>;
  palabra = vec!['A','N','I','T','A','L','A','V','A','L','A','T','I','N','A'];
  //Se crea una lista enlazada para almacenar los valores del palindromo
  let mut list1 = LinkedList::new();
  //Se almacena el valor de la palabra diviendo entre dos para saber donde dejar de almacenar en la lista 
  let mut tamanio = palabra.len()/2 ;
  /*
	Se utiliza un ciclo for para almacenar el valor que se tiene en la palabra en la lista
	Esto va a ir iterando desde 0 hasta tamaño de la cadena dividido entre dos más uno, esto para hacer el arreglo en caso de que el 
	palindromo sea impar  
  */
  for i in 0..tamanio+1 {
    //Se imprime la letra que se tiene del palindromo
    print!("Some({:?})", palabra[i]);
    //Se imprime el valor que tiene la lista
    println!("\t{:?}", list1);
    //Se almacena el valor en la lista 
    list1.push_back(palabra[i]);
  }
  //Se le hace un pop de la lista al ultimo elemento guardado ya que se imprimio en pantalla y no es necesario almacenarlo y mostrarlo denuevo
  list1.pop_back();
  for i in 0..tamanio {
    //Se imprime el valor que se extrae de la lista de la parte trasera como una pila o utilizando LIFO
    print!("{:?}", list1.pop_back());
    println!("\t{:?}", list1);
  }
  //Se imprime toda la palabra o palindromo
  println!("Palabra: {:?}", palabra);    
}

```

Este programa como salida se tiene

Some('A')       []
Some('N')       ['A']
Some('I')       ['A', 'N']
Some('T')       ['A', 'N', 'I']
Some('A')       ['A', 'N', 'I', 'T']
Some('L')       ['A', 'N', 'I', 'T', 'A']
Some('A')       ['A', 'N', 'I', 'T', 'A', 'L']
Some('V')       ['A', 'N', 'I', 'T', 'A', 'L', 'A']
Some('A')       ['A', 'N', 'I', 'T', 'A', 'L']
Some('L')       ['A', 'N', 'I', 'T', 'A']
Some('A')       ['A', 'N', 'I', 'T']
Some('T')       ['A', 'N', 'I']
Some('I')       ['A', 'N']
Some('N')       ['A']
Some('A')       []
Palabra: ['A', 'N', 'I', 'T', 'A', 'L', 'A', 'V', 'A', 'L', 'A', 'T', 'I', 'N', 'A']

Por lo que si queda resuelto el problema de los palindromos.
Al utilizar la teoria de las pilas se tiene que en este programa se usa LIFO para eliminar los datos de la lista y así poder imprimirlos del ultimo elemento al primero.

## Problemas al programar
De problemas que se tuvo fue que al momento de querer imprimir el valor que se iba borrando de la lista mostraba el mensaje Some(char) y realmente no se encontro una solucion
por otra parte fue entender el como se implementan las listas en Rust ya que anteriormente se vio en lenguaje C y ya se tenia la logica solo era entender la sintaxis.
Para utilizar las listas se utilizo la libreria ```std::collections::LinkedList``` para acceder a todos los metodos.


### ¿Como solucionaste el problema?
Se soluciono utilizando la libreria ```std::collections::LinkedList```

## License
[MIT](https://choosealicense.com/licenses/mit/)