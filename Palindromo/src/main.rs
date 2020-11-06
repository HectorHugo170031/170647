/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       06/11/2020
 *  Ultima actualización:  06/11/2020
 *----------------------------------------------------------------------*/
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