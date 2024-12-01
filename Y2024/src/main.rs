use color_eyre::Result;
use y2024::ui::app_ui::AppUI;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = AppUI::default().run(terminal).await;
    ratatui::restore();
    app_result
}

// Below some CHatGPT "answers" for questions regarding build the UI... Reconcille after some time maybe

// use std::sync::{Arc, Mutex};
// use tokio::task;
// use tokio::time::{self, Duration};

// // Eine Funktion, die den Zustand aktualisiert und ein Ergebnis zurückgibt
// async fn my_task_with_result(status: Arc<Mutex<String>>) -> Result<String, String> {
//     {
//         let mut state = status.lock().unwrap();
//         *state = "Task gestartet".to_string();
//     }

//     println!("Task läuft...");
//     time::sleep(Duration::from_secs(2)).await;

//     {
//         let mut state = status.lock().unwrap();
//         *state = "Task beendet".to_string();
//     }

//     Ok("Task erfolgreich!".to_string())
// }

// #[tokio::main]
// async fn main() {
//     // Gemeinsamer Zustand, der zwischen Tasks geteilt wird
//     let status = Arc::new(Mutex::new("Warten auf Start...".to_string()));

//     // Erstellen eines Handles für den Task
//     let status_clone = status.clone();
//     let handle = tokio::spawn(async move {
//         my_task_with_result(status_clone).await
//     });

//     // Schleife, um den Zustand in regelmäßigen Abständen zu drucken
//     loop {
//         {
//             let state = status.lock().unwrap();
//             println!("Status: {}", *state);
//             if *state == "Task beendet".to_string() {
//                 break;
//             }
//         }
//         time::sleep(Duration::from_millis(100)).await;
//     }

//     // Auf das Ergebnis des Tasks warten
//     match handle.await {
//         Ok(Ok(result)) => println!("Ergebnis: {}", result),
//         Ok(Err(err)) => eprintln!("Logikfehler im Task: {}", err),
//         Err(e) => eprintln!("Task fehlgeschlagen: {:?}", e),
//     }
// }

// ================================================================

// use ratatui::crossterm::{
//     event::{self, Event, KeyCode},
//     execute,
//     terminal::{self, disable_raw_mode, enable_raw_mode},
// };
// use ratatui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout},
//     style::{Color, Style},
//     widgets::{Block, Borders, Gauge},
//     Terminal,
// };
// use std::{
//     sync::{Arc, Mutex},
//     thread,
//     time::Duration,
// };
// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Terminal vorbereiten
//     enable_raw_mode()?;
//     let mut stdout = std::io::stdout();
//     execute!(stdout, terminal::EnterAlternateScreen)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // Shared State für den Fortschritt
//     let progress = Arc::new(Mutex::new(0u8));
//     let progress_clone = Arc::clone(&progress);

//     // Channel für Benachrichtigungen
//     let (tx, mut rx) = mpsc::channel::<()>(100);

//     // Task in einem separaten Thread starten, wenn `r` gedrückt wird
//     thread::spawn(move || {
//         loop {
//             if let Ok(Event::Key(key)) = event::read() {
//                 if key.code == KeyCode::Char('q') {
//                     break; // Beenden, wenn 'q' gedrückt wird
//                 }
//                 if key.code == KeyCode::Char('r') {
//                     // Task starten
//                     let progress_clone = Arc::clone(&progress_clone);
//                     let tx = tx.clone();
//                     thread::spawn(move || {
//                         for i in 0..=100 {
//                             {
//                                 let mut progress = progress_clone.lock().unwrap();
//                                 *progress = i; // Fortschritt aktualisieren
//                             }
//                             tx.blocking_send(()).unwrap(); // UI informieren
//                             thread::sleep(Duration::from_millis(50));
//                         }
//                     });
//                 }
//             }
//         }
//     });

//     // UI-Loop
//     loop {
//         // Fortschritt abfragen
//         if let Ok(_) = rx.try_recv() {
//             let progress_value = {
//                 let progress = progress.lock().unwrap();
//                 *progress
//             };

//             // UI aktualisieren
//             terminal.draw(|f| {
//                 let chunks = Layout::default()
//                     .direction(Direction::Vertical)
//                     .margin(1)
//                     .constraints(
//                         [Constraint::Length(3), Constraint::Length(7), Constraint::Min(1)]
//                             .as_ref(),
//                     )
//                     .split(f.area());

//                 let gauge = Gauge::default()
//                     .block(Block::default().title("Fortschritt").borders(Borders::ALL))
//                     .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
//                     .percent(progress_value.into());

//                 f.render_widget(gauge, chunks[1]);
//             })?;

//             if progress_value == 100 {
//                 // break; // Beenden, wenn Fortschritt 100% erreicht
//             }
//         }

//         // Tasteneingaben prüfen, ohne zu blockieren
//         if ratatui::crossterm::event::poll(Duration::from_millis(100))? {
//             if let Event::Key(key) = event::read()? {
//                 if key.code == KeyCode::Char('q') {
//                     break;
//                 }
//             }
//         }
//     }

//     // Terminal zurücksetzen
//     disable_raw_mode()?;
//     execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
//     terminal.show_cursor()?;

//     Ok(())
// }
