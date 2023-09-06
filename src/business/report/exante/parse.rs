use super::model::{Report, TradeOperation, Transaction};
use serde::de::DeserializeOwned;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

const EXANTE_REPORT_DELIMETER: char = '\t';

fn split_csv_line_to_fields(line: &str) -> Vec<String> {
    let mut chars = line.trim_start_matches('\u{feff}').chars().peekable();
    let mut current_field = String::new();
    let mut fields = Vec::new();
    let mut inside_quotes = false;
    while let Some(c) = chars.next() {
        match c {
            EXANTE_REPORT_DELIMETER if !inside_quotes => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            '"' => {
                // Check if the next character is also a double quote (escaped quote)
                if chars.peek() == Some(&'"') {
                    current_field.push(c);
                    let _ = chars.next(); // consume the next double quote
                } else {
                    inside_quotes = !inside_quotes;
                }
            }
            _ => current_field.push(c),
        }
    }
    fields.push(current_field.trim().to_string());
    fields
}

fn is_header_suitable<T>(header_fields: &[String]) -> bool
where
    T: DeserializeOwned,
{
    let required_fields = serde_aux::serde_introspection::serde_introspect::<T>();
    for required_field in required_fields.iter() {
        let found_position = header_fields.iter().position(|h| h == required_field);
        if let Some(_position) = found_position {
            // ... we could remove to optimize but thats more memory optimal
        } else {
            return false;
        }
    }
    true
}

enum ReportRecordType {
    Transaction,
    TradeOperation,
}

impl ReportRecordType {
    fn get_suitable_type_for_header(header: &[String]) -> Option<Self> {
        if is_header_suitable::<Transaction>(header) {
            Some(Self::Transaction)
        } else if is_header_suitable::<TradeOperation>(header) {
            Some(Self::TradeOperation)
        }
        // TODO: Elif-elif-elif here
        else {
            None
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ExanteReportParsingError {
    #[error(transparent)]
    IO { #[from] source: std::io::Error },
    #[error("This file starts with an unknown header")]
    UnknownHeader,
    #[error(transparent)]
    Csv { #[from] source: csv::Error }
}

#[allow(unused)]
async fn parse_report<R: AsyncRead + Unpin>(
    reader: R,
) -> Result<Report, ExanteReportParsingError> {
    let original_buf_read = BufReader::new(reader);
    let mut lines = original_buf_read.lines();
    let mut current_record_type = None;

    let mut trade_operations = Vec::new();
    let mut transactions = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let split_line = split_csv_line_to_fields(&line);
        if let Some(record_type) = ReportRecordType::get_suitable_type_for_header(&split_line) {
            let header = csv::StringRecord::from(split_line);
            current_record_type = Some((record_type, header));
        } else {
            let current_record_type = current_record_type
                .as_ref()
                .ok_or(ExanteReportParsingError::UnknownHeader)?;
            let record = csv::StringRecord::from(split_line);
            match current_record_type.0 {
                ReportRecordType::TradeOperation => {
                    let report_item: TradeOperation =
                        record.deserialize(Some(&current_record_type.1))?;
                    trade_operations.push(report_item);
                }
                ReportRecordType::Transaction => {
                    let report_item: Transaction =
                        record.deserialize(Some(&current_record_type.1))?;
                    transactions.push(report_item);
                }
            }
        }
    }

    Ok(Report {
        trade_operations,
        transactions,
    })
}

#[cfg(test)]
mod test {
    use tokio::fs::File;

    use super::*;

    #[tokio::test]
    async fn parses_empty_report() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/exante_empty_report.csv");
        let file = File::open(d).await.unwrap();

        let report = parse_report(file).await.unwrap();
        assert!(report.trade_operations.is_empty());
        assert!(report.transactions.is_empty());
    }

    #[tokio::test]
    async fn parses_small_report() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/exante_small_report.csv");
        let file = File::open(d).await.unwrap();

        let report = parse_report(file).await.unwrap();
        assert_eq!(report.trade_operations.len(), 25);
        assert_eq!(report.transactions.len(), 131);
    }
}
