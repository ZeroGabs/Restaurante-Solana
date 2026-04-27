use anchor_lang::prelude::*;
// ID del Solana Program, este espacio se llena automaticamente al haver el "build"
declare_id!("Gjf327RenZrfNfuqkMjPFGUHMCvxEWW5K9K5Dc4Zs5W9");

#[program] // Macro que convierte codigo de Rust a Solana. Apartir de aqui empieza tu codigo!
pub mod restaurante {
    use super::*; // Importa todas los structs y enums definidos fuera del modulo

    //////////////////////////// Instruccion: Crear un Restaurante /////////////////////////////////////
    /*
    Parametros de entrada:
        * nombre -> nombre de la biblioteca -> tipo string
     */
    pub fn crear_restaurante(context: Context<NuevoRestaurante>, nombre: String) -> Result<()> {
        // "Context" siempre suele ir como primer parametro, ya que permite acceder al objeto o cuenta con el que queremos interactuar
        // Dentro del context va al tipo de objeto o cuenta con el que deseamos interactuar.
        let owner_id = context.accounts.owner.key(); // Accedemos al wallet address del caller
        msg!("Owner id: {}", owner_id); // Print de verificacion

        let menu: Vec<ItemMenu> = Vec::new(); // Crea el menú vacio

        // Creamos un Struct de tipo restaurante y lo guardamos directamente
        context.accounts.restaurante.set_inner(Restaurante {
            owner: owner_id,
            nombre,
            menu,
        });
        Ok(()) // Representa una transaccion exitosa
    }

    //////////////////////////// Instruccion: Agregar platillos, bebidas, postres (Items) /////////////////////////////////////
    /*
    Agrega un item al menu del restaurante contenido en el struct Restaurante. 
    En este caso el contexto empleado es el struct GestionRestaurante. Mientras que NuevoRestaurante permite crear 
    Instancias de un restaurante. GestionRestaurante permite crear y modificar los valores relacionados a cualquier
    struct de tipo item.

    Parametros de entrada:
        * nombre -> nombre del item -> string
        * precios -> precio del item -> u64
        * tipo -> que es el item?? platillo, bebida o postre -> string
     */
    pub fn agregar_item(context: Context<GestionRestaurante>, nombre: String, precio: u64, tipo: String) -> Result<()> {
        require!(
            context.accounts.restaurante.owner == context.accounts.owner.key(), // Condicion, true -> continua, false -> error
            Errores::NoEresElOwner // Codigo de error, ver enum Errores
        );

        let item = ItemMenu {
            // Creacion de un struct tipo Item
            nombre,
            precio,
            tipo,
            disponible: true,
        };

        context.accounts.restaurante.menu.push(item); // Agrega el item al vector de menus del restaurante

        Ok(()) // Transaccion exitosa
    }

