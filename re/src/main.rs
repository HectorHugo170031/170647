/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       23/10/2020
 *  Ultima actualización:  23/10/2020
 *----------------------------------------------------------------------*/
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
		    aux_tokens = format!("{}{}","END_STATEMENT ",";");
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