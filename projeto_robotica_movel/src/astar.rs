
use crate::carteiro::Direcao;

pub struct Astar{
    trash: Vec<(i32, i32)>,
}

#[derive(Debug)]
pub struct VizinhoDirecao {
    pub vizinho: (i32, i32),
    pub direcao: Direcao,
}

impl Astar {
    pub fn new() -> Astar {
        Astar {
            trash: Vec::new(),
        }
    }


    pub fn descarta_caminho(&mut self, caminho: (i32, i32)) {
        if !self.trash.contains(&caminho){
            self.trash.push(caminho);
        }
    }

    pub fn verifica_rota(&mut self, carteiro: (i32, i32), destino: (i32, i32)) -> Direcao{

        // essa merda ta é errada ele não anda de X e Y ele anda de Y e X mds vou me matar
        let direcoes = vec![
            VizinhoDirecao {vizinho: (carteiro.0, carteiro.1 -1), direcao: Direcao::sul}, //Vai pro norte da tela, mas o norte do jogo é sul
            VizinhoDirecao {vizinho: (carteiro.0, carteiro.1 +1), direcao: Direcao::leste}, //Leste ta certo, mas em relação (y,x) - Direita
            VizinhoDirecao {vizinho: (carteiro.0-1, carteiro.1), direcao: Direcao::oeste}, //Oeste ta certo, mas em relação a (y,x) - Esquerda
            VizinhoDirecao {vizinho: (carteiro.0+1, carteiro.1), direcao: Direcao::norte}, //Vai pro sul da tela, mas o sul do jogo é norte
        ];

        let mut prev_distancia: f64 = f64::INFINITY;
        let mut vizin = (0,0);
        let mut direcao: Direcao = Direcao::norte;

        for item_vizinho in direcoes { //PORRRAAAAAAAA
            let distancia = self.distancia_entre_pontos(item_vizinho.vizinho, destino);
            if distancia < prev_distancia && !self.trash.contains(&item_vizinho.vizinho){
                prev_distancia = distancia;
                vizin = item_vizinho.vizinho;
                direcao = item_vizinho.direcao;

            }
            
            self.descarta_caminho(item_vizinho.vizinho);
           

            println!("Distância: {} - Vizinho: {:?}", distancia, item_vizinho.vizinho); // meu deus essa porra ta certa por que ele ta andando pro outro lado 😭😭

        }

            println!("Vizinho escolhido: \n Minha pos - {:?} \n Pos vizin - {:?} \n Direção - {:?}\n Lista de excluidos - {:?} \n Alvo: {:?}", carteiro, vizin, direcao, self.trash, destino);

        direcao
    }


    pub fn distancia_entre_pontos(&self, desejada: (i32, i32), destino: (i32, i32)) -> f64{
        let dx = (destino.0-desejada.0)  as f64;
        let dy = (destino.1-desejada.1)  as f64;

        return (dx*dx + dy*dy).sqrt();
    }




}