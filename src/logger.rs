use google_sheets4::api::ValueRange;
use google_sheets4::{hyper, hyper_rustls, oauth2, Sheets};
use oauth2::read_service_account_key;
use chrono::Utc;
use anyhow::Result;

pub struct SheetLogger {
    sheet_id: String,
    hub: Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl SheetLogger {
    pub async fn new(credentials_path: &str, sheet_id: &str) -> Result<Self> {
        let secret = read_service_account_key(credentials_path).await?;
        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await?;
        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Ok(SheetLogger {
            sheet_id: sheet_id.to_string(),
            hub,
        })
    }

    pub async fn append_row(&self, row_data: Vec<String>) -> Result<()> {
        let range = "Logs!A:F";
        let value_range = ValueRange {
            values: Some(vec![row_data]),
            ..Default::default()
        };

        self.hub
            .spreadsheets()
            .values_append(value_range, &self.sheet_id, range)
            .value_input_option("USER_ENTERED")
            .doit()
            .await?;

        Ok(())
    }

    pub async fn log_trade(
        &self,
        market_id: &str,
        side: &str,
        price: f64,
        size: f64,
        status: &str,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let row = vec![
            now,
            market_id.to_string(),
            side.to_string(),
            price.to_string(),
            size.to_string(),
            status.to_string(),
        ];
        self.append_row(row).await
    }
}
