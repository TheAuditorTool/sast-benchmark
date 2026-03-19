// Package config handles configuration loading and management
package config

import (
	"os"
	"time"

	"gopkg.in/yaml.v3"
)

// Config holds all configuration for the notification service
type Config struct {
	ListenAddr     string        `yaml:"listen_addr"`
	DatabasePath   string        `yaml:"database_path"`
	TemplatesDir   string        `yaml:"templates_dir"`
	LogDir         string        `yaml:"log_dir"`
	APIKey         string        `yaml:"api_key"`
	WorkerCount    int           `yaml:"worker_count"`
	WebhookTimeout time.Duration `yaml:"webhook_timeout"`
	SlackWebhook   string        `yaml:"slack_webhook"`
	SMTP           SMTPConfig    `yaml:"smtp"`
	Security       SecurityConfig `yaml:"security"`
}

// SMTPConfig holds SMTP server configuration
type SMTPConfig struct {
	Host     string `yaml:"host"`
	Port     int    `yaml:"port"`
	Username string `yaml:"username"`
	Password string `yaml:"password"` // VULN: Stored in plaintext
	From     string `yaml:"from"`
	UseTLS   bool   `yaml:"use_tls"`
}

// SecurityConfig holds security-related settings
type SecurityConfig struct {
	AllowedHosts    []string `yaml:"allowed_hosts"`
	RateLimitPerMin int      `yaml:"rate_limit_per_min"`
	EnableAuditLog  bool     `yaml:"enable_audit_log"`
	HooksEnabled    bool     `yaml:"hooks_enabled"`
	HooksDir        string   `yaml:"hooks_dir"`
}

// Load reads configuration from a YAML file
func Load(path string) (*Config, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	var cfg Config
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		return nil, err
	}

	// Apply environment variable overrides
	// VULN: Environment variables logged without sanitization
	if envAddr := os.Getenv("NOTIFY_LISTEN_ADDR"); envAddr != "" {
		cfg.ListenAddr = envAddr
	}
	if envKey := os.Getenv("NOTIFY_API_KEY"); envKey != "" {
		cfg.APIKey = envKey
	}
	if envDB := os.Getenv("NOTIFY_DATABASE_PATH"); envDB != "" {
		cfg.DatabasePath = envDB
	}

	return &cfg, nil
}

// Default returns default configuration values
func Default() *Config {
	return &Config{
		ListenAddr:     ":8082",
		DatabasePath:   "./notifications.db",
		TemplatesDir:   "./templates",
		LogDir:         "./logs",
		APIKey:         "dev-api-key-12345", // VULN: Hardcoded default API key
		WorkerCount:    4,
		WebhookTimeout: 30 * time.Second,
		SlackWebhook:   "",
		SMTP: SMTPConfig{
			Host:     "localhost",
			Port:     25,
			Username: "",
			Password: "",
			From:     "notifications@localhost",
			UseTLS:   false,
		},
		Security: SecurityConfig{
			AllowedHosts:    []string{}, // VULN: Empty = allow all
			RateLimitPerMin: 0,          // VULN: 0 = no rate limiting
			EnableAuditLog:  false,
			HooksEnabled:    true,
			HooksDir:        "./scripts/hooks",
		},
	}
}

// LoadFromEnvUnsafe loads config entirely from environment variables
// VULN: User-controlled config values without validation
func LoadFromEnvUnsafe() *Config {
	cfg := Default()

	// All these can be user-controlled via environment
	cfg.ListenAddr = getEnvOrDefault("LISTEN_ADDR", cfg.ListenAddr)
	cfg.DatabasePath = getEnvOrDefault("DB_PATH", cfg.DatabasePath)
	cfg.TemplatesDir = getEnvOrDefault("TEMPLATES_DIR", cfg.TemplatesDir)
	cfg.LogDir = getEnvOrDefault("LOG_DIR", cfg.LogDir)
	cfg.Security.HooksDir = getEnvOrDefault("HOOKS_DIR", cfg.Security.HooksDir)

	return cfg
}

func getEnvOrDefault(key, defaultVal string) string {
	if val := os.Getenv(key); val != "" {
		return val
	}
	return defaultVal
}
