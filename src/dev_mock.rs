use iced::{Application, executor, Command, Element, Settings, Theme};
use iced::widget::canvas::{self, Canvas, Frame, Path, Stroke};
use iced::{Length, Point, Rectangle, Size};
use data::data_format::Candlestick;

pub fn run_mock() -> iced::Result<()> {
    MockApp::run(Settings {
        window: iced::window::Settings {
            size: (900, 600),
            ..Default::default()
        },
        ..Settings::default()
    })
}

struct MockApp {
    data: Vec<Candlestick>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for MockApp {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { data: make_mock_data() }, Command::none())
    }

    fn title(&self) -> String {
        "Flowsurface Mock Chart".into()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(ChartCanvas { data: &self.data })
            .width(Length::Fill)
            .height(Length::Fill);

        canvas.into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

struct ChartCanvas<'a> {
    data: &'a [Candlestick],
}

impl<'a> canvas::Program<Message> for ChartCanvas<'a> {
    type State = ();

    fn draw(&self, _state: &Self::State, renderer: &iced::Renderer, theme: &Theme, bounds: Rectangle, _cursor: iced_core::mouse::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(bounds.size());

        if self.data.is_empty() {
            return vec![frame.into_geometry()];
        }

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
            frame.stroke(&wick, Stroke { width: 1.0, color: iced::Color::WHITE, line_cap: iced::widget::canvas::LineCap::Round, ..Stroke::default() });

            // body
            let bw = (w / n) * 0.6;
            let left = x - bw / 2.0;
            let right = x + bw / 2.0;
            let top_body = o.min(cl);
            let bottom_body = o.max(cl);

            let rect = Path::rectangle(Point::new(left, top_body), Size::new(bw, bottom_body - top_body));
            let color = if close >= open { iced::Color::from_rgb(0.0, 0.8, 0.0) } else { iced::Color::from_rgb(0.8, 0.0, 0.0) };
            frame.fill(&rect, color);
            frame.stroke(&rect, Stroke { width: 1.0, color: iced::Color::BLACK, ..Stroke::default() });
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