    //////////////////////////// Instruccion: Eliminar Item /////////////////////////////////////
    /*
    Elimina un item apartir de su nombre.

    Parametros de entrada:
        * nombre -> Nombre del item -> string
     */
    pub fn eliminar_item(context: Context<GestionRestaurante>, nombre: String) -> Result<()> {
        require!(
            // Medida de seguridad
            context.accounts.restaurante.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let menu = &mut context.accounts.restaurante.menu; // Referencia mutable al vector de menus

        for i in 0..menu.len() {
            // Se itera mediante el indice todo el contenido del vector en busca del item a eliminar
            if menu[i].nombre == nombre {
                // Si lo encuentra prodece a borrarlo mediante el metodo remove
                menu.remove(i);
                msg!("Item {} eliminado!", nombre); // Mensaje de borrado exitoso
                return Ok(()); // Transaccion exitosa
                
            }
        }
        Err(Errores::ItemNoExiste.into()) // Transaccion fallida, nunca encontro el item
    }

    //////////////////////////// Instruccion: Ver Items /////////////////////////////////////
    /*
    Muestra en el log de la transaccion el contenido completo del vector de items del Restaurante

   
     */
    pub fn ver_menu(context: Context<GestionRestaurante>) -> Result<()> {
        require!(
            // Medida de seguridad
            context.accounts.restaurante.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // :#? requiere que NuevoLibro tenga atributo Debug. Permite la visualizacion completa del vector en el log
        msg!(
            "El menú actual es: {:#?}",
            context.accounts.restaurante.menu
        ); // Print en log
        Ok(()) // Transaccion exitosa
    }

    //////////////////////////// Instruccion: Alternar Disponibilidad /////////////////////////////////////
    /* 
    Cambia el estado de disponible de false a true o de true a false.

    Parametros de entrada:
        * nombre -> Nombre del item -> string
     */
    pub fn alternar_disponibilidad(context: Context<GestionRestaurante>, nombre: String) -> Result<()> {
        require!(
            // Medida de seguridad
            context.accounts.restaurante.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let menu = &mut context.accounts.restaurante.menu; // Referencia mutable al vector de items
        for i in 0..menu.len() {
            // Se itera mediante el indice el vector de items
            if menu[i].nombre == nombre {
                // Si ecuentra el nombre del item procede a cambiar el valor del estado
                let nuevo_estado = !menu[i].disponible;
                menu[i].disponible = nuevo_estado;
                msg!(
                    "El item: {} esta disponibe: {}",
                    nombre,
                    nuevo_estado
                ); // log print de la nueva disponibilidad
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ItemNoExiste.into())
    }

}

/*
Codigos de error
Todos los codigos se almacenan en un enum con la siguiente estructura:
#[msg("MENSAJE DE ERROR")] (dentro de las comillas)
NombreDelError, (En camel case)
*/
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de este restaurante")]
    NoEresElOwner,
    #[msg("Error, el item del menú no existe")]
    ItemNoExiste,
}

#[account] // Especifica que el strcut es una cuenta que se almacenara en la blockchain
#[derive(InitSpace)] // Genera la constante INIT_SPACE y determina el espacio de almacenamiento necesario
pub struct Restaurante {
    // Define el restaurante
    pub owner: Pubkey, // Pubkey es un formato de llave publica de 32 bytes

    #[max_len(60)] // Cantidad maxima de caracteres del string: nombre
    pub nombre: String,

    #[max_len(15)] // Tamaño maximo del vector items
    pub menu: Vec<ItemMenu>,
}

/*
Struct interno o secundario (No es una cuenta). Se define por derive y cuenta con los siguientes atributos:
    * AnchorSerialize -> Permite guardar el struct en la cuenta 
    * AnchorDeserialize -> Permite leer su contenido desde la cuenta 
    * Clone -> Para copiar su contenido o valores 
    * InitSpace -> Calcula el tamaño necesario para ser almacenado en la blockchain
    * PartialEq -> Para usar sus valores y compararlos con "=="
    * Debug -> Para mostrarlo en log con ":?" o ":#?"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct ItemMenu {
    #[max_len(60)]
    pub nombre: String,

    pub precio: u64, //Precio
    #[max_len(20)]

    pub tipo: String, //Platillo, bebidad o postre

    pub disponible: bool,
}

// Creacion de los contextos para las instrucciones (funciones)
#[derive(Accounts)] // Especifica que este struct describe las cuentas que se requieren para determinada instruccion
pub struct NuevoRestaurante<'info> {
    // contexto de la instruccion
    #[account(mut)]
    pub owner: Signer<'info>, // Se define que el owner como el que pagara la transaccion, por eso es mut, para que cambie el balance de la cuenta

    #[account(
        init, // Inidica que al llamar la instruccuion se creara una cuenta
        // puede ser remplazado por "init_if_needed" para que solo se cree una vez por caller
        payer = owner, // Se especifica que quien paga el llamado a la instruccion, en este caso llama la instruccion 
        space = Restaurante::INIT_SPACE + 8, // Se calcula el espacio requerido para almacenar el Solana Program On-Chain
        seeds = [b"restaurante", owner.key().as_ref()], // Se especifica que la cuenta es una PDA que depende de un string y el id del owner
        bump // Metodo para determinar el el id de la biblioteca en base a lo anterior 
    )]
    pub restaurante: Account<'info, Restaurante>, // Se especifica que la cuenta creada (PDA) almacenara la biblioteca

    pub system_program: Program<'info, System>, // Programa necesario para crear la cuenta
}

// Contexto para la creacion y modificacion de libros
#[derive(Accounts)] // Especifica que este struct se requiere para todas las instrucciones relacionadas con la creacion o modificacion de Libro
pub struct GestionRestaurante<'info> {
    pub owner: Signer<'info>, // El owner de la cuenta es quien paga la transaccion

    #[account(mut)]
    pub restaurante: Account<'info, Restaurante>, // Se marca biblioteca como mutable porque se modificara tanto el vector como los libros que contiene
}
