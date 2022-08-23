use super::prelude::*;
use crate::args::Args;
use std::{
    path::PathBuf,
    str::FromStr,
};

#[instrument]
pub async fn get_token(args: &Args) -> Result<String, Box<dyn Error>> {
    let path;

    if let Some(token) = args.token.clone() {
        info!("retrieved token via argument");
        return Ok(token);
    } else if let Some(token_path) = args.token_path.clone() {
        info!("retrieving token via file '{}'", token_path.display());
        path = token_path
    } else {
        let _ = error_span!("e").enter();
        info!("retrieving token via default file './token'");
        path = PathBuf::from_str("./token")?;
    }

    tokio::fs::read_to_string(path).await.map_err(|e| e.into())
}
