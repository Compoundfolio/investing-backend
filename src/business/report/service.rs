use super::model::{BrokerType, AbstractReport, AbstractReportParseError};

pub async fn parse_report<R: tokio::io::AsyncRead + Unpin>(broker: BrokerType, reader: R) -> Result<AbstractReport, AbstractReportParseError> {
    let result: Result<AbstractReport, AbstractReportParseError> = match broker {
        BrokerType::Exante => super::exante::parse::parse_report(reader)
            .await
            .map(|ok| ok.into())
            .map_err(|err| err.into()),
        BrokerType::Freedomfinance => super::freedomfinance::parse::parse_report(reader)
            .await
            .map(|ok| ok.into())
            .map_err(|err| err.into()),
    };
    result
}
