package testcode

import (
	"os/exec"

	"github.com/gofiber/fiber/v2"
)

func BenchmarkTest00310(c *fiber.Ctx) error {
	body := string(c.Body())
	out, err := exec.Command("sh", "-c", body).CombinedOutput()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.JSON(fiber.Map{"output": string(out)})
}
