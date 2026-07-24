use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};
use std::time::Duration;

const FRAME_DATA: &[u8] = include_bytes!("../assets/stow.gif");

pub struct LogoAnim {
    pub frames: Vec<Vec<u8>>,
    pub width: u16,
    pub height: u16,
    total_duration: Duration,
}

impl LogoAnim {
    pub fn new() -> Result<Self, String> {
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let mut reader = decoder.read_info(FRAME_DATA).map_err(|e| e.to_string())?;

        let w = reader.width() as usize;
        let h = reader.height() as usize;
        let mut canvas = vec![0u8; w * h * 4];
        let mut previous = canvas.clone();
        let mut frames = Vec::new();
        let mut total_ms = 0u64;

        while let Some(frame) = reader.read_next_frame().map_err(|e| e.to_string())? {
            let fw = frame.width as usize;
            let fh = frame.height as usize;
            let fl = frame.left as usize;
            let ft = frame.top as usize;

            for y in 0..fh {
                for x in 0..fw {
                    let src = (y * fw + x) * 4;
                    let a = frame.buffer[src + 3];
                    if a < 128 {
                        continue;
                    }
                    let dx = fl + x;
                    let dy = ft + y;
                    if dx < w && dy < h {
                        let dst = (dy * w + dx) * 4;
                        canvas[dst..dst + 4].copy_from_slice(&frame.buffer[src..src + 4]);
                    }
                }
            }

            let delay_cs = frame.delay.max(1) as u64;
            total_ms += delay_cs * 10;
            frames.push(canvas.clone());

            match frame.dispose {
                gif::DisposalMethod::Any | gif::DisposalMethod::Keep => {}
                gif::DisposalMethod::Background => {
                    for y in 0..fh {
                        for x in 0..fw {
                            let dx = fl + x;
                            let dy = ft + y;
                            if dx < w && dy < h {
                                let dst = (dy * w + dx) * 4;
                                canvas[dst + 3] = 0;
                            }
                        }
                    }
                }
                gif::DisposalMethod::Previous => {
                    canvas.copy_from_slice(&previous);
                }
            }
            previous.copy_from_slice(&canvas);
        }

        Ok(LogoAnim {
            frames,
            width: w as u16,
            height: h as u16,
            total_duration: Duration::from_millis(total_ms),
        })
    }

    pub fn dim(&self) -> (u16, u16, u16) {
        let w = self.width;
        let h = self.height;
        let term_h = h / 2 + h % 2;
        (w, h, term_h)
    }

    pub fn frame_at(&self, elapsed: Duration) -> Option<&[u8]> {
        if self.frames.is_empty() {
            return None;
        }
        if elapsed >= self.total_duration {
            return self.frames.last().map(Vec::as_slice);
        }
        let n = self.frames.len();
        let ms = elapsed.as_millis() as u64;
        let total = self.total_duration.as_millis() as u64;
        let idx = (ms * n as u64 / total) as usize;
        let idx = idx.min(n - 1);
        self.frames.get(idx).map(Vec::as_slice)
    }

    pub fn last_frame(&self) -> &[u8] {
        self.frames.last().map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }
}

pub fn frame_to_text(pixels: &[u8], width: u16, height: u16) -> Text<'static> {
    let out_h = height / 2 + height % 2;
    let mut lines = Vec::with_capacity(out_h as usize);

    for y in (0..height).step_by(2) {
        let mut spans = Vec::with_capacity(width as usize);
        for x in 0..width {
            let top_base = (y * width + x) as usize * 4;
            let bot_base = ((y + 1) * width + x) as usize * 4;

            let top = &pixels[top_base..top_base + 4];
            let fg = if top[3] > 128 {
                Color::Rgb(top[0], top[1], top[2])
            } else {
                Color::Rgb(13, 17, 23)
            };

            let bg = if y + 1 < height {
                let b = &pixels[bot_base..bot_base + 4];
                if b[3] > 128 {
                    Color::Rgb(b[0], b[1], b[2])
                } else {
                    Color::Rgb(13, 17, 23)
                }
            } else {
                Color::Rgb(13, 17, 23)
            };

            spans.push(Span::styled("▀", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}
