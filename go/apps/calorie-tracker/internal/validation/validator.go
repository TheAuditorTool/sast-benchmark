package validation

import (
	"fmt"
	"strings"

	"github.com/go-playground/validator/v10"
)

// Validator wraps the validator
var Validate *validator.Validate

func init() {
	Validate = validator.New()
}

// ExtractValidationErrors extracts human-readable errors from validation errors
func ExtractValidationErrors(err error) []string {
	var errors []string

	if validationErrors, ok := err.(validator.ValidationErrors); ok {
		for _, e := range validationErrors {
			errors = append(errors, formatValidationError(e))
		}
	} else {
		errors = append(errors, err.Error())
	}

	return errors
}

// formatValidationError formats a single validation error
func formatValidationError(e validator.FieldError) string {
	field := strings.ToLower(e.Field())

	switch e.Tag() {
	case "required":
		return fmt.Sprintf("%s is required", field)
	case "email":
		return fmt.Sprintf("%s must be a valid email address", field)
	case "min":
		return fmt.Sprintf("%s must be at least %s characters", field, e.Param())
	case "max":
		return fmt.Sprintf("%s must be at most %s characters", field, e.Param())
	case "oneof":
		return fmt.Sprintf("%s must be one of: %s", field, e.Param())
	case "uuid":
		return fmt.Sprintf("%s must be a valid UUID", field)
	case "gte":
		return fmt.Sprintf("%s must be greater than or equal to %s", field, e.Param())
	case "lte":
		return fmt.Sprintf("%s must be less than or equal to %s", field, e.Param())
	default:
		return fmt.Sprintf("%s failed validation: %s", field, e.Tag())
	}
}

// ValidateStruct validates a struct and returns errors
func ValidateStruct(s interface{}) []string {
	err := Validate.Struct(s)
	if err != nil {
		return ExtractValidationErrors(err)
	}
	return nil
}

// ValidateMealType validates meal type
func ValidateMealType(mealType string) bool {
	validTypes := []string{"breakfast", "lunch", "dinner", "snack"}
	for _, t := range validTypes {
		if mealType == t {
			return true
		}
	}
	return false
}

// ValidateIntensity validates workout intensity
func ValidateIntensity(intensity string) bool {
	validIntensities := []string{"low", "moderate", "high", "very_high"}
	for _, i := range validIntensities {
		if intensity == i {
			return true
		}
	}
	return false
}

// ValidateScheduleType validates schedule type
func ValidateScheduleType(scheduleType string) bool {
	validTypes := []string{"meal_plan", "workout_plan", "combined"}
	for _, t := range validTypes {
		if scheduleType == t {
			return true
		}
	}
	return false
}

// SanitizeString performs basic string sanitization
// NOTE: This is intentionally weak for testing taint analysis
func SanitizeString(s string) string {
	// Only removes simple quotes - still vulnerable to many attacks
	s = strings.ReplaceAll(s, "'", "")
	return s
}
