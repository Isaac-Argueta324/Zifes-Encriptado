
use std::{fs::*};
use std::os::windows::fs::FileExt;
use sha256::*;
use std::io::*;
use std::time::Instant;
fn main() {
    //Lectura del archivo
    let mut archivo_nombre=String::new();
    println!("Hola usuario introduce el archivo que deseas codificar");
    stdin().read_line(&mut archivo_nombre ).expect("Error al leer el nombre del archivo a encriptar");
    archivo_nombre.pop();
    archivo_nombre.pop();
    let b= archivo_nombre.as_str();
    let archivo_bytes= read(b);
    match archivo_bytes {
        Ok(a)=>{
            
            let mut contrasena_usuario= String::new(); 
            let mut indice= 0;
            let mut  numero_aleatorio:u8;
            let mut nuevo_archivo_nombre=String::from("Crzipfes_");
            nuevo_archivo_nombre.push_str(b);

            let  archivo_encriptado= match File::create(nuevo_archivo_nombre) {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(archivo)=> archivo
            };

            println!("Introduce la contraseña de tu archivo");
           stdin().read_line(&mut contrasena_usuario).expect("Error durante la lectura de la contraseña");
           contrasena_usuario.pop();
           contrasena_usuario.pop();
            let tiemp= Instant::now();
            //Ciclo de encriptado, se itera en función del tamaño del archivo en bytes
            
            for i in 0..a.len() {
                //Seleccion de un numero aleatorio mediante la función numeros_aleaatorios que toma como parametro la contraseña del usuario
                numero_aleatorio=numeros_aleaatorios(&mut contrasena_usuario);
                let mut transformar:u16= a[i] as u16;

                //Seleccionar y ejecutar una transformación en función del numero aleatorio

                match numero_aleatorio  {
                    1=>{
                        transformar+=27;
                    }
                    2=>{
                        transformar*=3;
                    }
                    3=>{
                        transformar+=41;
                    }
                    4=>{
                        transformar*=6;
                    }
                    5=>{
                        transformar+=1;
                        transformar*=2;
                    }
                    6=>{
                        transformar+=7;
                        transformar*=3;
                    }
                    7=>{
                        transformar+=4;
                        transformar*=2;
                    }
                    8=>{
                        transformar*=3;
                        transformar+=7;
                    }
                    9=>{
                        transformar*=2;
                        transformar+=5;
                    }
                    0=>{
                        transformar*=3;
                        transformar+=9;
                    }
                    _default=>{

                    }
                }
                
                //Escritura del nuevo dato en el archivo

                let escribir=transformar.to_be_bytes();
                match archivo_encriptado.seek_write(&escribir,indice){
                    Ok(_a)=>{

                    }
                    Err(pq)=>{
                        print!("Error durante la escritura de archivos debdio a{}", pq);
                    }
                }
                indice+=2;
            }
            println!("Codificación exitosa tiempo de codificación: {} segundos", tiemp.elapsed().as_nanos());
        }
        Err(_a)=>{
            print!("Error al cargar el archivo");
        }
    }
}


/*
    La función numeros_aleaatorios toma como parametro un apuntador a un String, este parametro es llamado contrasena
    se almacena en una variable llamada nueva_contrasena el hash de la cadena a la que apunta el parametro contrasena
    Se limpia la cadena a la que apunta el parametro contrasena y se le asigna el valor del hash
    Se selecciona el ultimo caracter del hash, se convierte a un entero de, se aplica modulo 10 y regresa este ultimo valor
 */

fn numeros_aleaatorios(contraesena: &mut String) -> u8{
    
    let nueva_contrasena=digest(contraesena.as_str());
    contraesena.clear();
    contraesena.push_str(&nueva_contrasena);
    let indice= contraesena.len()-1;
    let valor_pseudoaleatorio= nueva_contrasena.as_bytes();
    let mut retorno= valor_pseudoaleatorio[indice];
    retorno=retorno%10;
    return retorno;
}

//