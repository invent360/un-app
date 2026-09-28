use crate::state::ChartDataPoint;

/// Chart dimensions and padding
pub struct ChartDimensions {
    pub width: f64,
    pub height: f64,
    pub padding_left: f64,
    pub padding_right: f64,
    pub padding_top: f64,
    pub padding_bottom: f64,
}

impl Default for ChartDimensions {
    fn default() -> Self {
        Self {
            width: 320.0,
            height: 160.0,
            padding_left: 10.0,
            padding_right: 10.0,
            padding_top: 10.0,
            padding_bottom: 25.0,
        }
    }
}

impl ChartDimensions {
    pub fn inner_width(&self) -> f64 {
        self.width - self.padding_left - self.padding_right
    }

    pub fn inner_height(&self) -> f64 {
        self.height - self.padding_top - self.padding_bottom
    }
}

/// Calculate the min and max values from data points
pub fn get_value_range(data: &[ChartDataPoint]) -> (f64, f64) {
    if data.is_empty() {
        return (0.0, 100.0);
    }

    let min = data.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
    let max = data.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);

    // Add some padding to the range
    let padding = (max - min) * 0.1;
    (0.0_f64.max(min - padding), max + padding)
}

/// Generate SVG path for a line chart
pub fn generate_line_path(data: &[ChartDataPoint], dims: &ChartDimensions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let (min_val, max_val) = get_value_range(data);
    let val_range = max_val - min_val;
    let inner_width = dims.inner_width();
    let inner_height = dims.inner_height();

    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let x = dims.padding_left + (i as f64 / (data.len() - 1).max(1) as f64) * inner_width;
            let y = dims.padding_top + inner_height - ((point.value - min_val) / val_range) * inner_height;
            (x, y)
        })
        .collect();

    let mut path = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", x, y));
        } else {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }

    path
}

/// Generate SVG path for a smooth curve (using quadratic bezier)
pub fn generate_smooth_line_path(data: &[ChartDataPoint], dims: &ChartDimensions) -> String {
    if data.len() < 2 {
        return generate_line_path(data, dims);
    }

    let (min_val, max_val) = get_value_range(data);
    let val_range = max_val - min_val;
    let inner_width = dims.inner_width();
    let inner_height = dims.inner_height();

    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let x = dims.padding_left + (i as f64 / (data.len() - 1).max(1) as f64) * inner_width;
            let y = dims.padding_top + inner_height - ((point.value - min_val) / val_range) * inner_height;
            (x, y)
        })
        .collect();

    let mut path = format!("M {:.1} {:.1}", points[0].0, points[0].1);

    for i in 1..points.len() {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        let cp_x = (x0 + x1) / 2.0;
        path.push_str(&format!(" Q {:.1} {:.1} {:.1} {:.1}", cp_x, y0, cp_x, y1));
        path.push_str(&format!(" L {:.1} {:.1}", x1, y1));
    }

    path
}

/// Generate SVG path for an area chart (closed path)
pub fn generate_area_path(data: &[ChartDataPoint], dims: &ChartDimensions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let line_path = generate_smooth_line_path(data, dims);
    let baseline_y = dims.height - dims.padding_bottom;
    let start_x = dims.padding_left;
    let end_x = dims.padding_left + dims.inner_width();

    format!(
        "{} L {:.1} {:.1} L {:.1} {:.1} Z",
        line_path,
        end_x, baseline_y,
        start_x, baseline_y
    )
}

/// Calculate bar positions and dimensions
pub fn calculate_bar_layout(
    data: &[ChartDataPoint],
    dims: &ChartDimensions,
    bar_gap: f64,
) -> Vec<(f64, f64, f64, f64)> {
    // Returns (x, y, width, height) for each bar
    if data.is_empty() {
        return vec![];
    }

    let (_, max_val) = get_value_range(data);
    let inner_width = dims.inner_width();
    let inner_height = dims.inner_height();
    let bar_count = data.len() as f64;
    let total_gaps = (bar_count - 1.0) * bar_gap;
    let bar_width = (inner_width - total_gaps) / bar_count;

    data.iter()
        .enumerate()
        .map(|(i, point)| {
            let x = dims.padding_left + i as f64 * (bar_width + bar_gap);
            let bar_height = (point.value / max_val) * inner_height;
            let y = dims.padding_top + inner_height - bar_height;
            (x, y, bar_width, bar_height)
        })
        .collect()
}

/// Get x positions for axis labels
pub fn get_label_positions(data: &[ChartDataPoint], dims: &ChartDimensions) -> Vec<(f64, String)> {
    let inner_width = dims.inner_width();

    data.iter()
        .enumerate()
        .map(|(i, point)| {
            let x = dims.padding_left + (i as f64 / (data.len() - 1).max(1) as f64) * inner_width;
            (x, point.label.clone())
        })
        .collect()
}

/// Get x positions for bar chart labels (centered under bars)
pub fn get_bar_label_positions(
    data: &[ChartDataPoint],
    dims: &ChartDimensions,
    bar_gap: f64,
) -> Vec<(f64, String)> {
    let inner_width = dims.inner_width();
    let bar_count = data.len() as f64;
    let total_gaps = (bar_count - 1.0) * bar_gap;
    let bar_width = (inner_width - total_gaps) / bar_count;

    data.iter()
        .enumerate()
        .map(|(i, point)| {
            let x = dims.padding_left + i as f64 * (bar_width + bar_gap) + bar_width / 2.0;
            (x, point.label.clone())
        })
        .collect()
}
