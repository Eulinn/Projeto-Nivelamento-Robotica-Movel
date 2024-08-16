
use std::time::Instant;

use crate::pid::PID;
use crate::sensor::Sensor;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direcao {
    norte,
    sul,
    leste,
    oeste,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status {
    jogando_sem_caixa,
    jogando_com_caixa,
    fim,
}

pub struct Carteiro {
    pos_x: i32,
    pos_y: i32,
    status: Status,
    sensor: Sensor,
    direcao: Direcao,
    pid_pitch: PID,
    pid_roll: PID,
    last_update: Instant,
}

impl Carteiro {
    // Construtor
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos_x: x,
            pos_y: y,
            status: Status::jogando_sem_caixa,
            sensor: Sensor::new(),
            direcao: Direcao::norte,
            pid_pitch: PID::new(0.9, 0.0005,0.0005),
            pid_roll: PID::new(0.9, 0.0005,0.0005),
            last_update: Instant::now(),
        }
    }

    // Metodos de get para os atributos
    pub fn get_pos_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32 {
        self.pos_y
    }

    pub fn get_sensor_pitch(&self) -> f64 {
        self.sensor.get_pitch()
    }

    pub fn get_sensor_roll(&self) -> f64 {
        self.sensor.get_roll()
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_direcao(&self) -> Direcao {
        self.direcao
    }

    pub fn set_status(&mut self, novo_status: Status) {
        self.status = novo_status;
    }

    pub fn muda_direcao(&mut self, nova_direcao: Direcao) {
        self.direcao = nova_direcao;
    }

    // Uptdate OBRIGATORIO do sensor
    pub fn update_sensor(&mut self) {
        self.sensor.update();

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_update).as_secs_f64();
        self.last_update = current_time;

        if delta_time < 0.001 {
            return;
        }


        let current_pitch = self.sensor.get_pitch();
        let current_roll = self.sensor.get_roll();

        let correction_pitch = self.pid_pitch.calculate(0.0, current_pitch, delta_time);
        let correction_roll = self.pid_roll.calculate(0.0, current_roll, delta_time);

        self.sensor.corr_pitch(correction_pitch);
        self.sensor.corr_roll(correction_roll);

    }

    // Verifica a possibilidade do robo se locomover para tal direcao
    pub fn verifica_andar(&self) -> bool {
        match self.get_direcao() {
            Direcao::norte => {
                if self.get_pos_y() >= 19 {
                    false
                } else {
                    true
                }
            }
            Direcao::sul => {
                if self.get_pos_x() <= 0 {
                    false
                } else {
                    true
                }
            }
            Direcao::leste => {
                if self.get_pos_x() >= 19 {
                    false
                } else {
                    true
                }
            }
            Direcao::oeste => {
                if self.get_pos_x() <= 0 {
                    false
                } else {
                    true
                }
            }
        }
    }

    // Utiliza a funcao verifica_andar() para saber a possibilidade de locomocao do robo.
    // Se verdadeiro, entao move o robo sem retorno.
    // Se falso, retorna falso sem mover o robo
    pub fn andar(&mut self) -> Option<bool> {
        match self.get_direcao() {
            Direcao::norte => {
                if self.get_pos_x() >= 19 {
                    Some(false)
                } else {
                    self.pos_x += 1;
                    None
                }
            }
            Direcao::sul => {
                if self.get_pos_x() <= 0 {
                    Some(false)
                } else {
                    self.pos_x -= 1;
                    None
                }
            }
            Direcao::leste => {
                if self.get_pos_y() >= 19 {
                    Some(false)
                } else {
                    self.pos_y += 1;
                    None
                }
            }
            Direcao::oeste => {
                if self.get_pos_y() <= 0 {
                    Some(false)
                } else {
                    self.pos_y -= 1;
                    None
                }
            }
        }
    }
}
