use molar::{core::Source, io::FileHandler};

fn main() -> anyhow::Result<()> {
    let (top,state) = FileHandler::open("test/bucle.gro")?.read()?;
    let membr = membralysis::membrane::Membrane{top, state};
    Ok(())  
}
