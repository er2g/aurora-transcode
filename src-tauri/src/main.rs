#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::api::process::{Command as SidecarCommand, CommandEvent};
use tauri::{State, Window};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionSettings {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub video_bitrate: Option<String>,
    pub audio_bitrate: Option<String>,
    pub resolution: Option<String>,
    pub fps: Option<String>,
    pub quality: Option<String>,
    pub preset: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConversionProgress {
    pub percentage: f32,
    pub time: String,
    pub speed: String,
    pub bitrate: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FormatInfo {
    pub name: String,
    pub extensions: Vec<String>,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PresetInfo {
    pub name: String,
    pub description: String,
    pub settings: ConversionSettings,
}

struct AppState {
    conversion_active: Arc<Mutex<bool>>,
}

#[tauri::command]
async fn convert_media(
    settings: ConversionSettings,
    window: Window,
    state: State<'_, AppState>,
) -> Result<String, String> {
    {
        let mut conversion_active = state.conversion_active.lock().unwrap();
        *conversion_active = true;
    }

    let mut args = vec![
        "-i".to_string(),
        settings.input_path.clone(),
        "-y".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
    ];

    // Video codec
    if let Some(vcodec) = &settings.video_codec {
        args.push("-c:v".to_string());
        args.push(vcodec.clone());
    }

    // Audio codec
    if let Some(acodec) = &settings.audio_codec {
        args.push("-c:a".to_string());
        args.push(acodec.clone());
    }

    // Video bitrate
    if let Some(vbitrate) = &settings.video_bitrate {
        args.push("-b:v".to_string());
        args.push(vbitrate.clone());
    }

    // Audio bitrate
    if let Some(abitrate) = &settings.audio_bitrate {
        args.push("-b:a".to_string());
        args.push(abitrate.clone());
    }

    // Resolution
    if let Some(resolution) = &settings.resolution {
        args.push("-s".to_string());
        args.push(resolution.clone());
    }

    // FPS
    if let Some(fps) = &settings.fps {
        args.push("-r".to_string());
        args.push(fps.clone());
    }

    // Quality (CRF for video)
    if let Some(quality) = &settings.quality {
        args.push("-crf".to_string());
        args.push(quality.clone());
    }

    // Preset
    if let Some(preset) = &settings.preset {
        args.push("-preset".to_string());
        args.push(preset.clone());
    }

    args.push(settings.output_path.clone());

    // Prefer the bundled sidecar (embedded ffmpeg) when available; fall back to system ffmpeg.
    if let Ok(mut cmd) = SidecarCommand::new_sidecar("ffmpeg") {
        cmd = cmd.args(&args);
        let (mut rx, child) = cmd
            .spawn()
            .map_err(|e| format!("FFmpeg baslatilamadi: {}", e))?;

        let mut stderr_buf = String::new();
        let mut exit_code: Option<i32> = None;

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    for line in line.lines() {
                        if let Some(time_str) = line.split("out_time_ms=").nth(1) {
                            if let Ok(time_ms) = time_str.trim().parse::<f64>() {
                                let time_sec = time_ms / 1_000_000.0;
                                let progress = ConversionProgress {
                                    percentage: 0.0,
                                    time: format!("{:.1}s", time_sec),
                                    speed: "1x".to_string(),
                                    bitrate: "0kbits/s".to_string(),
                                };
                                let _ = window.emit("conversion-progress", &progress);
                            }
                        }
                    }
                }
                CommandEvent::Stderr(line) => {
                    if stderr_buf.len() < 32_768 {
                        stderr_buf.push_str(&line);
                        stderr_buf.push('\n');
                    }
                }
                CommandEvent::Terminated(payload) => {
                    exit_code = payload.code;
                }
                _ => {}
            }

            let conversion_active = state.conversion_active.lock().unwrap();
            if !*conversion_active {
                let _ = child.kill();
                return Err("Dönüştürme iptal edildi".to_string());
            }
        }

        if exit_code.unwrap_or(1) == 0 {
            return Ok("Dönüştürme başarılı!".to_string());
        }

        if stderr_buf.trim().is_empty() {
            return Err("FFmpeg hatası".to_string());
        }
        return Err(format!("FFmpeg hatası: {}", stderr_buf.trim()));
    }

    let mut child = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("FFmpeg başlatılamadı: {}", e))?;

    let stdout = child
        .stdout
        .take()
        .ok_or("FFmpeg stdout alınamadı")?;

    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("out_time_ms=") {
                if let Some(time_str) = line.split("out_time_ms=").nth(1) {
                    if let Ok(time_ms) = time_str.trim().parse::<f64>() {
                        let time_sec = time_ms / 1_000_000.0;
                        let progress = ConversionProgress {
                            percentage: 0.0,
                            time: format!("{:.1}s", time_sec),
                            speed: "1x".to_string(),
                            bitrate: "0kbits/s".to_string(),
                        };
                        let _ = window.emit("conversion-progress", &progress);
                    }
                }
            }
        }

        let conversion_active = state.conversion_active.lock().unwrap();
        if !*conversion_active {
            let _ = child.kill();
            return Err("Dönüştürme iptal edildi".to_string());
        }
    }

    let output = child.wait_with_output()
        .map_err(|e| format!("FFmpeg çalıştırılamadı: {}", e))?;

    if output.status.success() {
        Ok("Dönüştürme başarılı!".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("FFmpeg hatası: {}", error))
    }
}

#[tauri::command]
async fn cancel_conversion(state: State<'_, AppState>) -> Result<(), String> {
    let mut conversion_active = state.conversion_active.lock().unwrap();
    *conversion_active = false;
    Ok(())
}

