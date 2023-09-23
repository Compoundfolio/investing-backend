use diesel::{insert_into, prelude::*};

use crate::database::{schema, CommonRepository, RepositoryError};

use super::model::{LoginMethodType, AppUser, AppUserLoginMethod, InsertLoginMethod, InsertAppUser};

impl CommonRepository {
    pub fn find_user_by_login_method(
        &self,
        _method_type: LoginMethodType,
        sub: &str,
    ) -> Result<Option<AppUser>, RepositoryError> {
        Ok(schema::app_user::table
            .inner_join(schema::app_user_login_method::table)
            .filter(schema::app_user_login_method::subject_id.eq(sub))
            .select(AppUser::as_select())
            .first::<AppUser>(&mut self.pool.get()?)
            .optional()?)
    }

    pub fn find_login_method(
        &self,
        user_id: &uuid::Uuid,
        _method_type: LoginMethodType,
    ) -> Result<Option<AppUserLoginMethod>, RepositoryError> {
        Ok(schema::app_user_login_method::table
            .filter(schema::app_user_login_method::app_user_id.eq(user_id))
            .select(AppUserLoginMethod::as_select())
            .first::<AppUserLoginMethod>(&mut self.pool.get()?)
            .optional()?)
    }

    pub fn find_user_by_email(&self, email: &str) -> Result<Option<AppUser>, RepositoryError> {
        Ok(schema::app_user::table
            .filter(schema::app_user::email.eq(email))
            .select(AppUser::as_select())
            .first::<AppUser>(&mut self.pool.get()?)
            .optional()?)
    }

    pub fn create_user(&self, user: &InsertAppUser) -> Result<AppUser, RepositoryError> {
        Ok(insert_into(schema::app_user::dsl::app_user)
            .values(user)
            .get_result(&mut self.pool.get()?)?)
    }

    pub fn create_login_method(
        &self,
        login_method: &InsertLoginMethod,
    ) -> Result<AppUserLoginMethod, RepositoryError> {
        Ok(
            insert_into(schema::app_user_login_method::dsl::app_user_login_method)
                .values(login_method)
                .get_result(&mut self.pool.get()?)?,
        )
    }
}
