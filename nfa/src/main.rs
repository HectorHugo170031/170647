/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       15/10/2020
 *  Ultima actualización:  16/10/2020
 *----------------------------------------------------------------------*/
//Dependencia para expresiones regulares
extern crate regex;
//Librerias
use std::io;
use regex::Regex;
use std::process;

fn caracter(character: char, estado:i32) -> i32{
    //Variables con las cuales se va a comparar el final de la cadena 
    let fin="";
    let a = Regex::new(r"a").unwrap();  
    let b = Regex::new(r"b").unwrap();  
    //Convertir cadena a caracter
    let mut cadena = String::from("");
    cadena.push(character);
    let caracterN: &str = &cadena[..];
    /*
	Esta parte se trabajo en colaboracion con: Herrera Martinez Juan Humberto - 170201
    */
    //Estados correspondientes al vacio
    if estado == 0 || estado == 1 || estado == 3 || estado == 4 || estado == 6 || estado == 7 || estado == 9 || estado == 10 {
        return 0;
    } 
    //Estado final para mostrar mensaje que la cadena es valida
    if estado == 11{
        println! ("|      {}      |         |Fin Cadena |               |",estado);
        body();
        println! ("|                Cadena valida                       |");
        println! ("+----------------------------------------------------+");
        process::exit(0x0100);
    }
    //Comparamos si la entrada es una "a"
    if a.is_match(caracterN){
        return 1;       
    }
    else {
	    //Comparamos si la entrada es una "b"
	    if b.is_match(caracterN) {
	        return 2;       
	    }
	    else  {
		//En caso de no ser ninguno de los dos anteriores se compara si es el final de la cadena
	    	if caracterN==fin  {
		    	return 3;       //Fin de cadena
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
    //Se crea un vector doble para utilizarlo como tabla de transiciones E y A
    let tabla: Vec<Vec<char>>;
    tabla = vec![
                vec! ['1','E','E','E'],     // q0
                vec! ['2','E','E','E'],     // q1
                vec! ['E','3','E','E'],     // q2
                vec! ['4','E','E','E'],     // q3
                vec! ['5','E','E','E'],     // q4
                vec! ['E','E','6','E'],     // q5
                vec! ['7','E','E','E'],     // q6
                vec! ['8','E','E','E'],     // q7
                vec! ['E','9','E','E'],     // q8
                vec! ['a','E','E','E'],     // q9
                vec! ['b','E','E','E'],     // q10
                vec! ['E','E','E','A']      // q11
            ];
    
    //Variables para imprimir los estados, simbolos y cadena 
    let mut estado: i32 = 0;
    let mut simbolo: String = "".to_string();
    println! ("+-------------------------------------+");
    println! ("|    Ingrese una cadena a evaluar:    |");
    println! ("+-------------------------------------+");
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);
    body();
    encabezado();

    //Estructura de repeticion para avanzar de caracter en caracter
    for  character in cadena.chars(){
        //Mandamos a llamar la funcion caracter para saber si es un caracter valido o invalido
        //el valor que retorna es en caso de ser caracter, operador, final o si no es valido
        let mut charCaracter = caracter(character,estado);
        while charCaracter == 0{
            let  estadoSig: i32 = estado;
            //Se almacena el estado en la tabla de acuerdo a las coordenadas que se obtuvieron
            estado =  (tabla[ estado as usize][charCaracter as usize]) as i32 - '0' as i32 ;
            //Se valida que no se tenga lambda o vacio antes y/o despues de b
            if estadoSig == 1  && character == 'b'  {
                estado = 4;                         
            }
            if estadoSig == 7  && character == '\r' {
                estado = 10;                         
            }

            //Se obtiene la expresion L(a*)={lambda,a,aa,aaa,...}
            if estadoSig == 4  && character != 'b' {
                estado = 1;                         
            }
            if estadoSig == 10  && character != '\r' {
                estado = 7;                         
            }
            // El siguiente estado es q10, el cual contiene el caracter "a"
            if estado == 49 {   
                estado=10;      
            }
            // El siguiente estado es q11, el cual contiene el caracter "b"
            if estado == 50 {
                estado=11;      
            }
            //Muestra en la tabla lo correspondiente a vacío
            contenido(estadoSig,' ',&"        ".to_string(),estado);
            //Vuelve a calcular charCaracter
            charCaracter = caracter(character,estado);
        }
        //Se obtiene denuevo el valor del estado según la tabla de transicion
        let  estadoSig: i32 = estado;
        estado =  (tabla[estado as usize][charCaracter as usize]) as i32 - '0' as i32 ;
        //Si en la funcion caracter regresa el valor de "1" significa que la entrada es una "a"
        //Si en la funcion caracter regresa el valor de "2" significa que la entrada es una "b"
	if charCaracter == 1 {
            simbolo = "   a    ".to_string();
        }
        else if charCaracter == 2{
            simbolo = "   b    ".to_string();
        }
        //Si el valor obtenido es una E imprimimos cadena no valida
        if estado==21{
            println! ("|      {}       |    {}    |  {} |     Error     |",estadoSig,character,simbolo);
            body();
            println! ("|              Cadena no valida                   |");
            println! ("+----------------------------------------------------+");
            process::exit(0x0100);
        }
	//Se imprime el contenido de la tabla
        contenido(estadoSig,character,&simbolo,estado); 
    }  
}