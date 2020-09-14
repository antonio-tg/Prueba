#![allow(non_snake_case)]
use std::io; //Este paqete es para permitir la entrada de información
fn main() {
    figura();
}

fn figura(){
    println!("Teclee el nombre de una de las siguientes figura geometricas");
    println!("Circulo, Triangulo, Cuadrado, Rectangulo, Poligono");
    println!("En el caso del poligono tendra que ser regular");
    let mut geo = String::new(); //leo el tipo de figura geometrica de la que se quiere obtener el perimetro y el area
    io::stdin().read_line(&mut geo).expect("failed to read line");
    
    if geo.trim().to_uppercase() == "CIRCULO" { //Este if seleccciona el tipo de figura y hace las preguntas correspondientes
        println!("Teclee el radio del circulo");
        let mut r = String::new();
    	io::stdin().read_line(&mut r).expect("failed to read line");
    	let r2 = r.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("El perimetro es {}",  2.0*3.14159*r2);
        println!("El area es {}",  3.14159*r2*r2);
    } else if  geo.trim().to_uppercase() == "TRIANGULO" {
        println!("Teclee la longitud del lado");
        let mut l = String::new();
    	io::stdin().read_line(&mut l).expect("failed to read line");
    	let l2 = l.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("El perimetro es {}", 3.0*l2);
        println!("El area es {}", 0.4330127*l2*l2);
    } else if geo.trim().to_uppercase() == "CUADRADO" {
        println!("Teclee la longitud del lado");
        let mut l = String::new();
    	io::stdin().read_line(&mut l).expect("failed to read line");
    	let l2 = l.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("El perimetro es {}", 4.0*l2);
        println!("El area es {}", l2*l2);
    } else if geo.trim().to_uppercase() == "RECTANGULO" {
        println!("Teclee la longitud de la base");
        let mut b = String::new();
    	io::stdin().read_line(&mut b).expect("failed to read line");
    	let b2 = b.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("Teclee la longitud de la altura");
        let mut a = String::new();
    	io::stdin().read_line(&mut a).expect("failed to read line");
    	let a2 = a.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("El perimetro es {}", 2.0*b2 +2.0*a2);
        println!("El area es {}", b2*a2);
    } else if geo.trim().to_uppercase() == "POLIGONO" {
        println!("Teclee el número de lados");
        let mut n = String::new();
    	io::stdin().read_line(&mut n).expect("failed to read line");
    	let n2 = n.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("Teclee la longitud de un lado");
        let mut l = String::new();
    	io::stdin().read_line(&mut l).expect("failed to read line");
    	let l2 = l.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("Teclee la longitud del apotema");
        let mut a = String::new();
    	io::stdin().read_line(&mut a).expect("failed to read line");
    	let a2 = a.parse::<f32>().unwrap(); //Aqui convierto una cadena en un flotante
        println!("El perimetro es {}", n2*l2);
        println!("El area es {}", n2*l2*a2*0.5);
    } else{
    	println!("La figura: {} no es una figura valida", geo.trim());
    }
}
