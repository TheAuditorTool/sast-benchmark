package testcode

import (
	"fmt"

	beego "github.com/beego/beego/v2/server/web"
)

type BenchmarkTest00318Controller struct {
	beego.Controller
}

func (c *BenchmarkTest00318Controller) Get() {
	id := c.GetString("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()
	c.Data["json"] = map[string]string{"status": "ok"}
	c.ServeJSON()
}
