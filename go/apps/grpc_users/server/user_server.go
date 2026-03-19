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
// This demonstrates taint sources from protobuf message fields
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

// ===============================================
// gRPC TAINT SOURCES:
// All fields in request messages are potential taint sources
// req.FieldName -> tainted data
// ===============================================

// GetUser retrieves a user by ID
// TAINT SOURCE: req.Id, req.Username (protobuf message fields)
func (s *UserServer) GetUser(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	var query string

	// TAINT SOURCE: gRPC request field
	if req.Id != "" {
		// TAINT PROPAGATION: Message field to SQL
		query = fmt.Sprintf("SELECT * FROM users WHERE id = %s", req.Id)
	} else if req.Username != "" {
		// TAINT PROPAGATION: Another message field
		query = fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", req.Username)
	} else {
		return &UserResponse{Error: "id or username required"}, nil
	}

	// TAINT SINK: SQL injection
	row := s.db.QueryRowContext(ctx, query)
	var user User
	err := row.Scan(&user.Id, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	return &UserResponse{User: &user}, nil
}

// SearchUsers searches for users with filters
// TAINT SOURCE: req.Query, req.Filter, req.SortBy (multiple fields)
func (s *UserServer) SearchUsers(ctx context.Context, req *SearchUsersRequest) (*SearchUsersResponse, error) {
	// TAINT SOURCES: Multiple gRPC message fields
	searchQuery := req.Query
	filter := req.Filter
	sortBy := req.SortBy
	limit := req.Limit
	offset := req.Offset

	// TAINT PROPAGATION: All fields flow to SQL
	// Multiple injection points in single query
	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND %s ORDER BY %s LIMIT %d OFFSET %d",
		searchQuery, filter, sortBy, limit, offset,
	)

	// TAINT SINK: SQL injection (5 injection points!)
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
// TAINT SOURCE: All CreateUserRequest fields
func (s *UserServer) CreateUser(ctx context.Context, req *CreateUserRequest) (*UserResponse, error) {
	// TAINT SOURCES: Multiple message fields
	username := req.Username
	email := req.Email
	password := req.Password
	role := req.Role

	// TAINT PROPAGATION: Fields to SQL
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, password, role) VALUES ('%s', '%s', '%s', '%s') RETURNING id",
		username, email, password, role,
	)

	// TAINT SINK: SQL injection
	var id string
	err := s.db.QueryRowContext(ctx, query).Scan(&id)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	// Handle metadata map (each key-value is tainted)
	for key, value := range req.Metadata {
		// TAINT SINK: SQL injection from map values
		metaQuery := fmt.Sprintf(
			"INSERT INTO user_metadata (user_id, key, value) VALUES ('%s', '%s', '%s')",
			id, key, value,
		)
		s.db.ExecContext(ctx, metaQuery)
	}

	return &UserResponse{User: &User{Id: id, Username: username, Email: email, Role: role}}, nil
}

// UpdateUser updates an existing user
// TAINT SOURCE: UpdateUserRequest fields
func (s *UserServer) UpdateUser(ctx context.Context, req *UpdateUserRequest) (*UserResponse, error) {
	// TAINT SOURCES: Message fields
	userID := req.Id
	username := req.Username
	email := req.Email
	role := req.Role

	// TAINT PROPAGATION: Multiple fields in UPDATE
	query := fmt.Sprintf(
		"UPDATE users SET username = '%s', email = '%s', role = '%s' WHERE id = %s",
		username, email, role, userID,
	)

	// TAINT SINK: SQL injection
	_, err := s.db.ExecContext(ctx, query)
	if err != nil {
		return &UserResponse{Error: err.Error()}, nil
	}

	return &UserResponse{User: &User{Id: userID, Username: username, Email: email, Role: role}}, nil
}

// DeleteUser deletes a user
// TAINT SOURCE: req.Id
func (s *UserServer) DeleteUser(ctx context.Context, req *DeleteUserRequest) (*DeleteResponse, error) {
	// TAINT SOURCE: Message field
	userID := req.Id

	// TAINT PROPAGATION -> SINK
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)
	_, err := s.db.ExecContext(ctx, query)
	if err != nil {
		return &DeleteResponse{Success: false, Error: err.Error()}, nil
	}

	return &DeleteResponse{Success: true}, nil
}

// ===============================================
// DANGEROUS ADMIN OPERATIONS
// ===============================================

