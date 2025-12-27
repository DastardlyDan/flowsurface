use iced::{daemon, Element, Settings, Theme};
use std::sync::atomic::{AtomicBool, Ordering};

static VIEW_CALLED: AtomicBool = AtomicBool::new(false);
use iced::widget::canvas::{self, Canvas, Frame, Path, Stroke, LineCap};
use iced::{Length, Point, Rectangle, Size};
use data::data_format::Candlestick;

fn view_fn<'a>(state: &'a MockApp, _window_id: iced_core::window::Id) -> Element<'a, Message> {
    // window id is unused in this simple mock view
    state.view()
}

pub fn run_mock() {
    log::info!("starting mock UI");

    // spawn a short watchdog to check whether the view was ever called (helps diagnose
    // renderer / windowing delays). If view isn't called within 2 seconds, log a warning.
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        if !VIEW_CALLED.load(Ordering::SeqCst) {
            log::warn!("MockApp::view has not been called after 2s — the renderer or windowing backend may be delayed or blocked");
        }
    });

    let _ = daemon(MockApp::new, MockApp::update, view_fn)
        .settings(Settings { antialiasing: true, ..Settings::default() })
        .title("FlowSurface Mock")
        .run();

    log::info!("mock UI finished");
}

struct MockApp {
    data: Vec<Candlestick>,
}

#[derive(Debug, Clone)]
enum Message {}

impl MockApp {
    fn new() -> (Self, iced::Task<Message>) {
        log::info!("MockApp::new called");

        // Explicitly open a window for the mock UI
        let config = crate::window::Settings {
            size: crate::window::default_size(),
            position: crate::window::Position::Centered,
            exit_on_close_request: true,
            ..crate::window::settings()
        };

        let (_id, open_task) = crate::window::open(config);

        (Self { data: make_mock_data() }, open_task.discard())
    }

    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        log::info!("MockApp::view called");
        VIEW_CALLED.store(true, Ordering::SeqCst);
        let canvas = Canvas::new(ChartCanvas { data: &self.data })
            .width(Length::Fill)
            .height(Length::Fill);

        canvas.into()
    }
}

struct ChartCanvas<'a> {
    data: &'a [Candlestick],
}

impl<'a> canvas::Program<Message> for ChartCanvas<'a> {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &iced::Renderer, _theme: &Theme, bounds: Rectangle, _cursor: iced_core::mouse::Cursor) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        if self.data.is_empty() {
            log::info!("ChartCanvas::draw called with n=0");
            return vec![frame.into_geometry()];
        }

        log::info!("ChartCanvas::draw called with n={}", self.data.len());

        // Compute min/max price
        let min_price = self.data.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
        let max_price = self.data.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);

        let w = bounds.width;
        let h = bounds.height;

        let n = self.data.len() as f32;
        for (i, c) in self.data.iter().enumerate() {
            let x = (i as f32 + 0.5) * (w / n);
            let open = c.open;
            let close = c.close;
            let high = c.high;
            let low = c.low;
            let y_of = |price: f64| {
                let p = (price - min_price) / (max_price - min_price);
                // invert y
                h as f32 * (1.0 - p as f32)
            };

            let top = y_of(high);
            let bottom = y_of(low);
            let o = y_of(open);
            let cl = y_of(close);

            // wick
            let wick = Path::line(Point::new(x, top), Point::new(x, bottom));
            frame.stroke(&wick, Stroke::with_color(Stroke { width: 1.0, line_cap: LineCap::Round, ..Stroke::default() }, iced::Color::WHITE));

            // body
            let bw = (w / n) * 0.6;
            let left = x - bw / 2.0;
            let _right = x + bw / 2.0;
            let top_body = o.min(cl);
            let bottom_body = o.max(cl);

            let rect = Path::rectangle(Point::new(left, top_body), Size::new(bw, bottom_body - top_body));
            let color = if close >= open { iced::Color::from_rgb(0.0, 0.8, 0.0) } else { iced::Color::from_rgb(0.8, 0.0, 0.0) };
            frame.fill(&rect, color);
            frame.stroke(&rect, Stroke::with_color(Stroke { width: 1.0, ..Stroke::default() }, iced::Color::BLACK));
        }

        vec![frame.into_geometry()]
    }
}

fn make_mock_data() -> Vec<Candlestick> {
    let mut result = Vec::new();
    let mut t = 1_672_444_800i64;
    let mut base = 150.0;

    for _ in 0..60 {
        let open = base + (rand::random::<f64>() - 0.5) * 2.0;
        let close = base + (rand::random::<f64>() - 0.5) * 2.0;
        let high = open.max(close) + rand::random::<f64>() * 1.0;
        let low = open.min(close) - rand::random::<f64>() * 1.0;
        let volume = (rand::random::<f64>() * 1e5) as f64;

        result.push(Candlestick { timestamp: t, open, high, low, close, volume });
        t += 60;
        base = close;
    }

    result
}
