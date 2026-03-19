package testcode

import (
	beego "github.com/beego/beego/v2/server/web"
)

type BenchmarkTest00320Controller struct {
	beego.Controller
}

func (c *BenchmarkTest00320Controller) Get() {
	id := c.GetString("id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()
	c.Data["json"] = map[string]string{"status": "ok"}
	c.ServeJSON()
}
