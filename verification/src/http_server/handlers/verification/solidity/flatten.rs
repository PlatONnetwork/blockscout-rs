use super::types::{FlattenedSource, VerificationRequest};
use crate::{
    compiler::{download_cache::DownloadCache, version::CompilerVersion},
    http_server::handlers::verification::VerificationResponse,
    solidity::github_fetcher::GithubFetcher,
};
use actix_web::{
    error,
    web::{self, Json},
    Error,
};
use ethers_solc::{CompilerInput, CompilerOutput, Solc};
use std::str::FromStr;

pub async fn compile(
    cache: &DownloadCache<GithubFetcher>,
    compiler_version: &str,
    input: &CompilerInput,
) -> Result<CompilerOutput, Error> {
    let ver = CompilerVersion::from_str(compiler_version).map_err(error::ErrorBadRequest)?;
    let solc_path = cache
        .get(&ver)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let solc = Solc::from(solc_path);
    solc.compile(&input)
        .map_err(error::ErrorInternalServerError)
}

pub async fn verify(
    cache: web::Data<DownloadCache<GithubFetcher>>,
    params: Json<VerificationRequest<FlattenedSource>>,
) -> Result<Json<VerificationResponse>, Error> {
    let params = params.into_inner();

    let input = CompilerInput::try_from(params.content).map_err(error::ErrorBadRequest)?;
    let output = compile(&cache, &params.compiler_version, &input).await?;
    // TODO: verify output
    let _ = output;

    todo!()
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn compile() {}
}
