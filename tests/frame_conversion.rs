use scap_vc::frame::{convert_bgra_to_rgb, get_cropped_data, remove_alpha_channel};

#[test]
fn public_channel_conversions_preserve_pixel_order() {
    let bgra = vec![10, 20, 30, 255, 40, 50, 60, 128];

    assert_eq!(
        remove_alpha_channel(bgra.clone()),
        vec![10, 20, 30, 40, 50, 60]
    );
    assert_eq!(convert_bgra_to_rgb(bgra), vec![30, 20, 10, 60, 50, 40]);
}

#[test]
fn public_crop_keeps_leftmost_pixels_from_each_row() {
    let data: Vec<u8> = (0..24).collect();

    assert_eq!(
        get_cropped_data(data, 3, 2, 2),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 15, 16, 17, 18, 19],
    );
}

#[test]
fn invalid_crop_dimensions_leave_the_buffer_unchanged() {
    let data = vec![1, 2, 3, 4];
    assert_eq!(get_cropped_data(data.clone(), 2, 2, 1), data);
}
