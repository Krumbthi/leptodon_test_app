use crate::models::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUser) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at"
        )
        .bind(new_user.email)
        .bind(new_user.password_hash)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
}

pub struct ExerciseRepository {
    pool: PgPool,
}

impl ExerciseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_exercise: NewExercise) -> Result<Exercise, sqlx::Error> {
        let exercise = sqlx::query_as::<_, Exercise>(
            "INSERT INTO exercises (name, muscle_group, equipment, created_by) VALUES ($1, $2, $3, $4) RETURNING id, name, muscle_group, equipment, created_by, created_at"
        )
        .bind(new_exercise.name)
        .bind(new_exercise.muscle_group)
        .bind(new_exercise.equipment)
        .bind(new_exercise.created_by)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(exercise)
    }

    pub async fn find_all(&self) -> Result<Vec<Exercise>, sqlx::Error> {
        let exercises = sqlx::query_as::<_, Exercise>(
            "SELECT id, name, muscle_group, equipment, created_by, created_at FROM exercises ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(exercises)
    }

    pub async fn find_by_muscle_group(&self, muscle_group: &str) -> Result<Vec<Exercise>, sqlx::Error> {
        let exercises = sqlx::query_as::<_, Exercise>(
            "SELECT id, name, muscle_group, equipment, created_by, created_at FROM exercises WHERE muscle_group = $1 ORDER BY name"
        )
        .bind(muscle_group)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(exercises)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Exercise>, sqlx::Error> {
        let exercise = sqlx::query_as::<_, Exercise>(
            "SELECT id, name, muscle_group, equipment, created_by, created_at FROM exercises WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(exercise)
    }
}

pub struct WorkoutSessionRepository {
    pool: PgPool,
}

impl WorkoutSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_session: NewWorkoutSession) -> Result<WorkoutSession, sqlx::Error> {
        let session = sqlx::query_as::<_, WorkoutSession>(
            "INSERT INTO workout_sessions (user_id, date, notes) VALUES ($1, $2, $3) RETURNING id, user_id, date, notes, created_at"
        )
        .bind(new_session.user_id)
        .bind(new_session.date)
        .bind(new_session.notes)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(session)
    }

    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<WorkoutSession>, sqlx::Error> {
        let sessions = sqlx::query_as::<_, WorkoutSession>(
            "SELECT id, user_id, date, notes, created_at FROM workout_sessions WHERE user_id = $1 ORDER BY date DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(sessions)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkoutSession>, sqlx::Error> {
        let session = sqlx::query_as::<_, WorkoutSession>(
            "SELECT id, user_id, date, notes, created_at FROM workout_sessions WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(session)
    }

    pub async fn find_with_exercises(&self, id: Uuid) -> Result<Option<WorkoutSessionWithExercises>, sqlx::Error> {
        // First get the session
        let session = sqlx::query_as::<_, WorkoutSession>(
            "SELECT id, user_id, date, notes, created_at FROM workout_sessions WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let session = match session {
            Some(s) => s,
            None => return Ok(None),
        };

        // Get all exercises for this session with their sets
        let rows = sqlx::query(
            "SELECT 
                e.id as exercise_id, e.name, e.muscle_group, e.equipment, e.created_by, e.created_at,
                es.id as set_id, es.session_id, es.exercise_id as set_exercise_id, es.set_number, es.reps, es.weight, es.created_at as set_created_at
             FROM exercises e
             INNER JOIN exercise_sets es ON e.id = es.exercise_id
             WHERE es.session_id = $1
             ORDER BY e.name, es.set_number"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let mut exercises_map: std::collections::HashMap<Uuid, ExerciseWithSets> = std::collections::HashMap::new();

        for row in rows {
            let exercise_id: Uuid = row.get("exercise_id");
            let set_id: Uuid = row.get("set_id");
            let set_number: i32 = row.get("set_number");
            let reps: i32 = row.get("reps");
            let weight: Option<f32> = row.get("weight");
            let set_created_at: chrono::DateTime<chrono::Utc> = row.get("set_created_at");

            let exercise = exercises_map.entry(exercise_id).or_insert_with(|| ExerciseWithSets {
                exercise: Exercise {
                    id: row.get("exercise_id"),
                    name: row.get("name"),
                    muscle_group: row.get("muscle_group"),
                    equipment: row.get("equipment"),
                    created_by: row.get("created_by"),
                    created_at: row.get("created_at"),
                },
                sets: Vec::new(),
            });

            exercise.sets.push(ExerciseSet {
                id: set_id,
                session_id: row.get("session_id"),
                exercise_id: row.get("set_exercise_id"),
                set_number,
                reps,
                weight,
                created_at: set_created_at,
            });
        }

        let exercises: Vec<ExerciseWithSets> = exercises_map.into_values().collect();

        Ok(Some(WorkoutSessionWithExercises { session, exercises }))
    }
}

pub struct ExerciseSetRepository {
    pool: PgPool,
}

impl ExerciseSetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_set: NewExerciseSet) -> Result<ExerciseSet, sqlx::Error> {
        let set = sqlx::query_as::<_, ExerciseSet>(
            "INSERT INTO exercise_sets (session_id, exercise_id, set_number, reps, weight) VALUES ($1, $2, $3, $4, $5) RETURNING id, session_id, exercise_id, set_number, reps, weight, created_at"
        )
        .bind(new_set.session_id)
        .bind(new_set.exercise_id)
        .bind(new_set.set_number)
        .bind(new_set.reps)
        .bind(new_set.weight)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(set)
    }

    pub async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<ExerciseSet>, sqlx::Error> {
        let sets = sqlx::query_as::<_, ExerciseSet>(
            "SELECT id, session_id, exercise_id, set_number, reps, weight, created_at FROM exercise_sets WHERE session_id = $1 ORDER BY exercise_id, set_number"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(sets)
    }

    pub async fn find_by_session_and_exercise(&self, session_id: Uuid, exercise_id: Uuid) -> Result<Vec<ExerciseSet>, sqlx::Error> {
        let sets = sqlx::query_as::<_, ExerciseSet>(
            "SELECT id, session_id, exercise_id, set_number, reps, weight, created_at FROM exercise_sets WHERE session_id = $1 AND exercise_id = $2 ORDER BY set_number"
        )
        .bind(session_id)
        .bind(exercise_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(sets)
    }

    pub async fn update(&self, id: Uuid, reps: i32, weight: Option<f32>) -> Result<ExerciseSet, sqlx::Error> {
        let set = sqlx::query_as::<_, ExerciseSet>(
            "UPDATE exercise_sets SET reps = $1, weight = $2 WHERE id = $3 RETURNING id, session_id, exercise_id, set_number, reps, weight, created_at"
        )
        .bind(reps)
        .bind(weight)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(set)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM exercise_sets WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}