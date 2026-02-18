use crate::core::workout::workout_dto::{
    WorkoutExerciseReq, WorkoutExerciseRes, WorkoutReq, WorkoutRes, WorkoutsFilterReq,
};
use crate::core::workout::workout_repo::WorkoutRepo;
use crate::db::pagination_support::{PaginationParams, PaginationRes};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct WorkoutService {
    pool: Pool<Sqlite>,
    repo: WorkoutRepo,
}

impl WorkoutService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            repo: WorkoutRepo::new(),
        }
    }

    pub async fn create(
        &self,
        workout_req: WorkoutReq,
        exercises_req: Vec<WorkoutExerciseReq>,
    ) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };

        let workout_id = self.repo.create_workout(&mut conn, workout_req).await?;
        for mut exercise_req in exercises_req {
            exercise_req.workout_id = workout_id;
            // bulk insert can be better here, but for now it's fine
            self.repo
                .create_workout_exercise(&mut conn, exercise_req)
                .await?;
        }
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn update(
        &self,
        id: u32,
        workout_req: WorkoutReq,
        exercises_req: Vec<(u32, WorkoutExerciseReq)>,
    ) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.update_workout(&mut conn, id, workout_req).await?;
        for (exercise_id, mut exercise_req) in exercises_req {
            exercise_req.workout_id = id;
            self.repo
                .update_workout_exercise(&mut conn, exercise_id, exercise_req)
                .await?;
        }

        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn delete(&self, id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.delete_workout(&mut conn, id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn create_exercise(&self, req: WorkoutExerciseReq) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.create_workout_exercise(&mut conn, req).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn update_exercise(&self, id: u32, req: WorkoutExerciseReq) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo
            .update_workout_exercise(&mut conn, id, req)
            .await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn delete_exercise(&self, id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.delete_workout_exercise(&mut conn, id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn get_one(&self, id: u32) -> Result<WorkoutRes, String> {
        Ok(self.repo.get_one_workout(&self.pool, id).await?)
    }

    pub async fn get_one_exercise(&self, id: u32) -> Result<WorkoutExerciseRes, String> {
        Ok(self.repo.get_one_workout_exercise(&self.pool, id).await?)
    }

    pub async fn get_all_exercises_by_workout_id(
        &self,
        workout_id: u32,
    ) -> Result<Vec<WorkoutExerciseRes>, String> {
        Ok(self
            .repo
            .get_workout_exercises_by_workout_id(&self.pool, workout_id)
            .await?)
    }

    pub async fn paginate(
        &self,
        pagination_filters: Option<WorkoutsFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<WorkoutRes>, String> {
        Ok(self
            .repo
            .paginate_workouts(&self.pool, pagination_filters, pagination_params)
            .await?)
    }
}
