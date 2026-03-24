package server

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/theauditor/grpc_users/repository"
)

// UserServer implements the gRPC UserService
type UserServer struct {
	db       *sql.DB
	userRepo *repository.UserRepository
	UnimplementedUserServiceServer
}

// NewUserServer creates a new gRPC user server
func NewUserServer(db *sql.DB) *UserServer {
	return &UserServer{
		db:       db,
		userRepo: repository.NewUserRepository(db),
	}
}

// UnimplementedUserServiceServer must be embedded for forward compatibility
type UnimplementedUserServiceServer struct{}

// GetUser retrieves a user by ID or username
func (s *UserServer) GetUser(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	var query string

	if req.Id != "" {
		query = fmt.Sprintf("SELECT * FROM users WHERE id = %s", req.Id)
	} else if req.Username != "" {
		query = fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", req.Username)
	} else {
		return &UserResponse{Error: "id or username required"}, nil
	}

	row := s.db.QueryRowContext(ctx, query)
	var user User
	err := row.Scan(&user.Id, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	return &UserResponse{User: &user}, nil
}

// SearchUsers searches for users with filters
func (s *UserServer) SearchUsers(ctx context.Context, req *SearchUsersRequest) (*SearchUsersResponse, error) {
	searchQuery := req.Query
	filter := req.Filter
	sortBy := req.SortBy
	limit := req.Limit
	offset := req.Offset

	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND %s ORDER BY %s LIMIT %d OFFSET %d",
		searchQuery, filter, sortBy, limit, offset,
	)

	rows, err := s.db.QueryContext(ctx, query)
	if err != nil {
		return &SearchUsersResponse{Error: err.Error()}, nil
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		var user User
		if err := rows.Scan(&user.Id, &user.Username, &user.Email, &user.Role); err != nil {
			continue
		}
		users = append(users, &user)
	}

	return &SearchUsersResponse{Users: users, Total: int32(len(users))}, nil
}

// CreateUser creates a new user
func (s *UserServer) CreateUser(ctx context.Context, req *CreateUserRequest) (*UserResponse, error) {
	username := req.Username
	email := req.Email
	password := req.Password
	role := req.Role

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, password, role) VALUES ('%s', '%s', '%s', '%s') RETURNING id",
		username, email, password, role,
	)

	var id string
	err := s.db.QueryRowContext(ctx, query).Scan(&id)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	for key, value := range req.Metadata {
		metaQuery := fmt.Sprintf(
			"INSERT INTO user_metadata (user_id, key, value) VALUES ('%s', '%s', '%s')",
			id, key, value,
		)
		s.db.ExecContext(ctx, metaQuery)
	}

	return &UserResponse{User: &User{Id: id, Username: username, Email: email, Role: role}}, nil
}

// UpdateUser updates an existing user
func (s *UserServer) UpdateUser(ctx context.Context, req *UpdateUserRequest) (*UserResponse, error) {
	userID := req.Id
	username := req.Username
	email := req.Email
	role := req.Role

	query := fmt.Sprintf(
		"UPDATE users SET username = '%s', email = '%s', role = '%s' WHERE id = %s",
		username, email, role, userID,
	)

	_, err := s.db.ExecContext(ctx, query)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	return &UserResponse{User: &User{Id: userID, Username: username, Email: email, Role: role}}, nil
}

// DeleteUser deletes a user
func (s *UserServer) DeleteUser(ctx context.Context, req *DeleteUserRequest) (*DeleteResponse, error) {
	userID := req.Id

	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)
	_, err := s.db.ExecContext(ctx, query)
	if err != nil {
		return &DeleteResponse{Success: false, Error: err.Error()}, nil
	}

	return &DeleteResponse{Success: true}, nil
}

// ExecuteQuery executes an arbitrary SQL query
func (s *UserServer) ExecuteQuery(ctx context.Context, req *QueryRequest) (*QueryResponse, error) {
	sqlQuery := req.Query

	rows, err := s.db.QueryContext(ctx, sqlQuery)
	if err != nil {
		result, execErr := s.db.ExecContext(ctx, sqlQuery)
		if execErr != nil {
			return &QueryResponse{Error: err.Error()}, nil
		}
		affected, _ := result.RowsAffected()
		return &QueryResponse{Affected: int32(affected)}, nil
	}
	defer rows.Close()

	var results [][]byte
	cols, _ := rows.Columns()
	for rows.Next() {
		columns := make([]interface{}, len(cols))
		columnPtrs := make([]interface{}, len(cols))
		for i := range columns {
			columnPtrs[i] = &columns[i]
		}
		rows.Scan(columnPtrs...)

		m := make(map[string]interface{})
		for i, colName := range cols {
			m[colName] = columns[i]
		}
		jsonBytes, _ := json.Marshal(m)
		results = append(results, jsonBytes)
	}

	return &QueryResponse{Rows: results}, nil
}

