use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub muscle_group: String,
    pub equipment: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkoutSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExerciseSet {
    pub id: Uuid,
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub set_number: i32,
    pub reps: i32,
    pub weight: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExercise {
    pub name: String,
    pub muscle_group: String,
    pub equipment: String,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkoutSession {
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExerciseSet {
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub set_number: i32,
    pub reps: i32,
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSessionWithExercises {
    pub session: WorkoutSession,
    pub exercises: Vec<ExerciseWithSets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseWithSets {
    pub exercise: Exercise,
    pub sets: Vec<ExerciseSet>,
}