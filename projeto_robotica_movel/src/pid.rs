// src/pid.rs
pub struct PID {
    kp: f64, // Ganho Proporcional
    ki: f64, // Ganho Integral
    kd: f64, // Ganho Derivativo
    previous_error: f64,
    integral: f64,
}

impl PID {

    pub fn new(kp: f64, ki: f64, kd: f64) -> PID {
        PID {
            kp,
            ki,
            kd,
            previous_error: 0.0,
            integral: 0.0,
        }
    }

    pub fn calculate(&mut self, setpoint: f64, measured_value: f64, delta_time: f64) -> f64 {
        let error = setpoint - measured_value;
        self.integral += error * delta_time;

        self.integral = self.integral.clamp(-2.0,2.0); //limitação pra não ferrar com tudo





        let derivative = (error - self.previous_error) / delta_time;

        let derivative = derivative.clamp(-50.0, 50.0);


        self.previous_error = error;


        println!("Error: {}, Integral: {}, Derivative: {}, Output: {}, delta time: {}", error, self.integral, derivative, (self.kp * error) + (self.ki * self.integral) + (self.kd * derivative), delta_time);

        (self.kp * error) + (self.ki * self.integral) + (self.kd * derivative)
    }

}
