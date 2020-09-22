/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       22/09/2020
 *  Ultima actualización:  22/09/2020
 *----------------------------------------------------------------------*/
fn main(){
  //Se crea un vector con los dos diferentes tipos de 
  //valores booleanos que se van a utilizar, true or false
  let booleanos = vec![false,true];
    
  //Tabla de verdad de OR
  println!("x\t\ty\t\tx OR y");
  println!("-----------------------");
  /*
     Ciclos for para iterar en el vector creado y realizar la operación OR
     con los valores que tenga la variable "x" y "y" e imprimirlo en forma de
     tabla
  */  
  for x in booleanos.iter(){
    for y in booleanos.iter(){
        println!("{}\t{}\t{}",x,y,(x|y));
    }
  }
  println!();

  //Tabla de verdad de AND
  println!("x\t\ty\t\tx AND y");
  println!("----------------------");
  /*
     Ciclos for para iterar en el vector creado y realizar la operación OR
     con los valores que tenga la variable "x" y "y" e imprimirlo en forma de
     tabla
  */   
  for x in booleanos.iter(){
    for y in booleanos.iter(){
      println!("{}\t{}\t{}",x,y,(x & y));
    }
  }       
  println!();
    
    
  //Tabla de verdad de NOT
  println!("x\t\tNOT x");
  println!("-------------");
  for x in booleanos.iter(){
    println!("{}\t{}",x,!x);
  }
    
  println!();
    
  //Tabla de verdad de ^
  println!("x\t\ty\t\tx^y");
  println!("----------------------");
  /*
     Ciclos for para iterar en el vector creado y realizar la operación OR
     con los valores que tenga la variable "x" y "y" e imprimirlo en forma de
     tabla
  */ 
  for x in booleanos.iter(){
    for y in booleanos.iter(){
      println!("{}\t{}\t{}",x,y,(x^y));
    }
  }
}