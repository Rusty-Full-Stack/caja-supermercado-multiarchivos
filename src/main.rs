mod models;
mod menu;
mod operaciones;
mod utileria;

use models::modelo_compras;
use menu::creacion_menu;

fn main() {
    // Creamos un vector para llevar el registro de los items de la compra
    let mut items_compra: Vec<modelo_compras::Item> = Vec::new();
    creacion_menu::menu_principal(&mut items_compra);

    println!("Programa Finalizado");
}
