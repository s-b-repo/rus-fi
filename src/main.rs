use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, Scale, DrawingArea};
use glib::{Continue};
use gstreamer as gst;
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::process::Command;
use rand::Rng;
use gstreamer::prelude::ObjectExt;

// The original LoFi Girl YouTube URL:
const LOFI_STREAM_URL: &str = "https://www.youtube.com/watch?v=jfKfPfyJRdk";

// Number of random bars in our fake EQ visualization:
const NUM_EQ_BARS: usize = 15;

// Labels for the 7 control buttons:
const LABEL_PLAY: &str = "Play";
const LABEL_PAUSE: &str = "Pause";
const LABEL_RESUME: &str = "Resume";
const LABEL_REWIND: &str = "Rewind";
const LABEL_STOP: &str = "Stop";

// A small helper struct to hold our GStreamer pipeline
struct Player {
    pipeline: gst::Element, // We'll use "playbin"
}

impl Player {
    fn new() -> Self {
        // Create a "playbin" element via the new builder API.
        // If your version doesn't provide .build(), remove that and just do
        // ElementFactory::make("playbin", Some("my-playbin")) or similar.
        let pipeline = gst::ElementFactory::make("playbin")
        .name("my-playbin")
        .build();

        // Make sure it's in Null state initially:
        pipeline.clone().expect("REASON").set_state(gst::State::Null).unwrap();

        Self { pipeline: pipeline.expect("REASON") }
    }

    fn set_uri(&self, uri: &str) {
        // set_property returns (), so no .expect(...) or .ok() needed
        self.pipeline.set_property("uri", uri);
    }

    fn play(&self) {
        let _ = self.pipeline.set_state(gst::State::Playing);
    }

    fn pause(&self) {
        let _ = self.pipeline.set_state(gst::State::Paused);
    }

    fn stop(&self) {
        let _ = self.pipeline.set_state(gst::State::Null);
    }

    fn resume(&self) {
        self.play();
    }

    // Attempt to seek backward by some margin (e.g., 10 seconds).
    fn rewind(&self, seconds: u64) {
        let position = match self.pipeline.query_position::<gst::ClockTime>() {
            Some(pos) => pos,
            None => return,
        };
        let new_pos = if position > gst::ClockTime::from_seconds(seconds) {
            position - gst::ClockTime::from_seconds(seconds)
        } else {
            gst::ClockTime::ZERO
        };
        let _ = self.seek(new_pos);
    }


    fn seek(&self, position: gst::ClockTime) -> bool {
        self.pipeline
        .seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE, position)
        .is_ok()
    }

    // Adjust volume: 0.0 = muted, 1.0 = max volume
    fn set_volume(&self, vol: f64) {
        self.pipeline.set_property("volume", &vol);
    }
}

// Helper function that calls out to `yt-dlp` to retrieve the direct audio URL.
// Returns Some(URL) if successful, or None if there's an error.
fn get_direct_audio_url(youtube_link: &str) -> Option<String> {
    // Example command: yt-dlp -f bestaudio --get-url "<link>"
    let output = Command::new("yt-dlp")
    .args(["-f", "bestaudio", "--get-url", youtube_link])
    .output()
    .ok()?;

    if !output.status.success() {
        eprintln!("yt-dlp failed with status: {:?}", output.status);
        return None;
    }

    // The output should be a direct media URL in plain text:
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    // Usually it's just one line. Trim it and return as a String.
    let direct_url = stdout_str.trim().to_string();

    if direct_url.is_empty() {
        eprintln!("yt-dlp returned an empty URL.");
        None
    } else {
        Some(direct_url)
    }
}

