
use std::{fs::*};
use std::os::windows::fs::FileExt;
use sha256::*;
fn main() {
    let archivo_bytes= read("a.txt");
    match archivo_bytes {
        Ok(a)=>{
            let mut contrasena_usuario= crear_contrasena_del_usuario();
            let mut nuevo_archivo:Vec<u16>= Vec::new();
            let mut  numero_aleatorio:u8;
            let  archivo_encriptado= match File::create("archivo3.encriptado") {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(archivo)=> archivo
            };
            println!("hola usuario la contraseña de tu aarchivo es: \n{}\n No lo olvides :D", contrasena_usuario);
            for i in 0..a.len() {
                numero_aleatorio=numeros_aleaatorios(&mut contrasena_usuario);
                let mut transformar:u16= a[i] as u16;
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
                nuevo_archivo.push(transformar);
            }
            let mut indice= 0;
            for i in 0..nuevo_archivo.len()  {
                let  escribir= nuevo_archivo[i].to_be_bytes();
                match archivo_encriptado.seek_write(&escribir,indice){
                    Ok(_a)=>{

                    }
                    Err(pq)=>{
                        print!("Error durante la escritura de archivos debdio a{}", pq);
                    }
                }
                indice+=2;
            }
        }
        Err(_a)=>{
            print!("Error al cargar el archivo");
        }
    }
}


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
fn crear_contrasena_del_usuario() -> String{
    let mut contrasena=String::new();
    for _i in 0..10 {
        let letra_aleatoria= rand::random::<char>();
        contrasena.push(letra_aleatoria);
    }
    return digest(contrasena);
}