use crate::database::init_database;
use crate::models::*;
use crate::repositories::*;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let db = init_database().await.unwrap();
        let pool = db.pool();
        
        // Test basic connection
        let result = sqlx::query("SELECT 1 as test")
            .fetch_one(pool)
            .await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_crud() {
        let db = init_database().await.unwrap();
        let user_repo = UserRepository::new(db.pool().clone());
        
        // Create user
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
        };
        
        let user = user_repo.create(new_user).await.unwrap();
        assert_eq!(user.email, "test@example.com");
        
        // Find user by email
        let found_user = user_repo.find_by_email("test@example.com").await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email, "test@example.com");
        
        // Find user by id
        let found_user = user_repo.find_by_id(user.id).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_exercise_crud() {
        let db = init_database().await.unwrap();
        let exercise_repo = ExerciseRepository::new(db.pool().clone());
        
        // Create exercise
        let new_exercise = NewExercise {
            name: "Test Exercise".to_string(),
            muscle_group: "Chest".to_string(),
            equipment: "Barbell".to_string(),
            created_by: None,
        };
        
        let exercise = exercise_repo.create(new_exercise).await.unwrap();
        assert_eq!(exercise.name, "Test Exercise");
        
        // Find all exercises
        let exercises = exercise_repo.find_all().await.unwrap();
        assert!(exercises.len() > 0);
        
        // Find by muscle group
        let chest_exercises = exercise_repo.find_by_muscle_group("Chest").await.unwrap();
        assert!(chest_exercises.len() > 0);
    }

    #[tokio::test]
    async fn test_workout_session_crud() {
        let db = init_database().await.unwrap();
        let session_repo = WorkoutSessionRepository::new(db.pool().clone());
        let user_repo = UserRepository::new(db.pool().clone());
        
        // Create a test user first
        let new_user = NewUser {
            email: "session_test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
        };
        let user = user_repo.create(new_user).await.unwrap();
        
        // Create workout session
        let new_session = NewWorkoutSession {
            user_id: user.id,
            date: chrono::Utc::now(),
            notes: Some("Test workout".to_string()),
        };
        
        let session = session_repo.create(new_session).await.unwrap();
        assert_eq!(session.user_id, user.id);
        
        // Find sessions by user
        let sessions = session_repo.find_by_user(user.id).await.unwrap();
        assert!(sessions.len() > 0);
    }

    #[tokio::test]
    async fn test_exercise_set_crud() {
        let db = init_database().await.unwrap();
        let session_repo = WorkoutSessionRepository::new(db.pool().clone());
        let user_repo = UserRepository::new(db.pool().clone());
        let exercise_repo = ExerciseRepository::new(db.pool().clone());
        let set_repo = ExerciseSetRepository::new(db.pool().clone());
        
        // Create test data
        let new_user = NewUser {
            email: "set_test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
        };
        let user = user_repo.create(new_user).await.unwrap();
        
        let new_exercise = NewExercise {
            name: "Test Exercise".to_string(),
            muscle_group: "Chest".to_string(),
            equipment: "Barbell".to_string(),
            created_by: None,
        };
        let exercise = exercise_repo.create(new_exercise).await.unwrap();
        
        let new_session = NewWorkoutSession {
            user_id: user.id,
            date: chrono::Utc::now(),
            notes: Some("Test workout".to_string()),
        };
        let session = session_repo.create(new_session).await.unwrap();
        
        // Create exercise set
        let new_set = NewExerciseSet {
            session_id: session.id,
            exercise_id: exercise.id,
            set_number: 1,
            reps: 10,
            weight: Some(50.0),
        };
        
        let set = set_repo.create(new_set).await.unwrap();
        assert_eq!(set.session_id, session.id);
        assert_eq!(set.exercise_id, exercise.id);
        assert_eq!(set.set_number, 1);
        assert_eq!(set.reps, 10);
        assert_eq!(set.weight, Some(50.0));
        
        // Find sets by session
        let sets = set_repo.find_by_session(session.id).await.unwrap();
        assert!(sets.len() > 0);
        
        // Find sets by session and exercise
        let sets = set_repo.find_by_session_and_exercise(session.id, exercise.id).await.unwrap();
        assert!(sets.len() > 0);
    }
}