package repository

import (
	"fmt"
	"log"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

// Database wraps the GORM DB connection
type Database struct {
	DB *gorm.DB
}

// NewDatabase creates a new database connection
func NewDatabase(dsn string) (*Database, error) {
	db, err := gorm.Open(sqlite.Open(dsn), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})
	if err != nil {
		return nil, fmt.Errorf("failed to connect to database: %w", err)
	}

	return &Database{DB: db}, nil
}

// AutoMigrate runs database migrations
func (d *Database) AutoMigrate() error {
	return d.DB.AutoMigrate(
		&models.User{},
		&models.Meal{},
		&models.Exercise{},
		&models.Workout{},
		&models.Schedule{},
		&models.ScheduleItem{},
		&models.Food{},
		&models.DailyLog{},
		&models.WeightLog{},
	)
}

// SeedData seeds initial data
func (d *Database) SeedData() error {
	// Seed exercises
	exercises := []models.Exercise{
		{Name: "Running", Category: "cardio", CaloriesPerMinute: 10, Description: "Outdoor or treadmill running"},
		{Name: "Walking", Category: "cardio", CaloriesPerMinute: 4, Description: "Brisk walking"},
		{Name: "Cycling", Category: "cardio", CaloriesPerMinute: 8, Description: "Stationary or outdoor cycling"},
		{Name: "Swimming", Category: "cardio", CaloriesPerMinute: 9, Description: "Lap swimming"},
		{Name: "Jump Rope", Category: "cardio", CaloriesPerMinute: 12, Description: "Jump rope exercise"},
		{Name: "Weight Training", Category: "strength", CaloriesPerMinute: 5, Description: "General weight lifting"},
		{Name: "Push-ups", Category: "strength", CaloriesPerMinute: 7, Description: "Bodyweight push-ups"},
		{Name: "Squats", Category: "strength", CaloriesPerMinute: 6, Description: "Bodyweight or weighted squats"},
		{Name: "Deadlifts", Category: "strength", CaloriesPerMinute: 6, Description: "Barbell deadlifts"},
		{Name: "Bench Press", Category: "strength", CaloriesPerMinute: 5, Description: "Barbell or dumbbell bench press"},
		{Name: "Yoga", Category: "flexibility", CaloriesPerMinute: 3, Description: "Yoga practice"},
		{Name: "Stretching", Category: "flexibility", CaloriesPerMinute: 2, Description: "General stretching"},
		{Name: "Pilates", Category: "flexibility", CaloriesPerMinute: 4, Description: "Pilates workout"},
		{Name: "HIIT", Category: "cardio", CaloriesPerMinute: 14, Description: "High-intensity interval training"},
		{Name: "Rowing", Category: "cardio", CaloriesPerMinute: 9, Description: "Rowing machine"},
	}

	for _, e := range exercises {
		var existing models.Exercise
		if err := d.DB.Where("name = ?", e.Name).First(&existing).Error; err != nil {
			if err := d.DB.Create(&e).Error; err != nil {
				log.Printf("Failed to seed exercise %s: %v", e.Name, err)
			}
		}
	}

	// Seed foods
	foods := []models.Food{
		{Name: "Chicken Breast", Category: "protein", ServingSize: 100, ServingUnit: "g", CaloriesPerServing: 165, ProteinGrams: 31, CarbsGrams: 0, FatGrams: 3.6},
		{Name: "Brown Rice", Category: "grains", ServingSize: 100, ServingUnit: "g", CaloriesPerServing: 112, ProteinGrams: 2.6, CarbsGrams: 23, FatGrams: 0.9},
		{Name: "Broccoli", Category: "vegetables", ServingSize: 100, ServingUnit: "g", CaloriesPerServing: 34, ProteinGrams: 2.8, CarbsGrams: 7, FatGrams: 0.4},
		{Name: "Salmon", Category: "protein", ServingSize: 100, ServingUnit: "g", CaloriesPerServing: 208, ProteinGrams: 20, CarbsGrams: 0, FatGrams: 13},
		{Name: "Egg", Category: "protein", ServingSize: 1, ServingUnit: "large", CaloriesPerServing: 78, ProteinGrams: 6, CarbsGrams: 0.6, FatGrams: 5},
		{Name: "Banana", Category: "fruit", ServingSize: 1, ServingUnit: "medium", CaloriesPerServing: 105, ProteinGrams: 1.3, CarbsGrams: 27, FatGrams: 0.4},
		{Name: "Apple", Category: "fruit", ServingSize: 1, ServingUnit: "medium", CaloriesPerServing: 95, ProteinGrams: 0.5, CarbsGrams: 25, FatGrams: 0.3},
		{Name: "Greek Yogurt", Category: "dairy", ServingSize: 170, ServingUnit: "g", CaloriesPerServing: 100, ProteinGrams: 17, CarbsGrams: 6, FatGrams: 0.7},
		{Name: "Oatmeal", Category: "grains", ServingSize: 40, ServingUnit: "g", CaloriesPerServing: 150, ProteinGrams: 5, CarbsGrams: 27, FatGrams: 3},
		{Name: "Almonds", Category: "nuts", ServingSize: 28, ServingUnit: "g", CaloriesPerServing: 164, ProteinGrams: 6, CarbsGrams: 6, FatGrams: 14},
	}

	for _, f := range foods {
		var existing models.Food
		if err := d.DB.Where("name = ?", f.Name).First(&existing).Error; err != nil {
			if err := d.DB.Create(&f).Error; err != nil {
				log.Printf("Failed to seed food %s: %v", f.Name, err)
			}
		}
	}

	return nil
}

// ExecuteRaw executes a raw SQL query - VULNERABLE
// TAINT SINK: Direct SQL execution
func (d *Database) ExecuteRaw(query string) error {
	return d.DB.Exec(query).Error
}

// QueryRaw executes a raw SQL query and returns results - VULNERABLE
// TAINT SINK: Direct SQL execution
func (d *Database) QueryRaw(query string) ([]map[string]interface{}, error) {
	var results []map[string]interface{}
	rows, err := d.DB.Raw(query).Rows()
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	columns, _ := rows.Columns()
	for rows.Next() {
		values := make([]interface{}, len(columns))
		valuePtrs := make([]interface{}, len(columns))
		for i := range values {
			valuePtrs[i] = &values[i]
		}
		rows.Scan(valuePtrs...)
		row := make(map[string]interface{})
		for i, col := range columns {
			row[col] = values[i]
		}
		results = append(results, row)
	}
	return results, nil
}
