use std::sync::Mutex;

pub trait Powerup {
    fn activate_powerup(&self, powerup_name: &str);
    fn get_powerups(&self) -> Vec<String>;
    fn is_locked(&self) -> bool;
}

pub struct PowerupManager {
    active_powerups: Mutex<Vec<String>>,
}

impl PowerupManager {
    pub fn new() -> Self {
        Self {
            active_powerups: Mutex::new(Vec::new()),
        }
    }
}

impl Powerup for PowerupManager {
    fn activate_powerup(&self, powerup_name: &str) {
        let mut powerups = self.active_powerups.lock().unwrap();
        powerups.push(powerup_name.to_string());
        println!("Activated powerup {}", powerup_name);
    }

    fn get_powerups(&self) -> Vec<String> {
        let powerups = self.active_powerups.lock().unwrap();
        powerups.clone()
    }

    fn is_locked(&self) -> bool {
        match self.active_powerups.try_lock() {
            Ok(_powerups) => true,
            _ => false,
        }
    }
}
