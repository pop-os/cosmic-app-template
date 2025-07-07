use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::time::Duration;

pub fn send_alarm_notification(label: &str, time: &str) {
    // Play multiple beeps for alarm
    play_beep_sequence(vec![800, 800, 800], 200);
    
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
    play_beep_sequence(vec![600, 400], 300);
    
    let _ = Notification::new()
        .summary("🔔 Timer Finished")
        .body("⏲️ Your timer has finished!")
        .icon("timer-symbolic")
        .timeout(Timeout::Milliseconds(8000))
        .urgency(notify_rust::Urgency::Critical)
        .show();
}

pub fn send_stopwatch_notification(time: &str) {
    // Single beep for stopwatch
    play_beep_sequence(vec![400], 200);
    
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

fn play_beep_sequence(frequencies: Vec<i32>, duration_ms: u64) {
    std::thread::spawn(move || {
        for freq in frequencies {
            // Try different beep commands
            let _ = Command::new("beep")
                .arg("-f")
                .arg(freq.to_string())
                .arg("-l")
                .arg(duration_ms.to_string())
                .output();
                
            // Alternative: Use paplay with generated tone
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "paplay <(sox -n -t wav - synth 0.{} sine {})",
                    duration_ms / 100,
                    freq
                ))
                .output();
                
            // Simple fallback: terminal bell
            let _ = Command::new("echo")
                .arg("-e")
                .arg("\\a")
                .output();
                
            std::thread::sleep(Duration::from_millis(duration_ms + 50));
        }
    });
}
