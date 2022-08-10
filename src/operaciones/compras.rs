use crate::models::modelo_compras::Item;
use crate::models::modelo_pagos;

use crate::operaciones::pagos;

pub fn agregar_item(items_compra: &mut Vec<Item>, item: Item) {
    // Agrega un item a un vector con todos los items de la compra
    items_compra.push(item);
}

pub fn quitar_item(items_compra: &mut Vec<Item>, indice: usize) {
    // Quitara un item del array a partir de un indice
    items_compra.remove(indice);
}

pub fn mostrar_items(items_compra: &Vec<Item>) {
    // Mostrando los items y el indice
    for (index, item) in items_compra.iter().enumerate() {
        let subtotal = item.cantidad * item.precio_unitario;
        println!("[{}]. {} - Cantidad: {} - Precio U: ${} - Subtotal: ${}", index, item.nombre, item.cantidad, item.precio_unitario, subtotal);
    }
}

pub fn total_compra(items_compra: &Vec<Item>) -> f32 {
    // Devolvera el totla a pagar de todos los items de la compra
    let mut total_compra: f32 = 0.0;
    for item in items_compra {
        total_compra = total_compra + (item.cantidad * item.precio_unitario);
    }

    // redondeando a dos decimales
    let y = 10i32.pow(2) as f32;
    total_compra = (total_compra * y).round() / y;

    total_compra
}

pub fn pagar_compra(metodo_de_pago: modelo_pagos::MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> modelo_pagos::ResultadoPago{
    // El parametro metodo_de_pago es la forma de pago elegida por el cliente.
    // El parametro monto_a_pagar es el total a pagar de la compra.
    // recibido_del_cliente es la cantidad de dinero recibida del cliente, si no es efectivo, es igual al monto a pagar
    // tarjeta, es el numero de tarjeta del cliente, si el pago es en efectivo o por transferencia, no es necesario, puede ser cualquiera

    // Ahora, dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas, esto puede hacerse
    // porque estan dentro del mismo alcance de este metodo.

    let resultado = pagos::pagar(metodo_de_pago, monto_a_pagar, recibido_del_cliente, tarjeta);

    resultado
}
