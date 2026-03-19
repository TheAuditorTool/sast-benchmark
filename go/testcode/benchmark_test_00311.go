package testcode

import (
	"github.com/gofiber/fiber/v2"
)

func BenchmarkTest00311(c *fiber.Ctx) error {
	id := c.Query("id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	defer rows.Close()
	return c.JSON(fiber.Map{"status": "ok"})
}
