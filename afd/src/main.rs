/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       06/10/2020
 *  Ultima actualización:  06/10/2020
 *----------------------------------------------------------------------*/
//Dependencia para expresiones regulares
extern crate regex;
//Librerias
use std::io;
use regex::Regex;
use std::process;

//Funcion para verificar cada caracter de la cadena de entrada
fn caracter(character: char) -> i32 {
    //Variables con las cuales se va a comparar el final de la cadena
    //los números del 0 al 9
    //y el operador para saber si se va a sumar, restar, dividir, multiplicar
    let mut Fin="";
    let digito = Regex::new(r"[0-9]").unwrap();
    let operador =  Regex::new(r"(\+|\-|\*|/)").unwrap();
    let mut s = String::from("");
	s.push(character);

	let caracterN: &str = &s[..];

    //Se compara si es un digito
    if digito.is_match(caracterN){
	//Retorna un cero en caso de que sea un digito
        return 0;
    }
    else{
            //En caso de que no sea un digito entra a la parte de verificar si es un operador
	    if operador.is_match(caracterN){
		//En caso de ser un operador retorna un 1
	        return 1;
	    }
	    else {
		//En caso de no ser ninguno de los dos anteriores se compara si es el final de la cadena
	    	if caracterN==Fin {
			//En caso de ser el final de la cadena retorna el número 2
	       		return 2;
	    	}
	    }
	    //En caso de que no sea ninguno de los casos anteriores se muestra un mensaje que no es valido
	    //y se sale del programa para volver a iniciar 
	    println!("Error el caracter: {} no es valido",character);
	    process::exit(0x0100);
    }    
}

//Funcion para imprimir el encabezado de la tabla
fn encabezado(){
    println! ("|  Estado actual |Caracter |  Simbolo  |Estado siguiente |");
    //Se manda a llamar la funcion para comenzar a imprimir el cuerpo de la tabla
    body();
}

//definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
fn contenido(estadoSig: i32,caracter: char,simbolo: &str,estado: i32){
    println! ("|      {}       |    {}    |  {} |      {}        |",estadoSig,caracter,simbolo,estado);
    //Se manda a llamar la funcion para comenzar a imprimir el cuerpo de la tabla
    body();
}
//Solo imprime la linea para dividir cada fila de la tabla
fn body(){
    println! ("+----------------+---------+-----------+----------------+");
}


fn main(){
    //Se crea un vector doble para utilizarlo como tabla de transiciones E=-100, A=-200 
    let tabla: Vec<Vec<i32>>;
    tabla = vec![
                vec! [1,-100,-100],
                vec! [-100,2,-100],
                vec! [3,-100,-100],
                vec! [-100,-100,-200]
            ];
    //Variables para imprimir los estados, simbolos y cadena 
    let mut estado: i32 = 0;
    let mut simbolo: String = "".to_string();
    println! ("+-------------------------------------+");
    println! ("|    Ingrese una cadena a evaluar:    |");
    println! ("+-------------------------------------+");
    //Variable cadena para lectura desde el teclado
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);
    //Se manda a llamar la funcion de cuerpo y encabezado 
    body();
    encabezado();

    //Estructura de repeticion para avanzar de caracter en caracter
    for  character in cadena.chars(){
        let mut estadoSig: i32 = estado;

        //Mandamos a llamar la funcion caracter para saber si es un caracter valido o invalido
	//el valor que retorna es en caso de ser caracter, operador, final o si no es valido
        let mut charCaracter= caracter(character);

        //Se almacena el estado en la tabla de acuerdo a las coordenadas que se obtuvieron
        estado = tabla[estado as usize][charCaracter as usize];

        //Se compara el valor recibido de la funcion caracter para saber si es simbolo u operador
      	if charCaracter == 0{
      	    simbolo = " Digito ".to_string();
      	}
      	else if charCaracter == 1{
      	    simbolo = "Operador".to_string();
      	}


        //Si el valor obtenido es una E imprimimos cadena no valida
        if estado==-100{
            println! ("|    {}    |  {}    | Error |     {}       |",estadoSig,character,estado);
            body();
            println! ("|              Cadena No Valida :(                   |");
            println! ("+----------------------------------------------------+");
        }
	//Mandamos a llamar la funcion contenido la cual va a imprimir todo lo que debe contener la tabla
        contenido(estadoSig,character,&simbolo,estado);
	//Para que sea un estado correcto se debe de tener el número 3 para poder imprimir que la cadena es valida
	//en caso de que no sea 3 se imprime que la cadena no es valida     
	if estado>3{
	   println! ("|              Cadena no valida                    |");
	   println! ("+----------------------------------------------------+");
	}
	//si el estado es 3 es una cadena de aceptacion
	if estado==3{
	   println! ("|      {}       |         |Fin cadena |               |",estado);
	   body();
	   println! ("|                Cadena valida                       |");
	   println! ("+----------------------------------------------------+");
	}  
    }  
}