// RunCommand executes a system command
func (s *UserServer) RunCommand(ctx context.Context, req *CommandRequest) (*CommandResponse, error) {
	command := req.Command
	args := req.Args

	cmd := exec.CommandContext(ctx, command, args...)

	for key, value := range req.Env {
		cmd.Env = append(cmd.Env, fmt.Sprintf("%s=%s", key, value))
	}

	output, err := cmd.CombinedOutput()
	if err != nil {
		return &CommandResponse{Output: string(output), ExitCode: 1, Error: err.Error()}, nil
	}

	return &CommandResponse{Output: string(output), ExitCode: 0}, nil
}

// GetFile retrieves a file by path
func (s *UserServer) GetFile(ctx context.Context, req *FileRequest) (*FileResponse, error) {
	filePath := req.Path
	baseDir := req.Directory
	if baseDir == "" {
		baseDir = "./files"
	}

	fullPath := filepath.Join(baseDir, filePath)

	content, err := os.ReadFile(fullPath)
	if err != nil {
		return &FileResponse{Error: err.Error()}, nil
	}

	return &FileResponse{Content: content, MimeType: "application/octet-stream"}, nil
}

// GenerateReport generates a report with SQL and file output
func (s *UserServer) GenerateReport(ctx context.Context, req *ReportRequest) (*ReportResponse, error) {
	name := req.Name
	sqlQuery := req.Query
	outputPath := req.OutputPath

	rows, err := s.db.QueryContext(ctx, sqlQuery)
	if err != nil {
		return &ReportResponse{Error: err.Error()}, nil
	}
	defer rows.Close()

	var data []map[string]interface{}
	cols, _ := rows.Columns()
	for rows.Next() {
		columns := make([]interface{}, len(cols))
		columnPtrs := make([]interface{}, len(cols))
		for i := range columns {
			columnPtrs[i] = &columns[i]
		}
		rows.Scan(columnPtrs...)

		m := make(map[string]interface{})
		for i, colName := range cols {
			m[colName] = columns[i]
		}
		data = append(data, m)
	}

	fullOutputPath := filepath.Join(outputPath, name+".json")
	content, _ := json.Marshal(data)

	if err := os.WriteFile(fullOutputPath, content, 0644); err != nil {
		return &ReportResponse{Error: err.Error()}, nil
	}

	return &ReportResponse{OutputPath: fullOutputPath, Records: int32(len(data))}, nil
}

// GetUserViaRepo retrieves a user through the repository layer
func (s *UserServer) GetUserViaRepo(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	userID := req.Id

	user, err := s.userRepo.FindByID(userID)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	return &UserResponse{User: &User{
		Id:       user.ID,
		Username: user.Username,
		Email:    user.Email,
		Role:     user.Role,
	}}, nil
}

// SearchViaRepo searches users through the repository layer
func (s *UserServer) SearchViaRepo(ctx context.Context, req *SearchUsersRequest) (*SearchUsersResponse, error) {
	searchTerm := req.Query
	filter := req.Filter

	users, err := s.userRepo.Search(searchTerm, filter)
	if err != nil {
		return &SearchUsersResponse{Error: err.Error()}, nil
	}

	var protoUsers []*User
	for _, u := range users {
		protoUsers = append(protoUsers, &User{
			Id:       u.ID,
			Username: u.Username,
			Email:    u.Email,
			Role:     u.Role,
		})
	}

	return &SearchUsersResponse{Users: protoUsers, Total: int32(len(protoUsers))}, nil
}

// GetUserWithMetadata uses context metadata for authentication
func (s *UserServer) GetUserWithMetadata(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	return s.GetUser(ctx, req)
}

// Placeholder types for proto generation compatibility
type GetUserRequest struct {
	Id       string
	Username string
}

type SearchUsersRequest struct {
	Query  string
	Filter string
	SortBy string
	Limit  int32
	Offset int32
}

type CreateUserRequest struct {
	Username string
	Email    string
	Password string
	Role     string
	Metadata map[string]string
}

type UpdateUserRequest struct {
	Id       string
	Username string
	Email    string
	Role     string
	Metadata map[string]string
}

type DeleteUserRequest struct {
	Id         string
	SoftDelete bool
}

type QueryRequest struct {
	Query  string
	Params []string
}

type CommandRequest struct {
	Command string
	Args    []string
	Env     map[string]string
}

type FileRequest struct {
	Path      string
	Directory string
}

type ReportRequest struct {
	Name       string
	Query      string
	OutputPath string
	Format     string
}

type User struct {
	Id        string
	Username  string
	Email     string
	Role      string
	CreatedAt string
}

type UserResponse struct {
	User  *User
	Error string
}

type SearchUsersResponse struct {
	Users []*User
	Total int32
	Error string
}

type DeleteResponse struct {
	Success bool
	Error   string
}

type QueryResponse struct {
	Rows     [][]byte
	Affected int32
	Error    string
}

type CommandResponse struct {
	Output   string
	ExitCode int32
	Error    string
}

type FileResponse struct {
	Content  []byte
	MimeType string
	Error    string
}

type ReportResponse struct {
	OutputPath string
	Records    int32
	Error      string
}
