use crate::models::modelo_pagos;

pub fn pagar(metodo_de_pago: modelo_pagos::MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> modelo_pagos::ResultadoPago{
    // El parametro metodo_de_pago es la forma de pago elegida por el cliente.
    // El parametro monto_a_pagar es el total a pagar de la compra.
    // recibido_del_cliente es la cantidad de dinero recibida del cliente, si no es efectivo, es igual al monto a pagar
    // tarjeta, es el numero de tarjeta del cliente, si el pago es en efectivo o por transferencia, no es necesario, puede ser cualquiera

    // Ahora, dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas, esto puede hacerse
    // porque estan dentro del mismo alcance de este metodo.
    let resultado = match metodo_de_pago {
        modelo_pagos::MetodoDePago::Efectivo => pago_en_efectivo(monto_a_pagar, recibido_del_cliente),
        modelo_pagos::MetodoDePago::Tarjeta => pago_con_tarjeta(monto_a_pagar, tarjeta),
        modelo_pagos::MetodoDePago::TransferenciaBancaria => pago_por_transferencia_bancaria(monto_a_pagar)
    };

    resultado
}

fn pago_en_efectivo(monto_a_pagar: f32, recibido_del_cliente: f32) -> modelo_pagos::ResultadoPago {
    // Si el pago es en efectivo, se calculara el cambio a devolver al cliente
    let mut cambio = recibido_del_cliente - monto_a_pagar;
    // redondeando a dos decimales
    let y = 10i32.pow(2) as f32;
    cambio = (cambio * y).round() / y;

    modelo_pagos::ResultadoPago {
        metodo_pago: String::from("En Efectivo"),
        fue_exitoso: true,
        cambio
    }
}

fn pago_con_tarjeta(monto_a_pagar: f32, numero_tarjeta: &str) -> modelo_pagos::ResultadoPago {
    // Si el pago es con tarjeta, simularemos el resultado

    // Aca se estaria procesando todo aquello critico a nivel de seguridad.
    println!("Realizando el pago con el servicio de tarjeta credito/debito");
    println!("Monto a pagar: {}", monto_a_pagar);
    println!("Tarjeta: {}", numero_tarjeta);

    //Ahora simulamos el resultado
    modelo_pagos::ResultadoPago {
        metodo_pago: String::from("Tarjeta"),
        fue_exitoso: true,
        cambio: 0.0
    }
}

fn pago_por_transferencia_bancaria(monto_a_pagar: f32) -> modelo_pagos::ResultadoPago {
    // Si el pago es via transferencia, simularemos que solamente necesitamos la cuenta del supermercado
    // la cual seria la cuenta a recibir el dinero y tambien simularemos el resultado de la transferencia
    // Esta cuenta supondriamos que es secreta

    // Aca se estaria procesando todo aquello critico a nivel de seguridad.
    println!("Realizando las conexiones y transferencias con el banco");
    println!("Monto a pagar: {}", monto_a_pagar);
    println!("Cuenta Bancaria Secreta (o no tanto jejeej): 123456789-0");

    //Ahora simulamos el resultado
    modelo_pagos::ResultadoPago {
        metodo_pago: String::from("Transferencia Bancaria"),
        fue_exitoso: true,
        cambio: 0.0
    }
}
