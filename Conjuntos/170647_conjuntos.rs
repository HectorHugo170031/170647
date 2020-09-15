/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    David Ulises Rodríguez Abella
 *  Matricula: 170647
 *  
 *  Escrito:       11/09/2020
 *  Ultima actualización:  13/09/2020
 *----------------------------------------------------------------------*/
use std::collections::HashSet;

fn main(){
    //Se declaran 3 conjuntos
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    let conj_C: HashSet<i32> = HashSet::new();
    //Se imprimen los 3 conjuntos en consola 
    println!("Conjunto A: {:?}", conj_A);
    println!("Conjunto B: {:?}", conj_B);
    println!("Conjunto C: {:?}", conj_C);

    //Se mandan a llamar todas las funciones que contienen las operaciones de conjuntos
    //Pertenencia de conjuntos
    pertenencia();
    //Transformar datos de conjuntos
    transformarConjunto();
    //Eliminar elemento del conjunto
    eliminarElemento();
    //Borrar todo el contenido del conjunto
    limpiarConjunto();
    //Copiar el contenido de un conjunto a otro conjunto
    copiarConjunto();
    //Agregar elemento al conjunto
    agregarElemento();
    //Operacion union de conjuntos
    unionConj();
    //Interseccion de conjuntos
    interseccion();
    //Diferencia de conjuntos
    diferencia();
    //Diferencia simetrica de conjuntos
    diferenciaSimetrica();
    //Subconjuntos
    subConjuntos();
    //Superconjuntos
    superConjuntos();
}

fn pertenencia(){
    //PERTENENCIA
    println!("\nPERTENENCIA");
    let conj_A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    //1 en conjunto A
    println!("El conjunto A contiene #1: {}",conj_A.contains(&1));
    //1 no esta en conjunto A 
    println!("El conjunto A no contiene #1:{}",!conj_A.contains(&1));
    //10 en conjunto A 
    println!("El conjunto A contiene #10: {}",conj_A.contains(&10));
    //10 no esta en conjunto A 
    println!("El conjunto A no contiene #10: {}",!conj_A.contains(&10));
}

fn transformarConjunto(){
    //TRANSFORMAR A CONJUNTO
    println!("\nCONVERTIR UN ELEMENTO A UN CONJUNTO");
    // Se declaran 3 variables las cuales son
    // un conj_a, conj_b y conj_c son de los tipos
    // vector, arreglo y una cadena respectivamente 
    let conj_a = vec![1, 2, 3, 4, 5]; 
    let conj_b = [1, 2, 3, 4, 5];    
    let conj_c = "Hola Mundo";        

    //Se convierten a conjuntos las variables conj_a y conj_b 
    //utilizando los metodos .iter y .collect 
    let conj_A: HashSet<_> = conj_a.iter().collect(); 
    let conj_B: HashSet<_> = conj_b.iter().collect(); 
    //Para un string se debe de separar cada caracter utilizando 
    //un for each para iterar en cada caracter que tiene la cadena
    //y se agregan al conjunto creado
    let mut conj_C: HashSet<char> = HashSet::new();
    for caracter in conj_c.chars(){
        conj_C.insert(caracter);
    }
    //Se imprimen los conjuntos que se convirtieron
    println!("Conjunto A: {:?}", &conj_A); 
    println!("Conjunto B: {:?}", &conj_B); 
    println!("Conjunto C: {:?}", &conj_C); 

}

fn eliminarElemento(){
    //Para eliminar un elemento unicamente se utiliza el metodo
    //remove(&n) donde n es el elemento a eliminar en este caso es 2
    let mut conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    //ELIMINAR UN DATO DEL CONJUNTO
    println!("\nELIMINAR UN DATO DEL CONJUNTO");
    conj_A.remove(&2);
    println!("Despues de eliminar un elemento del conjunto A: {:?}", conj_A);
}

fn limpiarConjunto(){
    //Para limpiar un elemento unicamente se utiliza el metodo
    //clear() y este limpia todo el conjunto
    let mut conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    //LIMPIAR TODO EL CONJUNTO
    println!("\nLIMPIAR TODO EL CONJUNTO");
    conj_A.clear();
    println!("Despues de limpiar del conjunto A: {:?}", conj_A);
}

