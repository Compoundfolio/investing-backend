use tokio::io::{AsyncRead, AsyncReadExt};


#[derive(Debug, thiserror::Error)]
enum FreedomfinanceReportParsingError {
    #[error(transparent)]
    IO { #[from] source: std::io::Error },
    #[error(transparent)]
    Serde { #[from] source: serde_json::Error },
}

#[allow(unused)]
async fn parse_report<R: AsyncRead + Unpin>(
    mut reader: R,
) -> Result<super::model::Report, FreedomfinanceReportParsingError> {
    let mut buffer_for_entire_file = Vec::new();
    reader.read_to_end(&mut buffer_for_entire_file).await?;
    let parsed: super::model::Report = serde_json::from_slice(&buffer_for_entire_file)?;
    Ok(parsed)
}


#[cfg(test)]
mod test {
    use tokio::fs::File;

    use super::*;

    #[tokio::test]
    async fn parses_report() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/freedomfinance_report.json");
        let file = File::open(d).await.unwrap();
        let report = parse_report(file).await.unwrap();
        assert_eq!(report.trades.detailed.len(), 23);
        assert_eq!(report.cash_flows.detailed.len(), 83);
    }
}