// ExecuteQuery executes arbitrary SQL
// TAINT SOURCE: req.Query (direct SQL execution)
func (s *UserServer) ExecuteQuery(ctx context.Context, req *QueryRequest) (*QueryResponse, error) {
	// TAINT SOURCE: SQL query from gRPC message
	// This is direct SQL injection - arbitrary query execution
	sqlQuery := req.Query

	// TAINT SINK: Direct SQL execution
	rows, err := s.db.QueryContext(ctx, sqlQuery)
	if err != nil {
		// Try as exec if query fails
		result, execErr := s.db.ExecContext(ctx, sqlQuery)
		if execErr != nil {
			return &QueryResponse{Error: err.Error()}, nil
		}
		affected, _ := result.RowsAffected()
		return &QueryResponse{Affected: int32(affected)}, nil
	}
	defer rows.Close()

	// Collect results as JSON
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
// TAINT SOURCE: req.Command, req.Args, req.Env
func (s *UserServer) RunCommand(ctx context.Context, req *CommandRequest) (*CommandResponse, error) {
	// TAINT SOURCES: gRPC message fields
	command := req.Command
	args := req.Args

	// TAINT SINK: Command injection - non-literal command
	cmd := exec.CommandContext(ctx, command, args...)

	// TAINT SOURCE: Environment variables from message
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
// TAINT SOURCE: req.Path
func (s *UserServer) GetFile(ctx context.Context, req *FileRequest) (*FileResponse, error) {
	// TAINT SOURCE: File path from gRPC message
	filePath := req.Path
	baseDir := req.Directory
	if baseDir == "" {
		baseDir = "./files"
	}

	// TAINT PROPAGATION: Path construction
	fullPath := filepath.Join(baseDir, filePath)

	// TAINT SINK: Path traversal
	content, err := os.ReadFile(fullPath)
	if err != nil {
		return &FileResponse{Error: err.Error()}, nil
	}

	return &FileResponse{Content: content, MimeType: "application/octet-stream"}, nil
}

// GenerateReport generates a report with SQL and file output
// TAINT SOURCE: req.Name, req.Query, req.OutputPath
// Multiple sinks from gRPC message
func (s *UserServer) GenerateReport(ctx context.Context, req *ReportRequest) (*ReportResponse, error) {
	// TAINT SOURCES: Multiple message fields
	name := req.Name
	sqlQuery := req.Query
	outputPath := req.OutputPath

	// TAINT SINK 1: SQL injection via query parameter
	rows, err := s.db.QueryContext(ctx, sqlQuery)
	if err != nil {
		return &ReportResponse{Error: err.Error()}, nil
	}
	defer rows.Close()

	// Collect data
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

	// TAINT SINK 2: Path traversal via outputPath
	fullOutputPath := filepath.Join(outputPath, name+".json")
	content, _ := json.Marshal(data)

	// TAINT SINK: Arbitrary file write
	if err := os.WriteFile(fullOutputPath, content, 0644); err != nil {
		return &ReportResponse{Error: err.Error()}, nil
	}

	return &ReportResponse{OutputPath: fullOutputPath, Records: int32(len(data))}, nil
}

// ===============================================
// MULTI-HOP: gRPC Handler -> Repository
// ===============================================

// GetUserViaRepo demonstrates multi-hop flow
// TAINT FLOW: gRPC Message -> Server -> Repository -> SQL
func (s *UserServer) GetUserViaRepo(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	// TAINT SOURCE: gRPC message field
	userID := req.Id

	// TAINT HOP 1: Server -> Repository
	user, err := s.userRepo.FindByIDVulnerable(userID)
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

// SearchViaRepo demonstrates multi-hop search
func (s *UserServer) SearchViaRepo(ctx context.Context, req *SearchUsersRequest) (*SearchUsersResponse, error) {
	// TAINT SOURCE: gRPC message fields
	searchTerm := req.Query
	filter := req.Filter

	// TAINT HOP 1: Server -> Repository
	users, err := s.userRepo.SearchVulnerable(searchTerm, filter)
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

// ===============================================
// METADATA FROM CONTEXT
// gRPC metadata (headers) can also be tainted
// ===============================================

// GetUserWithMetadata uses context metadata
func (s *UserServer) GetUserWithMetadata(ctx context.Context, req *GetUserRequest) (*UserResponse, error) {
	// TAINT SOURCE: gRPC metadata (similar to HTTP headers)
	// In real code: md, _ := metadata.FromIncomingContext(ctx)
	// apiKey := md.Get("x-api-key")[0]

	// For demonstration, assume metadata is passed through
	// This would be vulnerable to SQL injection via header

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
