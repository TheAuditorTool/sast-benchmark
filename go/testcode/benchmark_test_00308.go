package testcode

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
)

func BenchmarkTest00308(c *fiber.Ctx) error {
	id := c.Query("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	defer rows.Close()
	return c.JSON(fiber.Map{"status": "ok"})
}
