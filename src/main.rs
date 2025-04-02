// Untuk komunikasi antar thread, jadi kita bisa 
// irim data antar thread
use std::sync::mpsc;

// Untuk membuat dan mengelola thread 
// (misalnya buat stopwatch berjalan di thread terpisah)
use std::thread;

// Untuk menghitung waktu, kita butuh Duration 
// (berapa lama) dan Instant (waktu sekarang)
use std::time::{Duration, Instant};

// Untuk input keyboard
// (misalnya untuk menghentikan stopwatch)
use crossterm::event::{self, Event, KeyCode};

// chrono untuk dapetin waktu 
// lokal sekarang (tanggal dan jam)
use chrono::Local;

/* Stopwatch struct untuk menyimpan informasi tentang stopwatch */
struct Stopwatch {
    start_time: Option<Instant>,    // waktu mulai stopwatch
    elapsed: Duration,              // waktu yang sudah berlalu
    is_running: bool,               // apakah stopwatch sedang berjalan
}

/* implementasi method untuk Stopwatch struct */
impl Stopwatch {
    // constructor untuk Stopwatch
    fn new() -> Self {
        Stopwatch {
            start_time: None,
            elapsed: Duration::default(),
            is_running: false,
        }
    }

    // method untuk memulai stopwatch
    fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }

    // method untuk menghentikan stopwatch
    fn pause(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed += start.elapsed();
            self.start_time = None;
            self.is_running = false;
        }
    }

    // method untuk melanjutkan stopwatch
    fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::default();
        self.is_running = false;
    }

    // method untuk mendapatkan waktu yang sudah berlalu
    fn current_duration(&self) -> Duration {
        let mut total = self.elapsed;
        if let Some(start) = self.start_time {
            total += start.elapsed();
        }
        total
    }
}

// fungsi untuk menangani input dari keyboard
fn input_handler(tx: mpsc::Sender<KeyCode>) { 
    loop {
        // ada event yang dateng gak? selama 100ms
        if event::poll(Duration::from_millis(100)).unwrap() { 
            
            // kalo ada event, baca eventnya
            if let Event::Key(key) = event::read().unwrap() {
                // kirim ke channel
                let _ = tx.send(key.code);
            }
        }
    }
}

// fungsi main utama
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // buat channel untuk komunikasi antar thread
    let (tx, rx) = mpsc::channel();
    
    // thread buat input
    thread::spawn(move || {
        input_handler(tx);
    });

    let mut stopwatch = Stopwatch::new(); // stopwatch baru
    let mut last_update = Instant::now(); // waktu terakhir update tampilan
    
    loop {
        // Update tampilan tiap 100ms
        if last_update.elapsed() > Duration::from_millis(100) {
            let duration = stopwatch.current_duration();
            let time_str = format!(
                "\r{:02}:{:02}:{:02}", 
                duration.as_secs() / 3600,
                (duration.as_secs() % 3600) / 60,
                duration.as_secs() % 60
            );
            print!("{}", time_str);
            last_update = Instant::now();
        }

        // Cek input
        if let Ok(key) = rx.try_recv() {
            match key {
                KeyCode::Char('s') => stopwatch.start(),    // kalo 's' ditekan, start stopwatch
                KeyCode::Char('p') => stopwatch.pause(),    // kalo 'p' ditekan, pause stopwatch
                KeyCode::Char('r') => stopwatch.reset(),    // kalo 'r' ditekan, reset stopwatch
                KeyCode::Char('q') => break,                // kalo 'q' ditekan, keluar dari loop
                _ => {}
            }
        }

        // tidur sejenak supaya CPU gak terlalu sibuk
        thread::sleep(Duration::from_millis(10));
    }
    
    Ok(())
}
