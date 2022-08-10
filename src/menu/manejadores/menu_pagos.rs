use crate::models::modelo_compras;
use crate::models::modelo_pagos;

use crate::operaciones::compras;
use crate::utileria::lectura_consola;

pub fn manejar_realizar_pago(items_compra: &mut Vec<modelo_compras::Item>) {

    let monto_a_pagar = compras::total_compra(items_compra);

    println!("Monto a Pagar: ${}", monto_a_pagar);
    println!("Selecciona el metodo de pago.");
    println!("1. En Efectivo");
    println!("2. Tarjeta");
    println!("3. Transferencia Bancaria");

    let mut recibido_del_cliente = 0.0;
    let mut tarjeta = String::from("N/A");

    // Obtenemos la opcion que selecciona el usuario
    let opcion_seleccionada = lectura_consola::leer_usize();

    let metodo_de_pago = match opcion_seleccionada {
        1 => modelo_pagos::MetodoDePago::Efectivo,
        2 => modelo_pagos::MetodoDePago::Tarjeta,
        3 => modelo_pagos::MetodoDePago::TransferenciaBancaria,
        _ => modelo_pagos::MetodoDePago::Efectivo // por facilidad
    };

    if opcion_seleccionada == 1 {
        // El metodo de pago es efectivo, necesitamos saber cuanto recibimos del cliente
        println!("Monto Recibido del Cliente:");
        recibido_del_cliente = lectura_consola::leer_f32();
    }

    if opcion_seleccionada == 2 {
        println!("Num. De Tarjeta:");
        // El metodo de pago es con tarjeta, necesitamos saber el numero
        tarjeta = lectura_consola::leer_string();
    }

    let resultado_del_pago: modelo_pagos::ResultadoPago = compras::pagar_compra(metodo_de_pago, monto_a_pagar, recibido_del_cliente, &tarjeta);

    if resultado_del_pago.fue_exitoso {
        println!("El pago fue exitoso");
        println!("Metodo de Pago: {}", resultado_del_pago.metodo_pago);
        println!("Cambio: ${}", resultado_del_pago.cambio);
    } else {
        println!("Hubo un problema procesando el pago, intentalo de nuevo");
    }
}
