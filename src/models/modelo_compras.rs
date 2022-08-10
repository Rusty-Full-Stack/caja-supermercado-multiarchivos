#[derive(Debug)]
pub struct Item {
    pub nombre: String, // Nombre del item
    pub precio_unitario: f32, // Precio Unitario del item
    pub cantidad: f32, // Cantidad a comprar del item, es float porque pueden ser fracciones de unidades, como kilos
}
