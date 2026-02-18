use crate::core::exercise::exercise_dto::{
    ExerciseLibraryFilterReq, ExerciseLibraryReq, ExerciseLibraryRes, ValidExercise,
};
use crate::core::exercise::exercise_repo::ExerciseRepo;
use crate::db::pagination_support::{PaginationParams, PaginationRes};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct ExerciseService {
    pool: Pool<Sqlite>,
    repo: ExerciseRepo,
}

impl ExerciseService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            repo: ExerciseRepo::new(),
        }
    }

    pub async fn create(&self, req: ExerciseLibraryReq) -> Result<u32, String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };

        let res = self.repo.create_exercise(&mut conn, req).await;
        conn.commit().await.unwrap();
        res
    }

    pub async fn update(&self, valid_exercise: ValidExercise) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };

        self.repo.update_exercise(&mut conn, valid_exercise).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn delete(&self, exercise_id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };

        self.repo.delete_exercise(&mut conn, exercise_id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn get_one(&self, exercise_id: u32) -> Result<ValidExercise, String> {
        self.repo.get_one_exercise(&self.pool, exercise_id).await
    }

    pub async fn paginate(
        &self,
        filter_req: Option<ExerciseLibraryFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<ExerciseLibraryRes>, String> {
        self.repo
            .paginate_exercises(&self.pool, filter_req, pagination_params)
            .await
    }
}
