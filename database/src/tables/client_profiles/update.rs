use super::table_struct::{CLIENT_PROFILES_KEYS, CLIENT_PROFILES_TABLE_NAME};
use crate::{db::Db, tables::client_profiles::table_struct::ClientProfile};
use sqlx::{query_as, Transaction};

impl Db {
    pub async fn create_new_profile(
        &self,
        tx: Option<&mut Transaction<'_, sqlx::Postgres>>,
    ) -> Result<ClientProfile, sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {CLIENT_PROFILES_TABLE_NAME} ({CLIENT_PROFILES_KEYS}) VALUES (DEFAULT, DEFAULT) RETURNING {CLIENT_PROFILES_KEYS}"
        );
        let typed_query = query_as::<_, ClientProfile>(&query_body);

        return match tx {
            Some(tx) => typed_query.fetch_one(&mut **tx).await,
            None => typed_query.fetch_one(&self.connection_pool).await,
        };
    }

    pub async fn update_client_profile_merge_lookup(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        old_target_client_profile_id: i64,
        new_target_client_profile_id: i64,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "UPDATE {CLIENT_PROFILES_TABLE_NAME} SET merged_into_client_profile_id = $1 WHERE client_profile_id = $2"
        );

        let query_result = sqlx::query(&query_body)
            .bind(new_target_client_profile_id)
            .bind(old_target_client_profile_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tables::utils::to_microsecond_precision;
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_add_client_profile() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let now = to_microsecond_precision(&Utc::now());
        let created_profile = db.create_new_profile(None).await.unwrap();

        let expected_id = 1;
        let profile_result = db.get_profile_by_profile_id(expected_id).await.unwrap();

        assert_eq!(profile_result, created_profile);
        assert!(profile_result.client_profile_id == expected_id);
        assert!(profile_result.created_at >= now);
    }
}