fn main() {
    // Initialize GTK and GStreamer:
    gtk::init().expect("Failed to init GTK.");
    gst::init().expect("Failed to init GStreamer.");

    // Create a new GTK Application
    let app = Application::new(Some("com.example.lofi-gui-yt-dlp"), Default::default());

    // We'll keep our Player inside an Arc<Mutex<...>> so multiple callbacks can access it
    let player = Arc::new(Mutex::new(Player::new()));

    app.connect_activate(move |app: &Application| {
        // 1) Call yt-dlp to get the direct audio link
        let direct_url = match get_direct_audio_url(LOFI_STREAM_URL) {
            Some(url) => url,
                         None => {
                             eprintln!("Failed to retrieve direct audio URL from yt-dlp. Exiting.");
                             return;
                         }
        };

        // 2) Once we have it, set the pipeline's URI
        {
            let p = player.lock().unwrap();
            p.set_uri(&direct_url);
        }

        // Build the main application window
        let window = ApplicationWindow::new(app);
        window.set_title("LoFi Music Player (Audio Only) via yt-dlp");
        window.set_default_size(800, 400);

        // Main vertical box that holds:
        //   (1) horizontal box of buttons
        //   (2) volume slider
        //   (3) eq drawing area
        let vbox = gtk::Box::new(Orientation::Vertical, 10);
        vbox.set_vexpand(true);
        vbox.set_hexpand(true);

        // Create a horizontal box for the 7 buttons
        let hbox_buttons = gtk::Box::new(Orientation::Horizontal, 5);
        hbox_buttons.set_hexpand(true);

        // Helper to create a button
        let make_button = |label: &str| -> Button {
            let btn = Button::with_label(label);
            btn.set_hexpand(true);
            btn
        };

        // Create our 7 buttons
        let btn_play = make_button(LABEL_PLAY);
        let btn_pause = make_button(LABEL_PAUSE);
        let btn_resume = make_button(LABEL_RESUME);
        let btn_rewind = make_button(LABEL_REWIND);
        let btn_stop = make_button(LABEL_STOP);

        // Add them all horizontally
        hbox_buttons.pack_start(&btn_play, true, true, 0);
        hbox_buttons.pack_start(&btn_pause, true, true, 0);
        hbox_buttons.pack_start(&btn_resume, true, true, 0);
        hbox_buttons.pack_start(&btn_rewind, true, true, 0);
        hbox_buttons.pack_start(&btn_stop, true, true, 0);

        // Volume slider (horizontal); Range: 0.0 to 1.0
        let volume_scale = Scale::new(Orientation::Horizontal, None::<&gtk::Adjustment>);
        volume_scale.set_range(0.0, 1.0);
        volume_scale.set_value(1.0); // default at full volume
        volume_scale.set_draw_value(true);
        volume_scale.set_hexpand(true);

        // A small random “equalizer” drawing area
        let eq_area = DrawingArea::new();
        eq_area.set_vexpand(true);
        eq_area.set_hexpand(true);

        // Pack everything into the vertical box
        vbox.pack_start(&hbox_buttons, false, false, 0);
        vbox.pack_start(&volume_scale, false, false, 0);
        vbox.pack_start(&eq_area, true, true, 0);

        // Put the vbox in the window
        window.add(&vbox);

        // Clone references to player for each callback
        {
            let player_play = Arc::clone(&player);
            btn_play.connect_clicked(move |_| {
                let p = player_play.lock().unwrap();
                p.play();
            });
        }

        {
            let player_pause = Arc::clone(&player);
            btn_pause.connect_clicked(move |_| {
                let p = player_pause.lock().unwrap();
                p.pause();
            });
        }

        {
            let player_resume = Arc::clone(&player);
            btn_resume.connect_clicked(move |_| {
                let p = player_resume.lock().unwrap();
                p.resume();
            });
        }

        {
            let player_rewind = Arc::clone(&player);
            btn_rewind.connect_clicked(move |_| {
                let p = player_rewind.lock().unwrap();
                // Rewind 10 seconds just as an example
                p.rewind(10);
            });
        }

        {
            let player_stop = Arc::clone(&player);
            btn_stop.connect_clicked(move |_| {
                let p = player_stop.lock().unwrap();
                p.stop();
            });
        }

        // Volume scale changes
        {
            let player_volume = Arc::clone(&player);
            volume_scale.connect_value_changed(move |scale| {
                let val = scale.value();
                let p = player_volume.lock().unwrap();
                p.set_volume(val);
            });
        }

        // Random “equalizer” effect: We'll store an array of bar heights and update ~10 times/sec
        let eq_bar_heights = Arc::new(Mutex::new(vec![0.0; NUM_EQ_BARS]));

        // Connect a draw handler for eq_area
        {
            let eq_bar_heights = Arc::clone(&eq_bar_heights);
            eq_area.connect_draw(move |area, cr| {
                let width = area.allocated_width() as f64;
                let height = area.allocated_height() as f64;

                let bar_width = if NUM_EQ_BARS > 0 {
                    width / NUM_EQ_BARS as f64
                } else {
                    0.0
                };

                // Retrieve the current bars
                if let Ok(bars) = eq_bar_heights.lock() {
                    for (i, &bar_val) in bars.iter().enumerate() {
                        let x = i as f64 * bar_width;
                        let bar_height = bar_val * height;
                        let y = height - bar_height;

                        cr.rectangle(x, y, bar_width * 0.8, bar_height);
                        cr.fill().unwrap();
                    }
                }

                Inhibit(false)
            });
        }

        // Periodically update the bar heights and queue a redraw
        {
            let eq_area_clone = eq_area.clone();
            let eq_bar_heights_clone = Arc::clone(&eq_bar_heights);
            glib::timeout_add_local(Duration::from_millis(100), move || {
                if let Ok(mut bars) = eq_bar_heights_clone.lock() {
                    for bar in bars.iter_mut() {
                        *bar = rand::thread_rng().gen_range(0.0..1.0);
                    }
                }
                eq_area_clone.queue_draw();
                Continue(true)
            });
        }

        // Show the window and all children
        window.show_all();
    });

    // Run the GTK application
    app.run();
}
