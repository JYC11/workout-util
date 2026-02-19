use crate::db::pagination_support::{PaginationParams, PaginationRes};
use crate::workout_log::workout_log_dto::{
    WorkoutLogDetailRes, WorkoutLogFilterReq, WorkoutLogGroupFilterReq, WorkoutLogGroupPageRes,
    WorkoutLogGroupReq, WorkoutLogGroupRes, WorkoutLogReq,
};
use crate::workout_log::workout_log_repo::WorkoutLogRepo;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct WorkoutLogService {
    pool: Pool<Sqlite>,
    repo: WorkoutLogRepo,
}

impl WorkoutLogService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            repo: WorkoutLogRepo::new(),
        }
    }

    pub async fn create_log_group(
        &self,
        req: WorkoutLogGroupReq,
        logs: Vec<WorkoutLogReq>,
    ) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        let log_group_id = self.repo.create_log_group(&mut conn, req).await?;

        for mut log in logs {
            log.workout_log_group_id = log_group_id;
            self.repo.create_log(&mut conn, log).await?;
        }

        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn delete_log_group(&self, id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.delete_log_group(&mut conn, id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn delete_one_log_entry(&self, id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.delete_log(&mut conn, id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn get_log_group(&self, id: u32) -> Result<WorkoutLogGroupRes, String> {
        Ok(self
            .repo
            .get_one_log_group(&mut *self.pool.acquire().await.unwrap(), id)
            .await?)
    }

    pub async fn get_logs_by_workout_log_group_id(
        &self,
        workout_log_group_id: u32,
    ) -> Result<Vec<WorkoutLogDetailRes>, String> {
        Ok(self
            .repo
            .get_logs_by_workout_log_group_id(
                &mut *self.pool.acquire().await.unwrap(),
                workout_log_group_id,
            )
            .await?)
    }

    pub async fn paginate_log_groups(
        &self,
        pagination_filters: Option<WorkoutLogGroupFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<WorkoutLogGroupPageRes>, String> {
        Ok(self
            .repo
            .paginate_workout_log_groups(
                &mut *self.pool.acquire().await.unwrap(),
                pagination_filters,
                pagination_params,
            )
            .await?)
    }
}
