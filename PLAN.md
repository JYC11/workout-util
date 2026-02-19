# planning
### stack
- eframe
- sqlx
- sqlite
- something for audio

## plan
- setup data models and logic
- figure out how to use sqlx
- figure out how to use eframe


## flow
### General Layout Structure
- Top Panel (Navigation): Persistent tabs (Home, Exercises, Builder, Active Workout).
- Bottom Panel (Status/Timer): Persistent Metronome & Timer controls. This ensures that no matter where you are (checking logs or looking up an exercise), you can see your rest timer or hear the metronome.
- Central Panel: The main content area.
### Tab-Specific Layouts
- Tab 0: Home
  - Hero Section: "Start New Workout" (Big Green Button).
  - Recent Activity: Simple list of last 3 logged workouts.
  - Quick Stats: "Workouts this week: X".
- Tab 1: Exercise Library (CRUD)
  - Paged view and detail view for exercises
  - Design Tip: Use egui::ComboBox for Lever and Equipment selection to keep data clean.
- Tab 2: Create Workout (CRUD)
  - Workout Plans CRUD, construct a workout from selected workout exercises.
  - use drag and drop to create workout plans?
  - a searchable list on the top to select exercises.
  - a bottom form to add exercises to the workout, add sets, weight, seconds, reps, etc.
  - EG:
    - Workout A = [Exercise 1, Exercise 2, Exercise 3]
    - Workout B = [Exercise 4, Exercise 5, Exercise 6]
  - Action: "Start this Workout" button at the bottom right (save and then transition state to Active Workout).
- Tab 2.5: All Workouts
  - shows a list of all workouts. 
- Tab 3: Active Workout (The Logger)
  - Header: Current Exercise Name (Big Font).
  - Content:
    - Previous Data: "Last time: 5 sets, 10 reps @ 20kg" (faded text).
    - Current Set: Inputs for Reps, Weight, RPE.
    - Save Set Button: big button that also auto-starts the Rest Timer.
  - Footer: "Next Exercise" / "Finish Workout".
- Tab 4: History
  - shows logs of workouts 
  - page by log groups, has 1 button to see all log groups (get details by log group id)
  - 1 detail page for log group (shows log group and log entries in 1 detail page)
  - cannot delete or update log entries