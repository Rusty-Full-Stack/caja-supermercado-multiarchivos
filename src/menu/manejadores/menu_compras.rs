use crate::models::modelo_compras::Item;
use crate::operaciones::compras;
use crate::utileria::lectura_consola;

pub fn manejar_agregar_item(items_compra: &mut Vec<Item>) {
    // Solicitando que el usuario registre los datos del item
    // por facilidad no hemos colocado validaciones, en un ambiente
    // laboral SI debes colocarlas.
    // Quiza no es el input mas optimo del planeta pero sirve a manera
    // de ejemplo de uso de modulos
    println!("Escribe los detalles del Item");

    println!("NOMBRE:");
    let nombre = lectura_consola::leer_string();

    println!("CANTIDAD:");
    let cantidad = lectura_consola::leer_f32();

    println!("PRECIO UNITARIO: ");
    let precio_unitario = lectura_consola::leer_f32();

    // Creando el item con la estructura que importamos de nuestro modulo
    let item: Item = Item {
        nombre,
        precio_unitario,
        cantidad
    };

    // Agregando el item a la compra
    compras::agregar_item(items_compra, item);

    println!("Item Agregado!");
}

pub fn manejar_quitar_item(items_compra: &mut Vec<Item>) {

    // mostrando los items par que el usuario pueda saber cual quitar
    // REUTILIZANDO NUESTRO MODULO !!!
    println!("Selecciona el indice que quieres quitar");
    compras::mostrar_items(items_compra);

    // Obteniendo el itema a eliminar
    let indice = lectura_consola::leer_usize();

    // Eliminando el item utilizando la funcion dentro del modulo de compra
    compras::quitar_item(items_compra, indice);
    println!("Item eliminado");
}
