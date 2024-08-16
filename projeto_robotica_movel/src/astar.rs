

pub struct Astar{
    trash: Vec<(i32, i32)>,
    actual: Vec<(i32, i32)>,
}


impl Astar {
    pub fn new() -> Astar {
        Astar {
            trash: Vec::new(),
            actual: Vec::new(),
        }
    }


    pub fn adiciona_caminho_actual(&mut self, caminho: (i32, i32)) {
        self.actual.push(caminho);
    }

    pub fn descarta_caminho(&mut self, caminho: (i32, i32)) {
        self.trash.push(caminho);
    }

    pub fn verifica_rota(&mut self, carteiro: (i32, i32), destino: (i32, i32)) -> (i32,i32){
        let direcoes: Vec<(i32, i32)> = vec![
            (carteiro.0, carteiro.1 -1), //cima
            (carteiro.0, carteiro.1 +1), //baixo
            (carteiro.0-1, carteiro.1), //esquerda
            (carteiro.0+1, carteiro.1), //direita
        ];

        let mut prevDistancia: f64 = f64::INFINITY;
        let mut escolhido: (i32,i32) = (0,0);

        for vizinho in direcoes {
            let distancia = self.distancia_entre_pontos(vizinho, destino);
            if distancia < prevDistancia{
                escolhido = vizinho;
                prevDistancia = distancia;
            }

        }

        escolhido
    }


    pub fn distancia_entre_pontos(&self, desejada: (i32, i32), destino: (i32, i32)) -> f64{
        let dx = destino.0-desejada.0;
        let dy = destino.1-desejada.1;

        return ((dx*dx + dy*dy) as f64).sqrt();
    }




}