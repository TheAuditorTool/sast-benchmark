package testcode

import (
	"github.com/gofiber/fiber/v2"
)

func BenchmarkTest00309(c *fiber.Ctx) error {
	name := c.Params("name")
	query := "SELECT * FROM users WHERE name = '" + name + "'"
	row := DB.QueryRow(query)
	var id int
	if err := row.Scan(&id); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.JSON(fiber.Map{"id": id})
}
