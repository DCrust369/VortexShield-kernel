// direitos autorais (Rust) DCrust 16/04/2026
// Copyright (Rust) DCrust 16/04/2026

use std::fs;
use std::io::Write;

#[tokio::main]
async fn main() {
    // RAM: simulação configurável — escale conforme o hardware alvo
    let ram_size_mb: usize = 512; // troque para o valor desejado
    let ram: Vec<u8> = vec![0u8; ram_size_mb * 1024 * 1024];
    println!("RAM simulada: {} MB alocados", ram_size_mb);

    // SSD: escrita em chunks para não explodir a stack
    let ssd_size_mb: usize = 1024; // 1 GB simulado
    write_ssd_simulation("temp_data.bin", ssd_size_mb * 1024 * 1024)
        .expect("Falha ao escrever no SSD");
    println!("SSD simulado: {} MB escritos", ssd_size_mb);

    // Cloud: chamada assíncrona
    println!("Enviando dados para a nuvem...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Dados enviados com segurança!");

    // Borrow de parte da RAM
    let chunk = &ram[0..100];
    println!("Processando {} bytes da RAM", chunk.len());

    // Limpeza explícita
    drop(ram);
    let _ = fs::remove_file("temp_data.bin");
    println!("Memória liberada e arquivo removido.");
}

fn write_ssd_simulation(path: &str, size_bytes: usize) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let chunk = vec![0u8; 1024 * 1024]; // escreve 1MB por vez
    let mut written = 0;
    while written < size_bytes {
        let to_write = (size_bytes - written).min(chunk.len());
        file.write_all(&chunk[..to_write])?;
        written += to_write;
    }
    Ok(())
}
