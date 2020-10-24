##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoria computacional
##### Profesor: González Ibarra Juan Carlos
##### Alumno: Rodríguez Abella David Ulises 
##### Matricula: 170647
##### Carrera: Ingeniería en Tecnologías de la Información
## Programación expresiones regulares 
### Importancia de los lenguajes regulares.
  <li>La existencia y formalización de los lenguajes regulares (LR) y su vinculación con otros artefactos lingüísticos-matemáticos ya bien formalizados, estudiados e incluso llevados a la práctica ha sido de vital importancia en el ulterior desarrollo de los mecanismos de procesamiento de lenguajes de computadora, fundamentalmente en los analizadores lexicográficos gracias a la posibilidad de derivar el reconocimiento de los LR mediante autómatas finitos que son fáciles de implementar computacionalmente con mecanismos simples y rápidos, óptimos en la obtención de parsers veloces y robustos que a su vez le ofrecen a los desarrolladores información detallada de los errores léxicos, sintácticos e incluso advierten sobre errores semánticos.
Lo mismo sucede por ejemplo con las expresiones regulares implementadas ya en muchos Lenguaje de programación de propósito general modernos que permiten a los desarrolladores de lenguajes mecanismos muy eficientes para la obtención intuitiva de partes de compiladores que reconocen los tókenes o partículas lexicales del código fuente como fase del proceso completo de interpretación o compilado, según sea el caso.
A manera de ilustración en el caso del ejemplo anterior vemos la siguiente función Python que a partir de cualquiera de los elementos formalizadores del LR correspondiente permite decidir si un texto w es una variable PROLOG:  </li>

### Objetivos
 1. Desarrollar un programa que simule la funcion de un compilador para identificar las palabras que se usan como identificador, tipo de dato, operador y digito.
 2. Utilizar expresiones regulares para poder validar las cadenas de determinado lenguaje


### Expresiones regulares
Se utilizaron las siguientes expresiones:
```rust  
	let digito = Regex::new(r"[0-9]").unwrap();
	let alfabetoMin = Regex::new(r"[a-z]").unwrap();
	let alfabetoMay = Regex::new(r"[A-Z]").unwrap();
```
1. (0-9) para representar los números del 0 al 9 
2. (a-z) y (A-Z) para representar el alfabeto en mayusculas y minusculas

## Source Code

```rust
//Dependencia para expresiones regulares
extern crate regex;
//Librerias
use regex::Regex;

fn main(){
  //Variables para almacenar la instruccion de otro lenguaje, tokens y todas las expresiones regulares
	let mut tokens: Vec<String> = Vec::new();
	let  source_code = "int result = 100;".split(" ");
	let types: [&str; 3] = ["str", "int", "bool"];
	let digito = Regex::new(r"[0-9]").unwrap();
	let alfabetoMin = Regex::new(r"[a-z]").unwrap();
	let alfabetoMay = Regex::new(r"[A-Z]").unwrap();
  let operadores = Regex::new(r"(\+|\-|\*/)").unwrap();
	// Estructura de repeticion para iterar de letra en letra de la cadena de codigo fuente
	for word in source_code{
    //Variable auxiliar para almacenar los tokens
    let mut aux_tokens: String = "".to_string();
    //Se valida que se tenga el tipo de dato declarado en la variable source_code
    //Si no es el tipo de dato se valida que sea el identificador/nombre de la variable 
    //Sino es tipo de dato ni identificador se valida que sea un operador
    //En caso contrario se valida si es un digito y se valida que sea el final del codigo con ";"
    if types.contains(&word){
      //Si se encuentra la declaracion del tipo de dato se almacena en el token con la palabra que se contiene en source_code
    	aux_tokens = format!("{}{}","DATATYPE: ",word);
    }else if alfabetoMin.is_match(word) || alfabetoMay.is_match(word){
      //Si se encuentra la declaracion del nombre se almacena en el token con la palabra que se contiene en source_code
			aux_tokens = format!("{}{}","IDENTIFIER: ",word);
		}else if operadores.is_match(word){
      //Si se encuentra la declaracion el operador se almacena en el token con la palabra que se contiene en source_code
		 	aux_tokens = format!("{}{}","OPERATOR: ",word);
		}else if digito.is_match(word){
		  let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
		  let mut aux_w2: String = word.to_string();
	    //Se valida que sea el fin de la cadena con punto y coma ";" o el número entero
		  if (aux_w as char) == ';'{
			  aux_w2.remove(word.len() - 1);
		    aux_tokens = format!("{}{}","INTEGER ", aux_w2);
		    tokens.push(aux_tokens);
		    aux_tokens = format!("{}{}","END_STATEMENT",";");
	    }else{
        aux_tokens = format!("{}{}","INTEGER ",word);
		  }
		}
	  if aux_tokens != ""{
		 	tokens.push(aux_tokens);
		}
	}
	for token in &tokens{
		//Imprime todo el token
	 	println!("{}", token);
	}
}

fn variable_prolog(w_string: String)->bool{
	let mut w: Vec<char> = Vec::new();
	for w_chars in w_string.chars(){ 
		w.push(w_chars);
  }
	let aux_w: Vec<_> = w[0].to_uppercase().collect();
  //Si retorna un verdadero es que el nombre de la variable es correcto 
	if w[0].is_alphabetic() && w[0] == (aux_w[0] as char) || w[0] == '_'{
	  //El primer caracter es una mayuscula o un subrayado
    //Se quita el primer caracter
	  w.reverse();
		w.pop();
		w.reverse();
	  while !w.is_empty() && (w[0].is_numeric() || w[0]  ==  '_'){
      //Se repetira el ciclo mientras w contenga caracteres y el primer caracter sea alfanumerico 
		  w.reverse();
			w.pop();
			w.reverse(); 
	    if w.is_empty(){
        //En caso de que la cadena w este vacia se regresa un true de que es correcto sino retorna false
        //Si ya no quedan elementos a revisar, es una variable PROLOG
	      return true;
      }
    }
  }                     
	return false;
}

```
  
