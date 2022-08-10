use crate::models::modelo_compras;
use crate::menu::manejadores::{menu_principal, menu_pagos};

// Una forma interesante de hacer el import:
use super::manejadores::menu_compras; // tambien vale crate::menu::manejadores::menu_compras;
use crate::operaciones::compras;

use crate::utileria::lectura_consola;

pub fn menu_principal(items_compra: &mut Vec<modelo_compras::Item>) {
    // Iniciamos un loop en el cual vamos a preguntar al usuario la accion a realizar
    // Dependiendo de sus seeleccion vamos a realizar una tarea, todas ellas dependeran
    // de funciones dentro del modulo "compra"
    loop {
        menu_principal::mostrar_menu();

        // Obtenemos la opcion que selecciona el usuario
        // limpiando el input de la terminal
        let opcion_seleccionada = lectura_consola::leer_usize();

        match opcion_seleccionada {
            1 => menu_compras::manejar_agregar_item(items_compra), // Agregar un item
            2 => menu_compras::manejar_quitar_item(items_compra), // quitar un item
            3 => compras::mostrar_items(&items_compra), // mostrar todos los items y sus indices
            4 => println!("Total a pagar: ${}", compras::total_compra(&items_compra)), // mostrando el total a pagar
            5 => menu_pagos::manejar_realizar_pago(items_compra), // realizar el pago
            6 => break, // terminando el programa
            _ => println!("Opcion Invalida") // la opcion no es valida, el programa continua
        };
    }
}
