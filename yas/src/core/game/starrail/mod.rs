use super::*;
mod const_info;
mod relic;
mod scanner;
pub use const_info::get_window_info;
pub use relic::*;
pub use scanner::*;

#[derive(Clone, Debug)]
pub struct StarRailScanInfo<T = ScanInfoType> {
    pub shared: SharedScanInfo<T>,

    pub sub_stat_name_pos: [RectBound<T>; 4],
    pub sub_stat_value_pos: [RectBound<T>; 4],
}

pub type StarRailWindowInfo = StarRailScanInfo<WindowInfoType>;

impl ConvertToScanInfo<StarRailScanInfo> for StarRailWindowInfo {
    fn to_scan_info(&self, size: Size<f64>) -> StarRailScanInfo {
        let radio = self.shared.get_radio(size);

        StarRailScanInfo {
            shared: self.shared.to_scan_info(size),

            sub_stat_name_pos: [
                self.sub_stat_name_pos[0].scale_to_scan(radio),
                self.sub_stat_name_pos[1].scale_to_scan(radio),
                self.sub_stat_name_pos[2].scale_to_scan(radio),
                self.sub_stat_name_pos[3].scale_to_scan(radio),
            ],

            sub_stat_value_pos: [
                self.sub_stat_value_pos[0].scale_to_scan(radio),
                self.sub_stat_value_pos[1].scale_to_scan(radio),
                self.sub_stat_value_pos[2].scale_to_scan(radio),
                self.sub_stat_value_pos[3].scale_to_scan(radio),
            ],
        }
    }
}

impl DrawConfig for StarRailScanInfo {
    fn draw_config(&self, image: &mut image::RgbImage) {
        self.shared.draw_config(image);

        for pos in self.sub_stat_name_pos.iter() {
            pos.draw_config(image);
        }

        for pos in self.sub_stat_value_pos.iter() {
            pos.draw_config(image);
        }
    }
}