<p> </p>
Varias funciones de esa naturaleza componen los analizadores lexicográficos para discriminar la función de cada token del lenguaje en cuestión: constante númerica, literal de cadena de texto, identificador, separador, etc.; según sea el caso.
Y también puede verse que existe una estrecha relación en cómo implementar las funciones reconocedoras de LR y la expresión o la gramática regulares o el autómata finito correspondiente, al punto que desde hace más de 3 décadas existen aplicaciones generadoras de analizadores lexicográficos como el Flex o el Bisonte.
Pero el alcance de los LR no se queda en el mundo de la compilación de lenguajes de ordenador como se ve en el siguiente ejemplo.
<p> </p>

## Programación generadora gramática regular

<p> </p>


<ol>
Analizador léxico simple
Ejemplo#
En este ejemplo, te mostraré cómo hacer un lexer básico que creará los tokens para una declaración de variable entera en Python.
¿Qué hace el analizador léxico?
El propósito de un lexer (analizador léxico) es escanear el código fuente y dividir cada palabra en un elemento de la lista. Una vez hecho esto, toma estas palabras y crea un par de tipo y valor que se parece a esto ['INTEGER', '178'] para formar un token.
Estos tokens se crean para identificar la sintaxis de su idioma, por lo que todo el objetivo del lexer es crear la sintaxis de su idioma, ya que todo depende de cómo desee identificar e interpretar los diferentes elementos.
</ol>

### Source Code

```rust

//Dependencia para expresiones regulares
extern crate regex;
//Librerias
use regex::Regex;

fn main(){
  //Variables para almacenar la instruccion de otro lenguaje, tokens y todas las expresiones regulares
	let mut tokens: Vec<String> = Vec::new();
	let  source_code = "int result = 100;".split(" ");
	let types: [&str; 3] = ["str", "int", "bool"];
	let digito = Regex::new(r"[0-9]").unwrap();
	let alfabetoMin = Regex::new(r"[a-z]").unwrap();
	let alfabetoMay = Regex::new(r"[A-Z]").unwrap();
  let operadores = Regex::new(r"(\+|\-|\*/)").unwrap();
	// Estructura de repeticion para iterar de letra en letra de la cadena de codigo fuente
	for word in source_code{
    //Variable auxiliar para almacenar los tokens
    let mut aux_tokens: String = "".to_string();
    //Se valida que se tenga el tipo de dato declarado en la variable source_code
    //Si no es el tipo de dato se valida que sea el identificador/nombre de la variable 
    //Sino es tipo de dato ni identificador se valida que sea un operador
    //En caso contrario se valida si es un digito y se valida que sea el final del codigo con ";"
    if types.contains(&word){
      //Si se encuentra la declaracion del tipo de dato se almacena en el token con la palabra que se contiene en source_code
    	aux_tokens = format!("{}{}","DATATYPE: ",word);
    }else if alfabetoMin.is_match(word) || alfabetoMay.is_match(word){
      //Si se encuentra la declaracion del nombre se almacena en el token con la palabra que se contiene en source_code
			aux_tokens = format!("{}{}","IDENTIFIER: ",word);
		}else if operadores.is_match(word){
      //Si se encuentra la declaracion el operador se almacena en el token con la palabra que se contiene en source_code
		 	aux_tokens = format!("{}{}","OPERATOR: ",word);
		}else if digito.is_match(word){
		  let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
		  let mut aux_w2: String = word.to_string();
	    //Se valida que sea el fin de la cadena con punto y coma ";" o el número entero
		  if (aux_w as char) == ';'{
			  aux_w2.remove(word.len() - 1);
		    aux_tokens = format!("{}{}","INTEGER ", aux_w2);
		    tokens.push(aux_tokens);
		    aux_tokens = format!("{}{}","END_STATEMENT",";");
	    }else{
        aux_tokens = format!("{}{}","INTEGER ",word);
		  }
		}
	  if aux_tokens != ""{
		 	tokens.push(aux_tokens);
		}
	}
	for token in &tokens{
		//Imprime todo el token
	 	println!("{}", token);
	}
}

```

