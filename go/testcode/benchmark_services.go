package testcode

import (
	"database/sql"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

type BenchSvcRequest struct {
	Query     string
	OutputDir string
	Command   string
}

type BenchSvcStore struct {
	db *sql.DB
}

func BenchSvcQueryUser(id string) error {
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func BenchSvcQueryUserV2(id string) error {
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func BenchSvcBuildQuery(table, where string) string {
	return fmt.Sprintf("SELECT * FROM %s WHERE %s", table, where)
}

func BenchSvcExecCmd(cmd string) ([]byte, error) {
	return exec.Command("sh", "-c", cmd).CombinedOutput()
}

func BenchSvcExecCmdV2(host string) ([]byte, error) {
	return exec.Command("ping", "-c", "1", host).Output()
}

func BenchSvcReadPath(name string) ([]byte, error) {
	path := filepath.Join("/var/data", name)
	return os.ReadFile(path)
}

func BenchSvcReadPathV2(name string) ([]byte, error) {
	safeName := filepath.Base(name)
	return os.ReadFile(filepath.Join("/var/data", safeName))
}

func BenchSvcTransform(input string) string {
	return "processed_" + input
}

func BenchSvcProcessSQL(input string) error {
	mainQuery := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", input)
	rows, err := DB.Query(mainQuery)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func BenchSvcProcessAll(input string) error {
	logQuery := fmt.Sprintf("INSERT INTO audit_log (action) VALUES ('%s')", input)
	DB.Exec(logQuery)

	exec.Command("sh", "-c", "echo "+input).Run()

	path := filepath.Join("/tmp/data", input+".txt")
	os.WriteFile(path, []byte("data"), 0644)

	mainQuery := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", input)
	rows, err := DB.Query(mainQuery)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func BenchSvcProcessStruct(req *BenchSvcRequest) error {
	query := fmt.Sprintf("SELECT * FROM data WHERE filter = '%s'", req.Query)
	rows, err := DB.Query(query)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func BenchSvcProcessStructV2(req *BenchSvcRequest) error {
	rows, err := DB.Query("SELECT * FROM data WHERE filter = ?", req.Query)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func (s *BenchSvcStore) FindUser(id string) error {
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := s.db.Query(query)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}

func (s *BenchSvcStore) FindUserV2(id string) error {
	rows, err := s.db.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		return err
	}
	defer rows.Close()
	return nil
}
