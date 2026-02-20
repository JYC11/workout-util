# Status: MVP Finished ✅

# Why
- when I work out, I normally keep my plan and progress in a personal discord server or a simple text file.
- but that makes the logs hard to read/track
- I also use a metronome from google by literally googling "metronome" and using the google provided metronome
- I also don't really keep track of the rest times
- so to solve all of that, I made this
- it's a simple desktop app that keep track of my workouts, workout logs, rest times and metronome

## Goals of this project
- make something that I want to use ✅
- actually finish a project and not abandon it ✅
  - by aggressively cut down on scope and complexity
- learn intuition behind client side development (it's all about state management!) ✅
- use LLMs extensively during development to get more practice using LLMs ✅
- learn about egui, sqlx ✅

## Next Steps
- actually use the app and see where it's needs improvement
- statistics?
- something for the home page?

### AI usage note
- so I wrote this project with AI writing about 70~80% of the code

#### Some insights from using AI
- AI is very good at basic simple stuff (forms, pagination for client side development and SQL for "backend" development) and this project was a good showcase of that
- AI is terrible at writing good abstractions so if you let it go wild, you write a bunch of slop
- if you poke it ant prod it right, AI can make you super fast. Without AI, this would have taken me several weeks even months from trial and error

#### Models used
- Gemini 3.1 Pro (around 95% of the time)
- Claud 4.5 Opus (5% of the time when Gemini randomly failed)

#### AI integration tool
- Jetbrains official AI plugin with the AI Ultimate plan
  - IDE: IntelliJ IDEA with Rust support

#### Techniques used for AI code generation
- coming up with a high level plan
- splitting up plan into small tasks (just in my head)
- defining method/function signatures and scaffolding for the AI
- directing AI towards samples of code it can reference that was of good enough quality that can be reused
- aggressively starting new chats for every task, and not using long-running conversations