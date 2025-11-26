use egui::{Color32, CornerRadius, Rect, Response, Sense, Ui, Widget, vec2};
use qrcode::types::QrError;
use qrcode::{Color, QrCode};

enum CodeSrc<'a> {
    Ref(&'a QrCode),
    Owned(QrCode),
}

pub struct QrCodeWidget<'a> {
    code: CodeSrc<'a>,
    quiet_zone: f32,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(code: &'a QrCode) -> Self {
        QrCodeWidget {
            code: CodeSrc::Ref(code),
            quiet_zone: 1.,
        }
    }

    pub fn from_data(data: &[u8]) -> Result<Self, QrError> {
        let code = QrCode::new(data)?;
        Ok(QrCodeWidget {
            code: CodeSrc::Owned(code),
            quiet_zone: 1.,
        })
    }

    pub fn quiet_zone(mut self, quiet_zone: f32) -> Self {
        self.quiet_zone = quiet_zone;
        self
    }
}
impl QrCodeWidget<'_> {
    pub fn show(self, ui: &mut Ui) -> Response {
        self.ui(ui)
    }
    
    /// Calculate the desired size for layout purposes
    fn desired_size(&self, ui: &Ui) -> egui::Vec2 {
        let code_ref = match &self.code {
            CodeSrc::Ref(code) => code,
            CodeSrc::Owned(q) => q,
        };
        let w = code_ref.width() as f32;
        let total_size = w + (self.quiet_zone * 2.0);
        vec2(total_size, total_size) * ui.spacing().interact_size.y // Scale with UI
    }
}

impl Widget for QrCodeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = self.desired_size(ui);
        let response = ui.allocate_response(desired_size, Sense::click());
        
        if ui.is_rect_visible(response.rect) {
            self.paint_at(ui, response.rect);
        }
        
        response
    }
}

impl QrCodeWidget<'_> {
    /// Paint the QR code at a specific rectangle
    fn paint_at(&self, ui: &Ui, rect: Rect) {
        let painter = ui.painter();
        let code_ref = match &self.code {
            CodeSrc::Ref(code) => code,
            CodeSrc::Owned(q) => q,
        };
        
        let w = code_ref.width();
        let total_size = w as f32 + (self.quiet_zone * 2.0);
        let cell_size = rect.width() / total_size;
        
        // Fill background
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::WHITE);
        
        let quiet_zone_offset = self.quiet_zone * cell_size;
        
        for (i, color) in code_ref.to_colors().iter().enumerate() {
            if matches!(color, Color::Dark) {
                let row = i / w;
                let col = i % w;
                
                let pos = rect.left_top() 
                    + vec2(quiet_zone_offset, quiet_zone_offset)
                    + vec2(col as f32 * cell_size, row as f32 * cell_size);
                
                painter.rect_filled(
                    Rect::from_min_size(pos, vec2(cell_size, cell_size)),
                    CornerRadius::ZERO,
                    Color32::BLACK,
                );
            }
        }
    }
}