fn copiarConjunto(){
    //Para copiar todo el conjunto se clona a otro conjunto con el
    //metodo clone()
    //COPIAR CONJUNTO
    println!("\nCOPIAR CONJUNTO");
    let conj_A: HashSet<_> = [1,2,3,4,5].iter().collect();
    let conj_B: HashSet<_> = conj_A.clone();
    println!("Conjunto B: {:?}", &conj_B); 
}

fn agregarElemento(){
    //Para agregar un elemento se utiliza el metodo insert(&n)
    //donde n es el elemento que se va a agregar
    let mut conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //AGREGAR NUEVO ELEMENTO AL CONJUNTO
    println!("\nAGREGAR NUEVO ELEMENTO AL CONJUNTO");
    conj_B.insert(&987);
    println!("Despues de agregar elemento al conjunto B: {:?}", conj_B);
}

fn unionConj(){
    //Para obtener la union de los conjuntos se utiliza el metodo
    //union(&set) donde set es el conjunto con el que se quiere obtener la union 
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //OPERACIÓN UNION DE CONJUNTOS
    println!("\nOPERACIÓN UNION DE CONJUNTOS");
    println!("Union del conjunto A y conjunto B: {:?}", conj_A.union(&conj_B));
}

fn interseccion(){
    //Para obtener la interseccion de los conjuntos se utiliza el metodo
    //intersection(&set) donde set es el conjunto con el que se quiere obtener 
    //la interseccion
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //OPERACIÓN INTERSECCIÓN DE CONJUNTOS
    println!("\nOPERACIÓN INTERSECCIÓN DE CONJUNTOS");
    println!("Intersección del conjunto A y conjunto B: {:?}", conj_A.intersection(&conj_B));
}

fn diferencia(){
    //Para obtener la diferencia de los conjuntos se utiliza el metodo
    //difference(&set) donde set es el conjunto con el que se quiere obtener 
    //diferencia
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //OPERACIÓN DIFERENCIA DE CONJUNTOS
    println!("\nOPERACIÓN DIFERENCIA DE CONJUNTOS");
    println!("Diferencia del conjunto A y conjunto B: {:?}", conj_A.difference(&conj_B));
}

fn diferenciaSimetrica(){
    //Para obtener la diferencia simetrica de los conjuntos se utiliza el metodo
    //symmetric_difference(&set) donde set es el conjunto con el que se quiere  
    //obtener diferencia simetrica
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    let conj_C: HashSet<_> = [].iter().collect();
    //OPERACIÓN DIFERENCIA SIMETRICA DE CONJUNTOS
    println!("\nOPERACIÓN DIFERENCIA SIMETRICA DE CONJUNTOS");
    println!("Diferencia simetrica del conjunto A y conjunto B: {:?}", conj_A.symmetric_difference(&conj_B));
    println!("Diferencia simetrica del conjunto B y conjunto A: {:?}", conj_B.symmetric_difference(&conj_A));
    println!("Diferencia simetrica del conjunto A y conjunto C: {:?}", conj_A.symmetric_difference(&conj_C));
    println!("Diferencia simetrica del conjunto A y conjunto B: {:?}", conj_B.symmetric_difference(&conj_C));
}

fn subConjuntos(){
    //Para obtener si es un subconjunto se utiliza el metodo 
    //is_subset(&set) donde set es el segundo conjunto
    //regresa true si es un subconjunto y false sino lo es
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //SUBCONJUNTOS
    println!("\nSUBCONJUNTOS");
    println!("A es un subconjunto del conjunto B: {:?}", conj_A.is_subset(&conj_B));
    println!("B es un subconjunto del conjunto A: {:?}", conj_B.is_subset(&conj_A));
}

fn superConjuntos(){
    //Para obtener si es un superconjunto se utiliza el metodo 
    //is_superset(&set) donde set es el segundo conjunto
    //regresa true si es un superconjunto y false sino lo es
    let conj_A: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let conj_B: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    //SUPERCONJUNTOS
    println!("\nSUPERCONJUNTOS");
    println!("Conjunto A es un superconjunto del conjunto B: {:?}", conj_A.is_superset(&conj_B));
    println!("Conjunto B es un superconjunto del conjunto A: {:?}", conj_B.is_superset(&conj_A));
} 