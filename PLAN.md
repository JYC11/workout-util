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
- home/entry page (tab 0)
- log exercises in exercise library (tab 1)
- create workout by selecting exercises and equipment (tab 2)
- during workout, log progress and use template from the workout (tab 3)
  - use metronome to count seconds
  - use rest timer between sets
  - nice to have: EMOM timer

