use anyhow::{bail, Result};
use molar::core::{
    providers::AtomsProvider, MutableSerial, Sel, SelectionKind, State, StateUArc, Topology,
    TopologyUArc,
};
use serde::Deserialize;

pub struct Membrane {
    pub top: TopologyUArc,
    pub state: StateUArc,
}

#[derive(Deserialize, Debug)]
pub struct LipidSpecies {
    name: String,
    whole_sel_str: String,
    head_subsel_str: String,
    tail_subsel_str: String,
    surf_subsel_str: String,
    tails_descr: Vec<String>,
}

pub struct LipidTail {
    // Order of the bond between carbons i and i+1
    bond_orders: Vec<u8>,
    // Offset of carbon i from the beginnig of the whole lipid selection
    c_offsets: Vec<usize>,
}

impl LipidTail {
    pub fn from_tail_descr(descr: &str, sel: &Sel) -> anyhow::Result<Self> {
        let first = sel.nth(0)?.0;

        let c_offsets = descr.split(&['-','='])
        .map(|a| {
            let atsel = sel.subsel_from_str(&format!("name {a}"))?;
            if atsel.len() > 1 {
                bail!("Tail atom {a} is not unique in lipid!");
            }
            Ok(atsel.nth(0)?.0 - first)
        })
        .collect::<Result<Vec<usize>>>()?;
        
        let bond_orders: Vec<u8> = descr.chars()
        .filter_map(|c| {
            match c {
                '-' => Some(1),
                '=' => Some(2),
                _ => None
            }
        }).collect();
        
        if c_offsets.len()-1 != bond_orders.len() {
            bail!("There are Na={} atom names and Nb={} bonds in tail description. Expected Na-1==Nb",c_offsets.len(),bond_orders.len());
        }
        
        Ok(Self{c_offsets, bond_orders})
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub lipid_species: Vec<LipidSpecies>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_species() {
        let s = std::fs::read_to_string("test/inp.toml").unwrap();
        let sp: Config = toml::from_str(&s).unwrap();
        println!("{:?}", sp);
        //let value = s.parse::<toml::Table>().unwrap();
        //println!("{:?}",value);
    }
}
