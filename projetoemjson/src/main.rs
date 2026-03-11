use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct Usuario {
    id: u32,
    nome: String,
    idade: u8,
}

fn ler<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    match s.trim().parse::<T>() {
        Ok(valor) => valor,
        Err(_) => {
            println!("Erro: Entrada inválida! Tente digitar um número/texto correto.");
            // Recursão: tenta ler de novo até o usuário acertar
            ler()
        }
    }
}

fn salvar_no_arquivo(usuarios: &Vec<Usuario>) {
    let json = serde_json::to_string_pretty(usuarios).expect("Erro ao gerar JSON");
    fs::write("usuarios.json", json).expect("Erro ao escrever no arquivo");
}

fn carregar_arquivo() -> Vec<Usuario> {
    let conteudo = fs::read_to_string("usuarios.json").unwrap_or_else(|_| String::from("[]"));
    serde_json::from_str(&conteudo).expect("Erro ao processar o JSON")
}

fn main() {
    loop{
        let mut lista_usuarios = carregar_arquivo();

        println!("\n=== SISTEMA CRUD EM JSON ===");
        println!("1. Cadastrar usuário");
        println!("2. Listar todos");
        println!("3. Deleter por ID");
        println!("4. Sair");
        println!("Escolha uma opcao\n");

        let opcao: u32 = ler();

        match opcao {
            1 => {
                println!("Digite o nome: ");
                let nome: String = ler();
                println!("Idade: ");
                let idade: u8 = ler();

                let novo = Usuario {
                    id: (lista_usuarios.len() as u32) + 1,
                    nome,
                    idade,
                };

                lista_usuarios.push(novo);
                salvar_no_arquivo(&lista_usuarios);
                println!("\nUsuario Cadastrado\n");
            }

            2 => {
                println!("\n === Lista de usuarios ===");
                if lista_usuarios.is_empty(){
                    println!("Lista está vazia");
                }
                else{
                    for u in &lista_usuarios {
                        println!("[ID: {}] Nome: {} | Idade: {}", u.id, u.nome, u.idade);
                    }
                }
            }

            3 => {
                println!("Digite o ID que deseja remover:");
                let id_alvo: u32 = ler();

                let total_antes = lista_usuarios.len();

                lista_usuarios.retain(|u| u.id != id_alvo);

                if lista_usuarios.len() < total_antes {
                    salvar_no_arquivo(&lista_usuarios);
                    println!("Usuario removido.");
                }
                else{
                    println!("Erro: ID {} não encontrado.", id_alvo);
                }
            }

            4 => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Digite uma das opções");
            }
        }
    }
        

    

}
