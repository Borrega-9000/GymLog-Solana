use anchor_lang::prelude::*;

// ID del programa
declare_id!("DX8eW22izxkxLEGsSgwyuHioPXLj5otDUZWVgQh2FYKu");

#[program]
pub mod gym_log_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el log de gimnasio
    pub fn inicializar_log(ctx: Context<CrearLog>, nombre_usuario: String) -> Result<()> {
        let log = &mut ctx.accounts.log;
        log.owner = ctx.accounts.owner.key();
        log.nombre_usuario = nombre_usuario;
        log.rutina = Vec::new();
        
        msg!("Log de entrenamiento creado para: {}", log.nombre_usuario);
        Ok(())
    }

    // 2. CREATE (Dato): Registra un ejercicio con todas sus métricas obligatorias
    pub fn registrar_ejercicio(
        ctx: Context<GestionarLog>, 
        nombre: String, 
        series: u8, 
        repeticiones: u8, 
        peso: u16
    ) -> Result<()> {
        let log = &mut ctx.accounts.log;
        require!(log.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nuevo_ejercicio = Ejercicio {
            nombre,
            series,
            repeticiones,
            peso_kg: peso,
        };

        log.rutina.push(nuevo_ejercicio);
        msg!("Ejercicio registrado exitosamente.");
        Ok(())
    }

    // 3. UPDATE: Modifica todas las métricas de un ejercicio existente
    pub fn editar_ejercicio(
        ctx: Context<GestionarLog>, 
        nombre: String, 
        nuevas_series: u8, 
        nuevas_reps: u8, 
        nuevo_peso: u16
    ) -> Result<()> {
        let log = &mut ctx.accounts.log;
        require!(log.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut log.rutina;
        for i in 0..lista.len() {
            if lista[i].nombre == nombre {
                lista[i].series = nuevas_series;
                lista[i].repeticiones = nuevas_reps;
                lista[i].peso_kg = nuevo_peso;
                msg!("Ejercicio '{}' actualizado.", nombre);
                return Ok(());
            }
        }
        Err(Errores::EjercicioNoEncontrado.into())
    }

    // 4. DELETE: Elimina un ejercicio de la lista
    pub fn eliminar_ejercicio(ctx: Context<GestionarLog>, nombre: String) -> Result<()> {
        let log = &mut ctx.accounts.log;
        require!(log.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut log.rutina;
        let index = lista.iter().position(|e| e.nombre == nombre);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Ejercicio '{}' eliminado de la rutina.", nombre);
            Ok(())
        } else {
            Err(Errores::EjercicioNoEncontrado.into())
        }
    }

    // 5. READ: Visualiza la rutina completa
    pub fn ver_rutina(ctx: Context<GestionarLog>) -> Result<()> {
        msg!("Usuario: {}", ctx.accounts.log.nombre_usuario);
        msg!("Rutina Actual: {:#?}", ctx.accounts.log.rutina);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Ejercicio {
    #[max_len(30)]
    pub nombre: String,
    pub series: u8,
    pub repeticiones: u8,
    pub peso_kg: u16,
}

#[account]
#[derive(InitSpace)]
pub struct GymLog {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(12)] // Capacidad para 12 ejercicios por rutina
    pub rutina: Vec<Ejercicio>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearLog<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + GymLog::INIT_SPACE,
        seeds = [b"gymlog", owner.key().as_ref()],
        bump
    )]
    pub log: Account<'info, GymLog>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarLog<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub log: Account<'info, GymLog>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos sobre este log.")]
    NoEresElOwner,
    #[msg("El ejercicio no existe en la rutina.")]
    EjercicioNoEncontrado,
}