Cuando se ejecuta este fragmento de código, la salida debe ser la siguiente:

```python
[['DATATYPE', 'int'], ['IDENTIFIER', 'result'], ['OPERATOR', '='], ['INTEGER', '100'], ['END_STATEMENT', ';']]

```
Vamos a descomponerlo
1.	Comenzamos con la importación de la biblioteca de expresiones regulares porque será necesario cuando se compruebe si ciertas palabras coinciden con un determinado patrón de expresiones regulares.

2.	Creamos una lista vacía llamada tokens. Esto se utilizará para almacenar todos los tokens que creamos.

3.	Dividimos nuestro código fuente, que es una cadena en una lista de palabras donde cada palabra en la cadena separada por un espacio es un elemento de la lista. Luego los almacenamos en una variable llamada source_code .

4.	Comenzamos a recorrer nuestra lista de source_code palabra por palabra.

5.	Ahora realizamos nuestro primer control:

```rust 
	if types.contains(&word){
      //Si se encuentra la declaracion del tipo de dato se almacena en el token con la palabra que se contiene en source_code
    	aux_tokens = format!("{}{}","DATATYPE: ",word);
    }else if alfabetoMin.is_match(word) || alfabetoMay.is_match(word){
      //Si se encuentra la declaracion del nombre se almacena en el token con la palabra que se contiene en source_code
			aux_tokens = format!("{}{}","IDENTIFIER: ",word);
		}else if operadores.is_match(word){
      //Si se encuentra la declaracion el operador se almacena en el token con la palabra que se contiene en source_code
		 	aux_tokens = format!("{}{}","OPERATOR: ",word);
		}else if digito.is_match(word){
		  let aux_w: u8 = word.as_bytes()[(word.len() - 1) as usize];
		  let mut aux_w2: String = word.to_string();
	    //Se valida que sea el fin de la cadena con punto y coma ";" o el número entero
		  if (aux_w as char) == ';'{
			  aux_w2.remove(word.len() - 1);
		    aux_tokens = format!("{}{}","INTEGER ", aux_w2);
		    tokens.push(aux_tokens);
		    aux_tokens = format!("{}{}","END_STATEMENT",";");
	    }else{
        aux_tokens = format!("{}{}","INTEGER ",word);
		  }
		}

```
Lo que buscamos aquí es un tipo de datos que nos dirá qué tipo de variable será nuestra variable.

8.	Después de eso, realizamos más verificaciones como la anterior, identificando cada palabra en nuestro código fuente y creando un token para ella. Estos tokens se pasarán al analizador para crear un árbol de sintaxis abstracta (AST).


## Problemas al programar

Realmente no se tuvo ningun problema en la programación, el problema fue en la parte de compilar el programa, ya que anteriormente estaba usando un compilador online, 
investigando el como usar expresiones regulares me encontre que unicamente se podia usar con un compilador nativo o instalado en nuestra computadora mediante linea de comandos
por lo que se tuvo que instalar un compilador, posteriormente se intento compilar el programa pero mostro otro error el cual es ```error: linking with `link.exe` failed: exit code: 1```
el cual no me permitia compilar el programa, al investigar como resolverlo me encontre con que se necesitaban otras librerias de compilacion de visual studio, las cuales son
```C++ build tools``` una vez instaladas ya se pudo compilar el programa sin ningun problema.


### ¿Como solucionaste el problema?
Se soluciono utilizando la libreria Regex, la cual nos permite utilizar expresiones regulares.

## License
[MIT](https://choosealicense.com/licenses/mit/)