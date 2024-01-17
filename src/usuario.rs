use crate::registro_horas::RegistroHoras;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usuario {
    pub usuario: String,
    pub registro_horas: Vec<RegistroHoras>,
}