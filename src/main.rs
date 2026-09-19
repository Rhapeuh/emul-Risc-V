use std::{fs::File, io::{self, Read}};
use clap::Parser;

/// Un décodeur d'instruction RISC-V RV32I
#[derive(Parser, Debug)]
#[command(
    name = "decode_riscv", 
    override_usage = "decode_riscv [OPTIONS] FICHIER_BIN",
)]

struct Args {
    /// Un fichier au format binaire contenant les instructions à décoder
    #[arg(name = "FICHIER_BIN")]
    fichier_bin: String,
}
fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut fichier = File::open(&args.fichier_bin).unwrap_or_else(|err| {
        eprintln!("Erreur lors de l'ouverture du fichier {} : {}", args.fichier_bin, err);
        std::process::exit(1);
    });

    let mut buffer = [0u8; 4];
    let mut offset = 0;

    println!("offset,valeur,opcode,encoding");

    while fichier.read_exact(&mut buffer).is_ok() {
        let instruction = u32::from_le_bytes(buffer);

        let offset_hexa = format!("{:08x}", offset);
        let instruction_hexa = format!("{:08x}", instruction);

        let opcode = instruction & 0x7F;

        let (opcode_string, encodage) = match opcode {
            0b1100011 => ("BRANCHE", "S_B"),
            0b1100111 => ("JALR", "I"),
            0b0000011 => ("LOAD", "I"),
            0b0001111 => ("MISC-MEM", "I"),
            0b0010011 => ("OP-IMM", "I"),
            0b1110011 => ("SYSTEM", "I"), 
            0b1101111 => ("JAL", "U_J"),
            0b0110011 => ("OP", "R"),
            0b0100011 => ("STORE", "S"),
            0b0010111 => ("AUIPC", "U"),
            0b0110111 => ("LUI", "U"),
            _ => ("INCONNU", "INCONNU"),
        };

        println!("{},{},{},{}", offset_hexa, instruction_hexa, opcode_string, encodage);

        offset += 4;
    }

    Ok(())
}
