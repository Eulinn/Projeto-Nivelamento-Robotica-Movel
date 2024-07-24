// src/main.rs
use sensor::Sensor;
use pid::PID;
use jogo::Jogo;

mod sensor;
mod pid;
mod jogo;
mod carteiro;
mod caixa;

//  JOGO DA ENTREGA
//
// O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X') em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
// Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
// Obs:
//   - O carteiro só pode andar um 'índice' por iteração
//   - Apliquem a ideia de Encapsulamento
//   - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
//   - No dia da apresentação o código será posto em prática com um código diferente 

fn main() {
    let matriz: [[char; 20]; 20];

    let jogo = Jogo::new();
    jogo.cria_jogo();
}
