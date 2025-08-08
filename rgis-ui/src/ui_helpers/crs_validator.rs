use rgis_geodesy::GeodesyContext;
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Geodesy(#[from] rgis_geodesy::Error),
    #[error("RwLock poisoned")]
    RwLock,
}

pub fn parse_epsg_input_value(
    geodesy_ctx: &GeodesyContext,
    input: &str,
    parsed_text_field_value: &mut Option<u16>,
) -> Result<(geodesy::OpHandle, u16), Error> {
    let mut geodesy_ctx = geodesy_ctx.0.write().map_err(|_| Error::RwLock)?;
    let parsed = u16::from_str(input);
    *parsed_text_field_value = parsed.as_ref().ok().copied();
    let parsed = parsed?;
    let outcome = rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, parsed)
        .map_err(Error::Geodesy)?;
    Ok((outcome, parsed))
}
