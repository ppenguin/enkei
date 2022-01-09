use cairo::{
    ImageSurface,
    Rectangle,
};

use super::error::ImageError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Filter {
    Fast,
    Good,
    Best,
    Nearest,
    Bilinear,
    Gaussian,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Scaling {
    Fill,
    Fit,
    None,
}

impl Scaling {
    fn scale(
        &self,
        sur: &ImageSurface,
        geometry: &Rectangle,
        filter: Filter,
    ) -> Result<Vec<u8>, ImageError> {
        match self {
            Scaling::Fill => Scaling::fill(sur, geometry, filter),
            Scaling::Fit => Scaling::fit(sur, geometry, filter),
            Scaling::None => Scaling::none(sur, geometry),
        }
    }

    fn none(buf: &ImageSurface, geometry: &Rectangle) -> Result<Vec<u8>, ImageError> {
        let pad_width = (geometry.width - buf.width() as f64) / 2.0;
        let pad_height = (geometry.height - buf.height() as f64) / 2.0;

        {
            let target =
                cairo::ImageSurface::create(cairo::Format::ARgb32, geometry.width as i32, geometry.height as i32)
                    .map_err(|e| ImageError::CouldNotCreateSurface(e))?;
            let ctx = cairo::Context::new(&target).map_err(|e| ImageError::CouldNotCreateContext(e))?;
            ctx.set_source_surface(buf, pad_width, pad_height).map_err(|e| ImageError::CouldNotSetSource(e))?;
            ctx.paint().map_err(|e| ImageError::CouldNotWriteResult(e))?;

            Ok(target.take_data().map_err(|e| ImageError::CouldNotGetData(e))?.to_vec())
        }
    }

    fn fit(
        buf: &ImageSurface,
        geometry: &Rectangle,
        filter: Filter,
    ) -> Result<Vec<u8>, ImageError> {
        Scaling::fill_or_fit(buf, geometry, filter, f64::min)
    }

    fn fill(
        buf: &ImageSurface,
        geometry: &Rectangle,
        filter: Filter,
    ) -> Result<Vec<u8>, ImageError> {
        Scaling::fill_or_fit(buf, geometry, filter, f64::max)
    }

    fn fill_or_fit<F: Fn(f64, f64) -> f64>(
        buf: &ImageSurface,
        geometry: &Rectangle,
        filter: Filter,
        comp: F,
    ) -> Result<Vec<u8>, ImageError> {
        // 1. Crop the image if necessary
        // 2. Scale the image to the proper size

        let height_ratio = geometry.height as f64 / buf.height() as f64;
        let width_ratio = geometry.width as f64 / buf.width() as f64;
        let max_ratio = comp(height_ratio, width_ratio);

        // Get cropping edges (aspect)
        let crop_height = ((buf.height() as f64 * max_ratio) as i32)
            .checked_sub(geometry.height as i32)
            .map(|elem| (elem / 2) as f64 / max_ratio)
            .unwrap_or(0.0)
            .clamp(-geometry.height, geometry.height);
        let crop_width = ((buf.width() as f64 * max_ratio) as i32)
            .checked_sub(geometry.width as i32)
            .map(|elem| (elem / 2) as f64 / max_ratio)
            .unwrap_or(0.0)
            .clamp(-geometry.width, geometry.width);
        // Create context and scale and crop to fit
        {
            let target =
                cairo::ImageSurface::create(cairo::Format::ARgb32, geometry.width as i32, geometry.height as i32)
                    .map_err(|e| ImageError::CouldNotCreateSurface(e))?;
            let ctx = cairo::Context::new(&target).map_err(|e| ImageError::CouldNotCreateContext(e))?;
            ctx.scale(max_ratio, max_ratio);
            ctx.set_source_surface(buf, -crop_width, -crop_height).map_err(|e| ImageError::CouldNotSetSource(e))?;
            ctx.source().set_filter(filter.into());
            ctx.paint().map_err(|e| ImageError::CouldNotWriteResult(e))?;

            Ok(target.take_data().map_err(|e| ImageError::CouldNotGetData(e))?.to_vec())
        }
    }
}

impl From<Filter> for cairo::Filter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Fast => cairo::Filter::Fast,
            Filter::Good => cairo::Filter::Good,
            Filter::Best => cairo::Filter::Best,
            Filter::Nearest => cairo::Filter::Nearest,
            Filter::Bilinear => cairo::Filter::Bilinear,
            Filter::Gaussian => cairo::Filter::Gaussian,
        }
    }
}