#[tauri::command]
async fn get_supported_formats() -> Result<Vec<FormatInfo>, String> {
    Ok(vec![
        // Video Formats
        FormatInfo {
            name: "MP4".to_string(),
            extensions: vec!["mp4".to_string()],
            description: "MPEG-4 Part 14 - En yaygın video formatı".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "AVI".to_string(),
            extensions: vec!["avi".to_string()],
            description: "Audio Video Interleave".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "MKV".to_string(),
            extensions: vec!["mkv".to_string()],
            description: "Matroska Video - Yüksek kalite".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "MOV".to_string(),
            extensions: vec!["mov".to_string()],
            description: "QuickTime Movie".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "WEBM".to_string(),
            extensions: vec!["webm".to_string()],
            description: "Web video formatı".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "FLV".to_string(),
            extensions: vec!["flv".to_string()],
            description: "Flash Video".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "WMV".to_string(),
            extensions: vec!["wmv".to_string()],
            description: "Windows Media Video".to_string(),
            category: "video".to_string(),
        },
        FormatInfo {
            name: "M4V".to_string(),
            extensions: vec!["m4v".to_string()],
            description: "iTunes Video".to_string(),
            category: "video".to_string(),
        },
        // Audio Formats
        FormatInfo {
            name: "MP3".to_string(),
            extensions: vec!["mp3".to_string()],
            description: "MPEG Audio Layer 3".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "WAV".to_string(),
            extensions: vec!["wav".to_string()],
            description: "Waveform Audio - Lossless".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "FLAC".to_string(),
            extensions: vec!["flac".to_string()],
            description: "Free Lossless Audio Codec".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "AAC".to_string(),
            extensions: vec!["aac".to_string()],
            description: "Advanced Audio Coding".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "OGG".to_string(),
            extensions: vec!["ogg".to_string()],
            description: "Ogg Vorbis".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "WMA".to_string(),
            extensions: vec!["wma".to_string()],
            description: "Windows Media Audio".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "M4A".to_string(),
            extensions: vec!["m4a".to_string()],
            description: "MPEG-4 Audio".to_string(),
            category: "audio".to_string(),
        },
        FormatInfo {
            name: "OPUS".to_string(),
            extensions: vec!["opus".to_string()],
            description: "Opus Audio - Modern codec".to_string(),
            category: "audio".to_string(),
        },
    ])
}

#[tauri::command]
async fn get_presets() -> Result<Vec<PresetInfo>, String> {
    Ok(vec![
        PresetInfo {
            name: "Yüksek Kalite Video".to_string(),
            description: "En iyi görüntü kalitesi için".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "mp4".to_string(),
                video_codec: Some("libx264".to_string()),
                audio_codec: Some("aac".to_string()),
                video_bitrate: Some("5000k".to_string()),
                audio_bitrate: Some("320k".to_string()),
                resolution: None,
                fps: None,
                quality: Some("18".to_string()),
                preset: Some("slow".to_string()),
            },
        },
        PresetInfo {
            name: "Web Optimize".to_string(),
            description: "Web için optimize edilmiş, küçük dosya".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "mp4".to_string(),
                video_codec: Some("libx264".to_string()),
                audio_codec: Some("aac".to_string()),
                video_bitrate: Some("1500k".to_string()),
                audio_bitrate: Some("128k".to_string()),
                resolution: Some("1280x720".to_string()),
                fps: Some("30".to_string()),
                quality: Some("23".to_string()),
                preset: Some("fast".to_string()),
            },
        },
        PresetInfo {
            name: "Mobil Uyumlu".to_string(),
            description: "Mobil cihazlar için optimize".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "mp4".to_string(),
                video_codec: Some("libx264".to_string()),
                audio_codec: Some("aac".to_string()),
                video_bitrate: Some("800k".to_string()),
                audio_bitrate: Some("96k".to_string()),
                resolution: Some("854x480".to_string()),
                fps: Some("24".to_string()),
                quality: Some("28".to_string()),
                preset: Some("veryfast".to_string()),
            },
        },
        PresetInfo {
            name: "Sadece Ses Çıkar".to_string(),
            description: "Videodan ses dosyası oluştur".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "mp3".to_string(),
                video_codec: None,
                audio_codec: Some("libmp3lame".to_string()),
                video_bitrate: None,
                audio_bitrate: Some("320k".to_string()),
                resolution: None,
                fps: None,
                quality: None,
                preset: None,
            },
        },
        PresetInfo {
            name: "Kayıpsız Audio".to_string(),
            description: "En yüksek ses kalitesi".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "flac".to_string(),
                video_codec: None,
                audio_codec: Some("flac".to_string()),
                video_bitrate: None,
                audio_bitrate: None,
                resolution: None,
                fps: None,
                quality: None,
                preset: None,
            },
        },
        PresetInfo {
            name: "4K Video".to_string(),
            description: "Ultra HD video dönüşümü".to_string(),
            settings: ConversionSettings {
                input_path: String::new(),
                output_path: String::new(),
                format: "mp4".to_string(),
                video_codec: Some("libx265".to_string()),
                audio_codec: Some("aac".to_string()),
                video_bitrate: Some("15000k".to_string()),
                audio_bitrate: Some("320k".to_string()),
                resolution: Some("3840x2160".to_string()),
                fps: Some("60".to_string()),
                quality: Some("20".to_string()),
                preset: Some("slow".to_string()),
            },
        },
    ])
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            conversion_active: Arc::new(Mutex::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            convert_media,
            cancel_conversion,
            get_supported_formats,
            get_presets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
