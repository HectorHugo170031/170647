## Teoria de conjuntos
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoria computacional
##### Profesor: González Ibarra Juan Carlos
##### Alumno: Rodríguez Abella David Ulises 
##### Matricula: 170647
##### Carrera: Ingeniería en Tecnologías de la Información

## Programación en Rust
Los tres operadores lógicos básicos son O, Y y NO, representados en Python por or, and y not, respectivamente.
Podemos incluir también el O EXCLUSIVO, que es verdadero cuando uno y solo uno de los operandos lo es, pero estrictamente debes saber que se deriva a partir de los tres básicos. Su representación es ^, el sombrero o caret.
Como ejercicio práctico, vamos a realizar un programa que construya las tablas de verdad correspondientes.

La variable x recorrerá la lista booleanos, tomando en la primera iteración el valor False y en la siguiente True. 
Pero, por cada iteración, aparece una nueva variable y que también recorrerá booleanos de izquierda a derecha. 
Así garantizamos que se alcanzan las cuatro combinaciones posibles de x e y.
En la impresión con print, hemos empleado el argumento sep = ‘t’ para que separe cada elemento mediante un tabulador, 
en lugar de usar un espacio en blanco, valor por omisión. Aprecia el uso de la expresión x or y para que muestre el resultado del or.
El resto de las tablas se calcula del mismo modo, simplemente teniendo en cuenta que hay que emplear, naturalmente, 
la expresión lógica adecuada.
Debes saber que los operadores lógicos de Python son del tipo cortocircuitados, término que quizás te resulte familiar 
si conoces otros lenguajes de programación. Esto significa que, si a partir del primer operando ya se puede deducir el 
resultado final, Python ni se molestará en evaluar el segundo, con el consiguiente ahorro de tiempo.
En un or, si el primer operando es verdadero, sabemos que el resultado lo será ya, por lo que no es necesario que Python 
se moleste en comprobar la veracidad del segundo.
Del mismo modo, en un and, si el primer operando es falso, el resultado inmediatamente lo será y tampoco será necesario 
saber lo que ocurre con el segundo.
Para finalizar, una pequeña advertencia: es un error común confundir los operadores lógicos (or y and) con los operadores 
de unión e intersección de conjuntos (| y &).

## Objetivos
Realizar operaciones logicas con valores booleanos las cuales son:
1. AND
2. OR
3. NOT
4. ^

## ¿Como se solucionó el problema? 
Se utilizaron bucles de repetición los cuales fueron for each y dentro de estos se realizó la impresion y la operación

## Comandos para ejecutar las operaciones
Las estructuras de repeticion for each anidadas son las siguientes:
```rust
for x in booleanos.iter(){
    for y in booleanos.iter(){
        //Codigo para imprimir y realizar la operación logica
    }
  }
```
Las operaciónes fueron AND, OR, NOT y ^ por lo que unicamente se necesito una linea de codigo para cada operación
```rust
//Operacion OR
println!("{}\t{}\t{}",x,y,(x|y));
//Operacion AND
println!("{}\t{}\t{}",x,y,(x & y));
//Operacion NOT
println!("{}\t{}",x,!x);
//Operacion ^
println!("{}\t{}\t{}",x,y,(x^y));
```


## Problemas y soluciones al programar
En esta ocacion no hubo ningun problema al programar.

## License
[MIT](https://choosealicense.com/licenses/mit/)