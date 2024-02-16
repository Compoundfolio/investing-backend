use diesel::{insert_into, prelude::*};
use uuid::Uuid;

use crate::database::{schema, CommonRepository, RepositoryError};

use super::model::InsertReportUpload;

impl CommonRepository {

    pub fn create_report_upload(&self, report_upload: InsertReportUpload) -> Result<Uuid, RepositoryError> {
        use schema::report_upload::dsl;
        let report_upload_id = insert_into(dsl::report_upload)
            .values(report_upload)
            .returning(dsl::id)
            .get_result::<Uuid>(&mut self.pool.get()?)?;
        Ok(report_upload_id)
    }

}
