
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // let teal_str = "#64C09A";
        let rgb = iamge::Rgb([100, 192, 154]);
        let h = HSV::from_rgb(rgb);
        let rgb_out = HSV::to_rgb(h);
        assert_eq!(rgb, rgb_out);
    }
}