## Rust Program Finite Automaton

  <li>Dentro de los usos que se le pueden dar a las máquinas de estados, y en particular a los AFD, está el reconocimiento de cadenas. Para realizar este reconocimiento en forma precisa y automatizada, el mismo puede implementarse en cualquier lenguaje de programación.
 </li>
  <li>Será posible que habiendo diseñado un autómata que sea capaz de reconocer un conjunto de cadenas de un lenguaje, construir un programa que implemente dicho autómata en algún lenguaje de programación, a tal fin el Algoritmo de funcionamiento del programa puede ser obtenido a partir del AFD en forma directa. 
  </li>
  
<p> </p>
Te evalua expresiones como 1+2.
<p> </p>
La matriz de reglas es:
<p> </p>


| Estado | Digitos | Operadores | Fin de cadena |
| --- | ---: | :---: |  :---: |
| q0 | q1 | Error |  Error |
| q1 | Error | q2 | Error |
| q2 | qf | Error | Error |
| qf | error | Error | Aceptacion |

</ol>
En este ejemplo validaremos una expresión aritmética ejemplo 12+3 o tal vez 23*3/5-8+1, sea cual sea la expresión nuestro autómata será capaz de decidir si es o no una expresión aritmética, para ello hay que crear todo desde cero.

### Objetivos
 1. Desarrollar un autómata finito que valide cuatro operaciones matematicas con números del 0 al 9
 2. Utilizar expresiones regulares para poder validar las cadenas de entrada

### Expresiones regulares
Se utilizaron las siguientes expresiones:
1. (0-9) para representar los números del 0 al 9
2. (+|-|*|/) para representar los distintos operadores matematicos (suma, resta, multiplicación y división)

### ¿Como solucionaste el problema?
Se soluciono utilizando la libreria Regex, la cual nos permite utilizar expresiones regulares.

## Problemas al programar

Realmente no se tuvo ningun problema en la programación, el problema fue en la parte de compilar el programa, ya que anteriormente estaba usando un compilador online, 
investigando el como usar expresiones regulares me encontre que unicamente se podia usar con un compilador nativo o instalado en nuestra computadora mediante linea de comandos
por lo que se tuvo que instalar un compilador, posteriormente se intento compilar el programa pero mostro otro error el cual es ```error: linking with `link.exe` failed: exit code: 1```
el cual no me permitia compilar el programa, al investigar como resolverlo me encontre con que se necesitaban otras librerias de compilacion de visual studio, las cuales son
```C++ build tools``` una vez instaladas ya se pudo compilar el programa sin ningun problema.

## License
[MIT](https://choosealicense.com/licenses/mit/)