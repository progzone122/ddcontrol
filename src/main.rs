extern crate alsa;
use alsa::mixer::{Mixer, SelemId, Selem};
use std::time::Duration;
use std::thread;

/* Настройки звука и текущие значения */
struct SoundSettings {
    volume_speaker: i64,
    mixer: Mixer,
    selem_id: SelemId
}

impl SoundSettings {
    fn new(initial_volume: i64, mixer: Mixer, selem_id: SelemId) -> Self {
        if let Some(selem) = mixer.find_selem(&selem_id) {
            // Элемент найден
            println!("Selem found!");
    
            // Получение текущего значения громкости
            let (min, max) = selem.get_playback_volume_range();
            println!("Volume range: {} - {}", min, max);
    
            // Установка нового значения громкости
            let volume = (max - min) * initial_volume / 100 + min;
            selem.set_playback_volume_all(volume).unwrap();
            println!("Volume set to {}", volume);
        } else {
            // Элемент не найден
            println!("Selem not found!");
        }
        SoundSettings {
            volume_speaker: initial_volume,
            mixer,
            selem_id
        }
    }
    fn increase_speaker_volume(&mut self, volume: i64) {
        self.volume_speaker += volume;
        if let Some(selem) = self.mixer.find_selem(&self.selem_id) {
            // Элемент найден
            println!("Selem found!");
    
            // Получение текущего значения громкости
            let (min, max) = selem.get_playback_volume_range();
            println!("Volume range: {} - {}", min, max);
    
            // Установка нового значения громкости
            let volume = (max - min) * self.volume_speaker / 100 + min;
            selem.set_playback_volume_all(volume).unwrap();
            println!("Volume set to {}", volume);
        } else {
            // Элемент не найден
            println!("Selem not found!");
        }
    }

    fn decrease_speaker_volume(&mut self, volume: i64) {
        self.volume_speaker -= volume;
        if let Some(selem) = self.mixer.find_selem(&self.selem_id) {
            // Элемент найден
            println!("Selem found!");
    
            // Получение текущего значения громкости
            let (min, max) = selem.get_playback_volume_range();
            println!("Volume range: {} - {}", min, max);
    
            // Установка нового значения громкости
            let volume = (max - min) * self.volume_speaker / 100 + min;
            selem.set_playback_volume_all(volume).unwrap();
            println!("Volume set to {}", volume);
        } else {
            // Элемент не найден
            println!("Selem not found!");
        }
    }
}

/* Действия для управления звуком */
enum SoundControl {
    Increase,
    Decrease,
}

impl SoundControl {
    fn speaker(&self, sound_settings: &mut SoundSettings, volume: i64) {
        match self {
            SoundControl::Increase => {
                sound_settings.increase_speaker_volume(volume);
            }
            SoundControl::Decrease => {
                sound_settings.decrease_speaker_volume(volume);
            }
        }
    }
}

fn main() {
    let mixer = Mixer::new("default", false).unwrap();
    let selem_id = SelemId::new("Master", 0);
    let mut sound_settings = SoundSettings::new(100, mixer, selem_id);

    SoundControl::Decrease.speaker(&mut sound_settings, 10);
    // SoundControl::Increase.speaker(&mut sound_settings, 30);

    // for i in 1..=10 {
    //     SoundControl::Decrease.speaker(&mut sound_settings, 10);
    //     thread::sleep(Duration::from_millis(500));
    // }
}