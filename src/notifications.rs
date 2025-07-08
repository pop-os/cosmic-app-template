use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::time::Duration;
use std::io::Write;

pub fn send_alarm_notification(label: &str, time: &str) {
    // Play alarm sound using system notification sound
    play_system_sound("alarm");
    
    let _ = Notification::new()
        .summary("🔔 Alarm")
        .body(&format!("⏰ {}\nTime: {}", label, time))
        .icon("alarm-symbolic")
        .timeout(Timeout::Milliseconds(10000))
        .urgency(notify_rust::Urgency::Critical)
        .show();
}

pub fn send_timer_notification() {
    // Play completion sound
    play_system_sound("complete");
    
    let _ = Notification::new()
        .summary("🔔 Timer Finished")
        .body("⏲️ Your timer has finished!")
        .icon("timer-symbolic")
        .timeout(Timeout::Milliseconds(8000))
        .urgency(notify_rust::Urgency::Critical)
        .show();
}

pub fn send_stopwatch_notification(time: &str) {
    // Single notification sound
    play_system_sound("message");
    
    let _ = Notification::new()
        .summary("⏱️ Stopwatch Stopped")
        .body(&format!("Final time: {}", time))
        .icon("chronometer-symbolic")
        .timeout(Timeout::Milliseconds(3000))
        .urgency(notify_rust::Urgency::Normal)
        .show();
}

pub fn send_alarm_set_notification(time: &str) {
    let _ = Notification::new()
        .summary("✅ Alarm Set")
        .body(&format!("Alarm scheduled for {}", time))
        .icon("alarm-symbolic")
        .timeout(Timeout::Milliseconds(2000))
        .urgency(notify_rust::Urgency::Low)
        .show();
}

fn play_system_sound(sound_type: &str) {
    let sound_type = sound_type.to_string();
    std::thread::spawn(move || {
        let sound_name = match sound_type.as_str() {
            "alarm" => "alarm-clock-elapsed",
            "complete" => "complete", 
            "message" => "message-new-instant",
            _ => "bell",
        };
        
        // Pop!_OS specific sound methods (in order of preference)
        let methods = vec![
            // Method 1: GNOME/Pop!_OS default sound player
            ("canberra-gtk-play", vec!["-i", sound_name]),
            ("canberra-gtk-play", vec!["-i", "bell"]),
            
            // Method 2: PulseAudio (standard on Pop!_OS)
            ("pactl", vec!["play-sample", sound_name]),
            ("pactl", vec!["play-sample", "bell"]),
            
            // Method 3: Direct sound file playback
            ("paplay", vec!["/usr/share/sounds/freedesktop/stereo/bell.oga"]),
            ("paplay", vec!["/usr/share/sounds/gnome/default/alerts/bark.ogg"]),
            
            // Method 4: ALSA fallback
            ("aplay", vec!["/usr/share/sounds/alsa/Front_Left.wav"]),
        ];
        
        let mut success = false;
        for (cmd, args) in methods {
            if let Ok(output) = Command::new(cmd).args(&args).output() {
                if output.status.success() {
                    success = true;
                    break;
                }
            }
        }
        
        // Pop!_OS fallback: Multiple beeps for different sound types
        if !success {
            let (repeat_count, interval_ms) = match sound_type.as_str() {
                "alarm" => (4, 250),    // Urgent alarm pattern
                "complete" => (2, 150), // Completion pattern  
                _ => (1, 100),          // Single beep
            };
            
            for i in 0..repeat_count {
                print!("\x07");
                std::io::stdout().flush().ok();
                if i < repeat_count - 1 {
                    std::thread::sleep(Duration::from_millis(interval_ms));
                }
            }
        }
    });
}
