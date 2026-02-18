use crate::core::workout::workout_dto::{WorkoutExerciseReq, WorkoutReq};
use crate::core::workout::workout_repo::WorkoutRepo;
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

    pub async fn delete_exercise(&self, id: u32) -> Result<(), String> {
        let mut conn = match self.pool.begin().await {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        self.repo.delete_workout_exercise(&mut conn, id).await?;
        conn.commit().await.unwrap();
        Ok(())
    }

    pub async fn get_one(&self) -> Result<(), String> {
        // TODO
        Ok(())
    }

    pub async fn get_all_exercises_by_workout_id(&self, workout_id: u32) -> Result<(), String> {
        // TODO
        Ok(())
    }

    pub async fn paginate(&self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}
