package testcode

import (
	"path/filepath"

	"github.com/gofiber/fiber/v2"
)

func BenchmarkTest00312(c *fiber.Ctx) error {
	filename := c.Params("file")
	safeName := filepath.Base(filename)
	return c.SendFile("/uploads/" + safeName)
}
