
use std::{fs::*};
use std::os::windows::fs::FileExt;
use sha256::*;
use std::time::*;
use std::io::*;
fn main() {
    //Lectura del archivo
    let mut archivo_nombre=String::new();
    println!("Hola usuario introduce el archivo que deseas codificar");
    stdin().read_line(&mut archivo_nombre ).expect("Error durante la lectura de la contraseña");
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

            //Ciclo de encriptado
            let tiempo= Instant::now();
            for i in 0..a.len() {
                //Seleccion de un numero aleatorio
                numero_aleatorio=numeros_aleaatorios(&mut contrasena_usuario);
                let mut transformar:u16= a[i] as u16;

                //Seleccionar y ejecutar una transformación

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
            println!("{}", tiempo.elapsed().as_secs());
            
        }
        Err(_a)=>{
            print!("Error al cargar el archivo");
        }
    }
}

 //

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