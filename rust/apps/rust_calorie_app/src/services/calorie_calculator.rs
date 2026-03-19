//! Calorie calculation service.
//!
//! Provides calculations for workout calories and nutritional conversions.
//! TAINT TRANSFORM: User input data flows through calculations.

use crate::models::Intensity;

/// Calorie calculation utilities
pub struct CalorieCalculator;

impl CalorieCalculator {
    /// Calculate calories burned during a workout.
    ///
    /// TAINT TRANSFORM: duration (HTTP) * calories_per_minute * intensity -> result
    pub fn calculate_workout_calories(
        duration_minutes: i32,
        calories_per_minute: i32,
        intensity: &Intensity,
    ) -> i32 {
        // TAINT: duration_minutes from HTTP input
        let base_calories = duration_minutes * calories_per_minute;

        // Apply intensity multiplier
        // TAINT: intensity from HTTP input
        let adjusted = base_calories as f64 * intensity.calorie_multiplier();

        adjusted.round() as i32
    }

    /// Estimate calories from duration using standard MET values.
    ///
    /// TAINT TRANSFORM: duration (HTTP) -> estimation
    pub fn estimate_calories_from_duration(duration_minutes: i32, intensity: &Intensity) -> i32 {
        // Average MET values by intensity
        // TAINT: duration_minutes from HTTP input
        let base_met = match intensity {
            Intensity::Light => 3.0,     // Light activity
            Intensity::Moderate => 5.0,  // Moderate activity
            Intensity::Intense => 8.0,   // Vigorous activity
            Intensity::Extreme => 12.0,  // Very high intensity
        };

        // Assume average weight of 70kg
        // Calories = MET * weight(kg) * time(hours)
        let hours = duration_minutes as f64 / 60.0;
        let calories = base_met * 70.0 * hours;

        calories.round() as i32
    }

    /// Calculate calories from macronutrients.
    ///
    /// TAINT TRANSFORM: macro inputs -> calorie calculation
    pub fn calculate_calories_from_macros(protein_g: f64, carbs_g: f64, fat_g: f64) -> i32 {
        // Standard calorie values per gram
        // TAINT: All inputs can be from HTTP
        let protein_cal = protein_g * 4.0;
        let carbs_cal = carbs_g * 4.0;
        let fat_cal = fat_g * 9.0;

        (protein_cal + carbs_cal + fat_cal).round() as i32
    }

    /// Calculate BMR (Basal Metabolic Rate) using Mifflin-St Jeor equation.
    ///
    /// TAINT TRANSFORM: User profile data -> BMR
    pub fn calculate_bmr(weight_kg: f64, height_cm: f64, age_years: i32, is_male: bool) -> i32 {
        // TAINT: All inputs from user profile (HTTP origin)
        let bmr = if is_male {
            (10.0 * weight_kg) + (6.25 * height_cm) - (5.0 * age_years as f64) + 5.0
        } else {
            (10.0 * weight_kg) + (6.25 * height_cm) - (5.0 * age_years as f64) - 161.0
        };

        bmr.round() as i32
    }

    /// Calculate TDEE (Total Daily Energy Expenditure).
    ///
    /// TAINT TRANSFORM: BMR + activity level -> TDEE
    pub fn calculate_tdee(bmr: i32, activity_level: &str) -> i32 {
        // TAINT: activity_level from HTTP
        let multiplier = match activity_level.to_lowercase().as_str() {
            "sedentary" => 1.2,
            "light" => 1.375,
            "moderate" => 1.55,
            "active" => 1.725,
            "very_active" => 1.9,
            _ => 1.55, // Default to moderate
        };

        (bmr as f64 * multiplier).round() as i32
    }

    /// Calculate net calories for a day.
    ///
    /// TAINT TRANSFORM: consumed (meals) - burned (workouts) = net
    pub fn calculate_net_calories(calories_consumed: i32, calories_burned: i32) -> i32 {
        // TAINT: Both inputs derived from HTTP data (meal entries, workout entries)
        calories_consumed - calories_burned
    }

    /// Calculate macro percentages.
    ///
    /// TAINT TRANSFORM: macro grams -> percentages
    pub fn calculate_macro_percentages(
        protein_g: f64,
        carbs_g: f64,
        fat_g: f64,
    ) -> (f64, f64, f64) {
        // TAINT: All inputs from meal data (HTTP origin)
        let total_cal = Self::calculate_calories_from_macros(protein_g, carbs_g, fat_g) as f64;

        if total_cal == 0.0 {
            return (0.0, 0.0, 0.0);
        }

        let protein_pct = (protein_g * 4.0 / total_cal) * 100.0;
        let carbs_pct = (carbs_g * 4.0 / total_cal) * 100.0;
        let fat_pct = (fat_g * 9.0 / total_cal) * 100.0;

        (protein_pct, carbs_pct, fat_pct)
    }

    /// Calculate remaining calories for goal.
    ///
    /// TAINT TRANSFORM: goal (user setting) - consumed + burned = remaining
    pub fn calculate_remaining(goal: i32, consumed: i32, burned: i32) -> i32 {
        // TAINT: All values derived from HTTP data
        goal - consumed + burned
    }

    /// Check if calorie goal is met.
    pub fn is_goal_met(goal: i32, consumed: i32, burned: i32) -> bool {
        let net = Self::calculate_net_calories(consumed, burned);
        net <= goal
    }

    /// Calculate calories needed to reach goal.
    pub fn calories_to_goal(goal: i32, consumed: i32, burned: i32) -> i32 {
        let remaining = Self::calculate_remaining(goal, consumed, burned);
        remaining.max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workout_calories() {
        let calories = CalorieCalculator::calculate_workout_calories(30, 10, &Intensity::Moderate);
        assert_eq!(calories, 300);

        let intense = CalorieCalculator::calculate_workout_calories(30, 10, &Intensity::Intense);
        assert_eq!(intense, 375); // 300 * 1.25
    }

    #[test]
    fn test_macro_calories() {
        // 20g protein (80cal) + 50g carbs (200cal) + 10g fat (90cal) = 370cal
        let calories = CalorieCalculator::calculate_calories_from_macros(20.0, 50.0, 10.0);
        assert_eq!(calories, 370);
    }

    #[test]
    fn test_net_calories() {
        let net = CalorieCalculator::calculate_net_calories(2000, 500);
        assert_eq!(net, 1500);
    }
}
