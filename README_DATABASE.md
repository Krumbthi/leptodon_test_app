# Workout Tracker Database Setup

This project includes a PostgreSQL database with the following domain model:

## Database Schema

### Core Entities

1. **users** - User accounts
   - id (UUID, primary key)
   - email (unique)
   - password_hash
   - created_at

2. **exercises** - Exercise definitions
   - id (UUID, primary key)
   - name
   - muscle_group
   - equipment
   - created_by (foreign key to users)
   - created_at

3. **workout_sessions** - Individual workout sessions
   - id (UUID, primary key)
   - user_id (foreign key to users)
   - date
   - notes
   - created_at

4. **exercise_sets** - Individual sets within workouts
   - id (UUID, primary key)
   - session_id (foreign key to workout_sessions)
   - exercise_id (foreign key to exercises)
   - set_number
   - reps
   - weight
   - created_at

## Setup Instructions

### 1. Start PostgreSQL with Docker

```bash
docker-compose up -d
```

This will start a PostgreSQL container with:
- Database: `workout_tracker`
- User: `workout_user`
- Password: `workout_password`
- Port: `5432`

### 2. Set Environment Variables

The `.env` file is already configured with the database connection string:
```
DATABASE_URL=postgres://workout_user:workout_password@localhost:5432/workout_tracker
```

### 3. Run Migrations

The migrations are automatically run when the application starts, but you can also run them manually:

```bash
cargo sqlx migrate run
```

### 4. Test the Database

Run the database tests:

```bash
cargo test --test tests
```

## Usage

### Starting the Application

```bash
cargo run
```

The application will:
1. Connect to the PostgreSQL database
2. Run any pending migrations
3. Start the web server

### Docker Commands

```bash
# Start database
docker-compose up -d

# Stop database
docker-compose down

# View logs
docker-compose logs -f postgres

# Access PostgreSQL shell
docker-compose exec postgres psql -U workout_user -d workout_tracker
```

## API Endpoints (Future)

The database is ready to support REST API endpoints for:
- User registration and authentication
- Exercise management (CRUD operations)
- Workout session tracking
- Exercise set logging
- Workout history and analytics

## Sample Data

The migration includes sample exercises:
- Bench Press (Chest, Barbell)
- Squat (Legs, Barbell)
- Deadlift (Back, Barbell)
- Overhead Press (Shoulders, Barbell)
- And more...

## Troubleshooting

### Database Connection Issues

1. Ensure Docker is running
2. Check that the PostgreSQL container is running:
   ```bash
   docker-compose ps
   ```
3. Verify the database is accessible:
   ```bash
   docker-compose exec postgres pg_isready -U workout_user -d workout_tracker
   ```

### Migration Issues

If migrations fail:
1. Check the database connection
2. Verify the migrations directory exists
3. Check the migration file syntax

### Development

To reset the database:
```bash
docker-compose down -v
docker-compose up -d
```

This will remove the volume and start fresh.