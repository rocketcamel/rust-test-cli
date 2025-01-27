use std::sync::Mutex;

pub trait Powerup {
    fn activate_powerup(&self, powerup_name: &str);
    fn get_powerups(&self) -> Vec<String>;
    fn powerups_locked(&self) -> bool;
    fn get_recent(&self) -> Vec<String>;
}

pub struct PowerupManager {
    active_powerups: Mutex<Vec<String>>,
    recent_powerups: Mutex<Vec<String>>,
}

impl PowerupManager {
    pub fn new() -> Self {
        Self {
            active_powerups: Mutex::new(Vec::new()),
            recent_powerups: Mutex::new(Vec::new()),
        }
    }
}

impl Powerup for PowerupManager {
    fn activate_powerup(&self, powerup_name: &str) {
        let mut powerups = self.active_powerups.lock().unwrap();
        powerups.push(powerup_name.to_string());
        log::info!("Activated powerup {}", powerup_name);
        self.recent_powerups
            .lock()
            .unwrap()
            .push(powerup_name.to_string())
    }

    fn get_powerups(&self) -> Vec<String> {
        let powerups = self.active_powerups.lock().unwrap();
        powerups.clone()
    }

    fn get_recent(&self) -> Vec<String> {
        let recent_powerups = self.recent_powerups.lock().unwrap();
        recent_powerups.clone()
    }

    fn powerups_locked(&self) -> bool {
        match self.active_powerups.try_lock() {
            Ok(_powerups) => true,
            _ => false,
        }
    }
}
