use opencv::core::Scalar;
use opencv::core::Mat;
use opencv::types::VectorOfMat;
use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, Display)]
pub enum Filter {
    None,
    Grayscale,
    Blur(u16),
    Sharpen(u16),
    Saturate(i32),
}

macro_rules! prompt_filter_strength {
    ($enum_val:ident::$variant:ident) => {{
        println!("Enter {} strength:", $enum_val::$variant(0));
        $enum_val::$variant(input::input_valid())
    }};
}

pub(crate) use prompt_filter_strength;

impl Filter {
    pub fn grayscale(img: &Mat) -> Result<Mat, opencv::Error> {
        let mut out = Mat::default();

        opencv::imgproc::cvt_color(img, &mut out, opencv::imgproc::COLOR_BGR2GRAY, 0)?;

        Ok(out)
    }

    pub fn blur(img: &Mat, mut strength: u16) -> Result<Mat, opencv::Error> {
        let mut out = Mat::default();

        strength += if strength % 2 == 0 { 1 } else { 0 };

        opencv::imgproc::gaussian_blur(
            img,
            &mut out,
            opencv::core::Size_::new(strength.into(), strength.into()),
            0.0,
            0.0,
            opencv::core::BORDER_DEFAULT,
        )?;

        Ok(out)
    }

    pub fn sharpen(img: &Mat, mut strength: u16) -> Result<Mat, opencv::Error> {
        let mut out = Mat::default();

        strength += if strength % 2 == 0 { 1 } else { 0 };

        opencv::imgproc::gaussian_blur(
            img,
            &mut out,
            opencv::core::Size_::new(0, 0),
            strength.into(),
            0.0,
            opencv::core::BORDER_DEFAULT,
        )?;
        opencv::core::add_weighted(img, 1.5, &out.clone(), -0.5, 0.0, &mut out, -1)?;

        Ok(out)
    }

    pub fn saturate(img: &Mat, saturation_value: i32) -> Result<Mat, opencv::Error> {
        let mut hsv_image = Mat::default();
        opencv::imgproc::cvt_color(img, &mut hsv_image, opencv::imgproc::COLOR_BGR2HSV, 0)?;

        let mut channels = VectorOfMat::new();
        opencv::core::split(&hsv_image, &mut channels)?;

        let mut new_saturation = Mat::default();
        opencv::core::add(
            &channels.get(1)?,
            &Scalar::all(saturation_value.into()),
            &mut new_saturation,
            &opencv::core::no_array(),
            -1,
        )?;

        channels.set(1, new_saturation)?;

        opencv::core::merge(&channels, &mut hsv_image)?;

        let mut out = Mat::default();

        opencv::imgproc::cvt_color(&hsv_image, &mut out, opencv::imgproc::COLOR_HSV2BGR, 0)?;

        Ok(out)
    }
}
