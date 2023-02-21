
use std::{fs::*};
use std::os::windows::fs::FileExt;
//use rand::Rng;
fn main() {
    let archivo_bytes= read("AAA.jfif");
    match archivo_bytes {
        Ok(a)=>{
            let mut nuevo_archivo:Vec<u16>= Vec::new();
            let mut indice=0;
            let mut  digitos_pi:[u8; 10]= [0; 10];
            let  archivo_encriptado= match File::create("archivo3.encriptado") {
                Err(pq)=> panic!("Error al escribir el archivo debido a {}", pq ),
                Ok(archivo)=> archivo
            };
            
            for i in 0..10 {
                digitos_pi[i]=get_pi_digits();
            }
            for i in 0..a.len() {
                let mut transformar:u16= a[i] as u16;
                match digitos_pi[indice]  {
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
                        print!("{transformar}");
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
                indice+=1;
                if indice==10 {
                    indice=0;
                }
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
fn get_pi_digits() -> u8{
    
    return 8;
}