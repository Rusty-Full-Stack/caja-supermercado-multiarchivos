pub enum MetodoDePago {
    Efectivo,
    Tarjeta,
    TransferenciaBancaria,
}

pub struct ResultadoPago {
    pub metodo_pago: String, // La descripcion del metodo de pago
    pub fue_exitoso: bool, // true si el pago se pudo hacer o false si no se pudo hacer
    pub cambio: f32, // Cambio a devolver al cliente.
}
