/**
 * Pre-built workout plan templates
 */

import type { PlanDraft } from './builderState';

export interface WorkoutTemplate {
  id: string;
  name: string;
  description: string;
  category: 'strength' | 'cardio' | 'hybrid';
  difficulty: 'beginner' | 'intermediate' | 'advanced';
  plan: PlanDraft;
}

export const templates: WorkoutTemplate[] = [
  {
    id: '5x5-strength',
    name: '5×5 Strength Program',
    description: 'Classic strength building program with compound lifts. 3 days per week, alternating A/B workouts.',
    category: 'strength',
    difficulty: 'beginner',
    plan: {
      plan_version: 1,
      meta: {
        name: '5×5 Strength Program',
        description: 'Classic barbell strength program focusing on compound lifts',
        author: 'PWF Templates',
        equipment: ['barbell', 'squat-rack', 'bench'],
        days_per_week: 3,
        tags: ['strength', 'barbell', 'beginner', 'compound']
      },
      cycle: {
        days: [
          {
            focus: 'Full Body A',
            notes: 'Start with the bar and add 5lbs each workout',
            exercises: [
              {
                name: 'Squat',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Barbell Row',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              }
            ]
          },
          {
            focus: 'Full Body B',
            notes: 'Add 5lbs to deadlift each workout',
            exercises: [
              {
                name: 'Squat',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Overhead Press',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Deadlift',
                modality: 'strength',
                target_sets: 1,
                target_reps: 5,
                rest_between_sets_sec: 180
              }
            ]
          },
          {
            focus: 'Full Body A',
            notes: 'Repeat workout A',
            exercises: [
              {
                name: 'Squat',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              },
              {
                name: 'Barbell Row',
                modality: 'strength',
                target_sets: 5,
                target_reps: 5,
                rest_between_sets_sec: 180
              }
            ]
          }
        ]
      }
    }
  },
  {
    id: 'ppl-split',
    name: 'Push/Pull/Legs Split',
    description: '6-day training split separating pushing, pulling, and leg movements',
    category: 'strength',
    difficulty: 'intermediate',
    plan: {
      plan_version: 1,
      meta: {
        name: 'Push/Pull/Legs Split',
        description: 'Classic bodybuilding split for hypertrophy',
        author: 'PWF Templates',
        equipment: ['barbell', 'dumbbells', 'cables', 'machines'],
        days_per_week: 6,
        tags: ['hypertrophy', 'bodybuilding', 'intermediate']
      },
      cycle: {
        days: [
          {
            focus: 'Push',
            notes: 'Chest, shoulders, and triceps',
            exercises: [
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 4,
                target_reps: 8,
                rest_between_sets_sec: 90
              },
              {
                name: 'Overhead Press',
                modality: 'strength',
                target_sets: 4,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Incline Dumbbell Press',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              },
              {
                name: 'Lateral Raises',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              },
              {
                name: 'Tricep Pushdowns',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              }
            ]
          },
          {
            focus: 'Pull',
            notes: 'Back and biceps',
            exercises: [
              {
                name: 'Deadlift',
                modality: 'strength',
                target_sets: 4,
                target_reps: 6,
                rest_between_sets_sec: 120
              },
              {
                name: 'Pull-ups',
                modality: 'strength',
                target_sets: 4,
                target_reps: 8,
                rest_between_sets_sec: 90
              },
              {
                name: 'Barbell Row',
                modality: 'strength',
                target_sets: 4,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Face Pulls',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              },
              {
                name: 'Barbell Curl',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              }
            ]
          },
          {
            focus: 'Legs',
            notes: 'Quads, hamstrings, and calves',
            exercises: [
              {
                name: 'Squat',
                modality: 'strength',
                target_sets: 4,
                target_reps: 8,
                rest_between_sets_sec: 120
              },
              {
                name: 'Romanian Deadlift',
                modality: 'strength',
                target_sets: 4,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Leg Press',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 90
              },
              {
                name: 'Leg Curl',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              },
              {
                name: 'Calf Raises',
                modality: 'strength',
                target_sets: 4,
                target_reps: 15,
                rest_between_sets_sec: 60
              }
            ]
          },
          {
            focus: 'Push',
            notes: 'Second push day with variation',
            exercises: [
              {
                name: 'Incline Barbell Press',
                modality: 'strength',
                target_sets: 4,
                target_reps: 8,
                rest_between_sets_sec: 90
              },
              {
                name: 'Dumbbell Shoulder Press',
                modality: 'strength',
                target_sets: 4,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Cable Flyes',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              },
              {
                name: 'Front Raises',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              },
              {
                name: 'Overhead Tricep Extension',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              }
            ]
          },
          {
            focus: 'Pull',
            notes: 'Second pull day with variation',
            exercises: [
              {
                name: 'Weighted Pull-ups',
                modality: 'strength',
                target_sets: 4,
                target_reps: 6,
                rest_between_sets_sec: 120
              },
              {
                name: 'T-Bar Row',
                modality: 'strength',
                target_sets: 4,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Lat Pulldown',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              },
              {
                name: 'Rear Delt Flyes',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              },
              {
                name: 'Hammer Curl',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 60
              }
            ]
          },
          {
            focus: 'Legs',
            notes: 'Second leg day with variation',
            exercises: [
              {
                name: 'Front Squat',
                modality: 'strength',
                target_sets: 4,
                target_reps: 8,
                rest_between_sets_sec: 120
              },
              {
                name: 'Walking Lunges',
                modality: 'strength',
                target_sets: 3,
                target_reps: 12,
                rest_between_sets_sec: 90
              },
              {
                name: 'Leg Extension',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60
              },
              {
                name: 'Glute Ham Raise',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                rest_between_sets_sec: 90
              },
              {
                name: 'Seated Calf Raises',
                modality: 'strength',
                target_sets: 4,
                target_reps: 20,
                rest_between_sets_sec: 60
              }
            ]
          }
        ]
      }
    }
  },
  {
    id: 'hiit-cardio',
    name: 'HIIT Cardio Program',
    description: '4-week high-intensity interval training program',
    category: 'cardio',
    difficulty: 'intermediate',
    plan: {
      plan_version: 1,
      meta: {
        name: 'HIIT Cardio Program',
        description: 'High-intensity interval training for fat loss and cardiovascular fitness',
        author: 'PWF Templates',
        equipment: ['timer', 'none'],
        days_per_week: 4,
        tags: ['cardio', 'hiit', 'fat-loss', 'bodyweight']
      },
      cycle: {
        days: [
          {
            focus: 'Tabata Intervals',
            notes: '20 seconds work, 10 seconds rest, 8 rounds',
            exercises: [
              {
                name: 'Burpees',
                modality: 'interval',
                target_sets: 8,
                target_duration_sec: 20,
                rest_between_sets_sec: 10,
                target_notes: 'Maximum effort for 20 seconds'
              },
              {
                name: 'Mountain Climbers',
                modality: 'interval',
                target_sets: 8,
                target_duration_sec: 20,
                rest_between_sets_sec: 10
              },
              {
                name: 'Jump Squats',
                modality: 'interval',
                target_sets: 8,
                target_duration_sec: 20,
                rest_between_sets_sec: 10
              }
            ]
          },
          {
            focus: 'Endurance Intervals',
            notes: '45 seconds work, 15 seconds rest',
            exercises: [
              {
                name: 'High Knees',
                modality: 'interval',
                target_sets: 6,
                target_duration_sec: 45,
                rest_between_sets_sec: 15
              },
              {
                name: 'Jumping Jacks',
                modality: 'interval',
                target_sets: 6,
                target_duration_sec: 45,
                rest_between_sets_sec: 15
              },
              {
                name: 'Plank Jacks',
                modality: 'interval',
                target_sets: 6,
                target_duration_sec: 45,
                rest_between_sets_sec: 15
              },
              {
                name: 'Butt Kicks',
                modality: 'interval',
                target_sets: 6,
                target_duration_sec: 45,
                rest_between_sets_sec: 15
              }
            ]
          },
          {
            focus: 'Power Intervals',
            notes: '30 seconds work, 30 seconds rest',
            exercises: [
              {
                name: 'Box Jumps',
                modality: 'interval',
                target_sets: 10,
                target_duration_sec: 30,
                rest_between_sets_sec: 30,
                target_notes: 'Land softly, full hip extension'
              },
              {
                name: 'Sprawls',
                modality: 'interval',
                target_sets: 10,
                target_duration_sec: 30,
                rest_between_sets_sec: 30
              },
              {
                name: 'Lateral Bounds',
                modality: 'interval',
                target_sets: 10,
                target_duration_sec: 30,
                rest_between_sets_sec: 30
              }
            ]
          },
          {
            focus: 'Mixed Intervals',
            notes: 'EMOM style - every minute on the minute',
            exercises: [
              {
                name: 'Sprint in Place',
                modality: 'interval',
                target_sets: 12,
                target_duration_sec: 20,
                rest_between_sets_sec: 40,
                target_notes: 'Go hard for 20s, rest remaining 40s'
              },
              {
                name: 'Push-up to T',
                modality: 'interval',
                target_sets: 12,
                target_duration_sec: 20,
                rest_between_sets_sec: 40
              }
            ]
          }
        ]
      }
    }
  },
  {
    id: 'beginner-calisthenics',
    name: 'Beginner Calisthenics',
    description: 'Bodyweight training program for beginners',
    category: 'strength',
    difficulty: 'beginner',
    plan: {
      plan_version: 1,
      meta: {
        name: 'Beginner Calisthenics Program',
        description: 'Build strength with bodyweight exercises, no equipment required',
        author: 'PWF Templates',
        equipment: ['pull-up-bar', 'none'],
        days_per_week: 3,
        tags: ['bodyweight', 'calisthenics', 'beginner', 'home-workout']
      },
      cycle: {
        days: [
          {
            focus: 'Upper Body Push',
            notes: 'Focus on form over speed',
            exercises: [
              {
                name: 'Push-ups',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                rest_between_sets_sec: 90,
                target_notes: 'Knee push-ups are fine if needed'
              },
              {
                name: 'Pike Push-ups',
                modality: 'strength',
                target_sets: 3,
                target_reps: 8,
                rest_between_sets_sec: 90,
                target_notes: 'For shoulders'
              },
              {
                name: 'Tricep Dips',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                rest_between_sets_sec: 90,
                target_notes: 'Use a chair or bench'
              },
              {
                name: 'Plank',
                modality: 'countdown',
                target_duration_sec: 30,
                target_notes: 'Hold strong core position'
              }
            ]
          },
          {
            focus: 'Lower Body',
            notes: 'Control the movement, focus on depth',
            exercises: [
              {
                name: 'Bodyweight Squats',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60,
                target_notes: 'Squat to parallel or below'
              },
              {
                name: 'Walking Lunges',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                rest_between_sets_sec: 60,
                target_notes: '10 reps per leg'
              },
              {
                name: 'Glute Bridges',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15,
                rest_between_sets_sec: 60,
                target_notes: 'Squeeze glutes at top'
              },
              {
                name: 'Calf Raises',
                modality: 'strength',
                target_sets: 3,
                target_reps: 20,
                rest_between_sets_sec: 45
              },
              {
                name: 'Wall Sit',
                modality: 'countdown',
                target_duration_sec: 30,
                target_notes: 'Thighs parallel to ground'
              }
            ]
          },
          {
            focus: 'Upper Body Pull',
            notes: 'Use assistance if needed for pull-ups',
            exercises: [
              {
                name: 'Pull-ups',
                modality: 'strength',
                target_sets: 3,
                target_reps: 5,
                rest_between_sets_sec: 120,
                target_notes: 'Use resistance band or negatives if needed'
              },
              {
                name: 'Inverted Rows',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                rest_between_sets_sec: 90,
                target_notes: 'Use a table or low bar'
              },
              {
                name: 'Superman Hold',
                modality: 'countdown',
                target_duration_sec: 20,
                target_notes: 'Strengthen lower back'
              },
              {
                name: 'Side Plank',
                modality: 'countdown',
                target_duration_sec: 20,
                target_notes: '20 seconds each side'
              },
              {
                name: 'Hollow Body Hold',
                modality: 'countdown',
                target_duration_sec: 20,
                target_notes: 'Core compression exercise'
              }
            ]
          }
        ]
      }
    }
  }
];

/**
 * Get all available templates
 */
export function getAllTemplates(): WorkoutTemplate[] {
  return templates;
}

/**
 * Get template by ID
 */
export function getTemplateById(id: string): WorkoutTemplate | undefined {
  return templates.find(t => t.id === id);
}

/**
 * Get templates by category
 */
export function getTemplatesByCategory(category: 'strength' | 'cardio' | 'hybrid'): WorkoutTemplate[] {
  return templates.filter(t => t.category === category);
}

/**
 * Get templates by difficulty
 */
export function getTemplatesByDifficulty(difficulty: 'beginner' | 'intermediate' | 'advanced'): WorkoutTemplate[] {
  return templates.filter(t => t.difficulty === difficulty);
}
