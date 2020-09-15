## Teoria de conjuntos - programación en Rust
  <li>1. In this example, we have defined two set variables and we have performed different set operations: union, intersection, difference and symmetric difference.,  </li>
  <li>2. To understand this example, you should have the knowledge of the following Python programming topics:  </li>
  <li>3.Python Sets  </li>
  <li>4. Python Input, Output and Import  </li>
  <li>Python offers a datatype called set whose elements must be unique. It can be used to perform different set operations like union, intersection, difference and symmetric difference. </li>
  <li>Is the Quadratic Allocation Problem NP-complete or is it in P? Either give a reduction to show it is NP-complete or give a polytime algorithm to solve it. </li>
</ol>
Aside: This problem arose during some consulting I was doing, where the integers represented the sizes of different software jobs, and the quadratic term is there because the cost of implementing software goes up faster than linearly with the size of the job. 
<p></p>

## Objetivos
Realizar operaciones con conjuntos realizando un programa en Rust. Las operaciones son las siguientes:
1. Union
2. Intersección
3. Diferencia
4. Diferencia simetrica
5. SubConjuntos
6. SuperConjuntos 

## ¿Como se solucionó el problema? 
Se utilizó la libreria de Rust la cual es ```std::collections::HashSet```, de esta clase se utilizaron algunos métodos para realizar las operaciones directamente a los conjuntos,
para poder programar los conjuntos se utilizo el tipo de dato HashSet el cual contiene diversos métodos propios como lo puede ser ```iter()```, ```collect()```, etc. Esto nos 
facilita el como se utilizan los conjuntos.


## Comandos para ejecutar las operaciones
Se utilizaron diversos métodos para realizar las operaciones de conjuntos.

Primero se declararon los 3 conjuntos y se imprimieron en pantalla para visualizar los elementos de los conjuntos
```rust
    //Se declaran 3 conjuntos
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    let conj_C: HashSet<i32> = HashSet::new();
    //Se imprimen los 3 conjuntos en consola 
    println!("Conjunto A: {:?}", conj_A);
    println!("Conjunto B: {:?}", conj_B);
    println!("Conjunto C: {:?}", conj_C);
```
La primera operación de conjuntos se hizo fue la de saber si un elemento pertenecia a determinado conjunto, esto se realizó
con el método ```contains(&n)``` donde n es el elemento que se quiere saber si pertenece al conjunto, este método regresa un valor
booleano donde es true cuando pertenece al conjunto y false cuando no pertenece.
```rust
    //1 en conjunto A
    println!("El conjunto A contiene #1: {}",conj_A.contains(&1));
    //1 no esta en conjunto A 
    println!("El conjunto A no contiene #1:{}",!conj_A.contains(&1));
    //10 en conjunto A 
    println!("El conjunto A contiene #10: {}",conj_A.contains(&10));
    //10 no esta en conjunto A 
    println!("El conjunto A no contiene #10: {}",!conj_A.contains(&10));
```
La segunda operación de conjuntos es el convertir algun tipo de dato a un conjunto, en este caso fue un vector, arreglo y cadena o string
este paso fue muy fácil para convertir el vector y arreglo ya que unicamente se utilizaron los métodos ```iter().collect()``` y se convertia automaticamente
a un conjunto pero con las cadenas se utilizo un ciclo for each por lo que esto fue un poco más complicado como se muestra en el siguiente fragmento de codigo:
```rust
    let conj_c = "Hola Mundo"; 
    //Para un string se debe de separar cada caracter utilizando 
    //un for each para iterar en cada caracter que tiene la cadena
    //y se agregan al conjunto creado
    let mut conj_C: HashSet<char> = HashSet::new();
    for caracter in conj_c.chars(){
        conj_C.insert(caracter);
    }
```
Las siguientes operaciones fueron más sencillas de usar ya que la libreria nos otorga los métodos para unicamente utilizarlos
directamente con los conjuntos, los conjuntos estan representados con las variables conj_A, conj_B y conj_C, las operaciónes realizadas
fueron, agregar elemento al conjunto, limpiar conjunto, eliminar elemento del conjunto, copiar, union, intersección, diferencia, diferencia simetrica,
subconjunto y super conjunto, estas ultimas dos unicamente regresa el valor de true o false lo que significa que si es o no un subconjunto o superconjunto.
n  es el valor que se va a agregar o eliminar.
```rust
    conj_A.remove(&n);
    conj_A.clear();
    conj_A.clone();
    conj_B.insert(&n);
    conj_A.union(&conj_B);
    conj_A.intersection(&conj_B);
    conj_A.difference(&conj_B);
    conj_A.symmetric_difference(&conj_B);
    conj_A.is_subset(&conj_B);
    conj_A.is_superset(&conj_B);
```

## Problemas y soluciones al programar
El problema que se tuvo al programar fue que para las cadenas no había un metodo que convierta directamente a un conjunto y se
tuvo que utilizar un ciclo for each para iterar en cada letra de la cadena y así poder insertarla en el conjunto.
```rust
    let conj_c = "Hola Mundo"; 
    //Para un string se debe de separar cada caracter utilizando 
    //un for each para iterar en cada caracter que tiene la cadena
    //y se agregan al conjunto creado
    let mut conj_C: HashSet<char> = HashSet::new();
    for caracter in conj_c.chars(){
        conj_C.insert(caracter);
    }
```

## License
[MIT](https://choosealicense.com/licenses/mit/)