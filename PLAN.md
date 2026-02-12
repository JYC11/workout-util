# planning

### what I need
- metronome to count seconds while working out for statics
- rest timer
- EMOM timer
- exercise library (CRUD)
  - exercise name
  - lever variation (tuck, adv tuck, straddle, one leg, half lay, full)
  - grip
- equipment library
  - band
  - parallettes (high, low)
  - bench
  - dumbbells
  - barbell
  - smith
  - gymnastic rings
  - pull up bar
  - dip bar
- current workout (CRUD)
- workout logger (CRUD)
  - sets, reps, weight, equipment, duration, date, notes

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
  - Layout: Master-Detail view.
  - Left Column (30% width): Scrollable list of exercises with a search bar at the top.
  - Right Column (70% width):
    - View Mode: Details (Name, Lever, Grip).
    - Edit Mode: Input fields.
  - Design Tip: Use egui::ComboBox for Lever and Equipment selection to keep data clean.
- Tab 2: Create Workout (CRUD)
  - Layout: Two distinct sub pages you can switch between.
  - Subpage 1: "Available Exercises" (draggable or clickable "+" button).
    - Workout Exercises CRUD
  - Subpage 2: "Current Workouts"
    - Workout Plans CRUD, construct a workout from selected workout exercises.
    - EG:
      - Workout A = [Exercise 1, Exercise 2, Exercise 3]
      - Workout B = [Exercise 4, Exercise 5, Exercise 6]
  - Action: "Start this Workout" button at the bottom right.
- Tab 3: Create Workout Plans (CRUD)
  - Layout: List of workouts.
  - Builds Workout Plan from selected Workouts
    - A workout plan is a list of workouts
    - EG:
      - Workout Plan A = [Workout A, Workout B]
        - repeated every week, for N weeks.
        - Alternating A and B with rest days in between.
  - Action: "Start this Workout" button at the bottom right. 
- Tab 4: Active Workout (The Logger)
  - Header: Current Exercise Name (Big Font).
  - Content:
    - Previous Data: "Last time: 5 sets, 10 reps @ 20kg" (faded text).
    - Current Set: Inputs for Reps, Weight, RPE.
    - Save Set Button: big button that also auto-starts the Rest Timer.
  - Footer: "Next Exercise" / "Finish Workout".
