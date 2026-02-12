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
- Tab 2: Create Workout (Builder)
  - Layout: Two distinct vertical lists side-by-side.
  - Left: "Available Exercises" (draggable or clickable "+" button).
  - Right: "Current Workout Plan" (list of selected exercises).
  - Action: "Start this Workout" button at the bottom right.
- Tab 3: Active Workout (The Logger)
  - Header: Current Exercise Name (Big Font).
  - Content:
    - Previous Data: "Last time: 5 sets, 10 reps @ 20kg" (faded text).
    - Current Set: Inputs for Reps, Weight, RPE.
    - Save Set Button: big button that also auto-starts the Rest Timer.
  - Footer: "Next Exercise" / "Finish Workout".
