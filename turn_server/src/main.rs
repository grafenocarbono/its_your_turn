/*=================================================================
/ __|/ _ \ '__\ \ / / _ \ '__|
\__ \  __/ |   \ V /  __/ |   
|___/\___|_|    \_/ \___|_|  
==================================================================*/

use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;
/* https://rebellionhost.com/client-server-application-tutorial */
/*
crea una estructura para guardar el estado
del stream
Operaciones E/S
Se tiene que crear un TcpStream para habilitar comunicación
cliente-servidor
*/
fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    /*Maneja accesos múltiples dentro de un stream */
    
    let mut buf = [0;512];
    
    for _ in 0..1000{
        // El servidor recoge un mensaje del cliente
        //prepara un buffer de enteros sin signo para 
        //almacenar los mensajes recibidos por cliente
        //Almacena los bytes recibidos por stream
        //en el buffer "buf"
        let bytes_read: usize = stream.read(&mut buf)?;
        
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        //Imprimir el mensaje recibido. 
        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        // Se paraliza este hilo durante un segundo
        thread::sleep(time::Duration::from_secs(1));   
    }
    // success value
    Ok(())
}

fn main() -> io::Result<()>{
    // Habilitar el puerto 7878 para crear vínculos con clientes
    // (binding)
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
    // Se obtiene un manejador del hilo
    //Crea un vector de hilos
    //growable array type, written as Vec<T>, short for ‘vector’
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // recepciona mensajes entrantes y les hace un binding
    for stream in receiver_listener.incoming() {
        // acepta conexiones y las procesas en orden de petición
        //for stream in listener.incoming() {
       
       //El siguiente método: receiver_listener.incoming()
       //devuelve en cada iteración de un bucle típico "for",
       //un objeto de tipo TcpStream
    
        //chequea si el stream de tipo TcpStream ha tenido
        //algún tipo de incidencia
        let stream = stream.expect("failed");

        // SPAWN significa engendrar. Crea un hilo
        //move lo que hace es copiar por valor aunque se
        //envíen valores mutables o por referencia
        let handle = thread::spawn(move || {
            //fallo de lectura del servidor de los datos envíados 
            //por cliente
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        
        // Hace un push con el handle que estamos tratando ahora mismo
        //dentro de la estructura vectorial que se indicó que su tipo
        //genérico era el siguiente thread::JoinHandle<()>
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // devuelve cada valor a la salida
        handle.join().unwrap();
    }
    // success value
    Ok(())
}
