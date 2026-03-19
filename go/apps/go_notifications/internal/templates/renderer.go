// Package templates handles template rendering for notifications
package templates

import (
	"bytes"
	"fmt"
	htmltemplate "html/template"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	texttemplate "text/template"
)

// Renderer handles template loading and rendering
type Renderer struct {
	templatesDir string
	cache        map[string]*htmltemplate.Template
}

// NewRenderer creates a new template renderer
func NewRenderer(templatesDir string) *Renderer {
	return &Renderer{
		templatesDir: templatesDir,
		cache:        make(map[string]*htmltemplate.Template),
	}
}

// Render renders a template with the provided data
// VULN: Template path traversal and template injection
func (r *Renderer) Render(templateName string, data map[string]interface{}) (string, error) {
	// VULN: Path traversal - templateName not sanitized
	// templateName could be "../../../etc/passwd" or "../../secrets/config.yaml"
	templatePath := filepath.Join(r.templatesDir, templateName) // TAINT SINK

	// Try to load from cache first
	tmpl, ok := r.cache[templateName]
	if !ok {
		// Load template from file
		// VULN: Arbitrary file read via path traversal
		content, err := os.ReadFile(templatePath) // TAINT SINK
		if err != nil {
			return "", fmt.Errorf("failed to load template %s: %w", templateName, err)
		}

		// Parse template
		// VULN: User-controlled template content if file can be written elsewhere
		tmpl, err = htmltemplate.New(templateName).Funcs(r.unsafeFuncMap()).Parse(string(content))
		if err != nil {
			return "", fmt.Errorf("failed to parse template: %w", err)
		}

		r.cache[templateName] = tmpl
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil { // TAINT SINK: User data in template
		return "", fmt.Errorf("failed to execute template: %w", err)
	}

	return buf.String(), nil
}

// RenderString renders a template from a string
// VULN: Server-Side Template Injection (SSTI)
func (r *Renderer) RenderString(templateStr string, data map[string]interface{}) (string, error) {
	// VULN: SSTI - User-controlled template string
	// templateStr could contain: {{exec "id"}} or {{readFile "/etc/passwd"}}
	tmpl, err := htmltemplate.New("inline").Funcs(r.unsafeFuncMap()).Parse(templateStr) // TAINT SINK
	if err != nil {
		return "", err
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		return "", err
	}

	return buf.String(), nil
}

// RenderText renders using text/template (no HTML escaping)
// VULN: XSS when output is used in HTML context
func (r *Renderer) RenderText(templateStr string, data map[string]interface{}) (string, error) {
	// VULN: text/template doesn't escape HTML - XSS risk
	tmpl, err := texttemplate.New("text").Funcs(r.unsafeTextFuncMap()).Parse(templateStr)
	if err != nil {
		return "", err
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		return "", err
	}

	return buf.String(), nil
}

// unsafeFuncMap returns template functions including dangerous ones
// VULN: Dangerous template functions
func (r *Renderer) unsafeFuncMap() htmltemplate.FuncMap {
	return htmltemplate.FuncMap{
		// VULN: Command execution from template
		"exec": func(cmd string, args ...string) string {
			output, _ := exec.Command(cmd, args...).Output() // TAINT SINK
			return string(output)
		},

		// VULN: Arbitrary file read from template
		"readFile": func(path string) string {
			content, _ := os.ReadFile(path) // TAINT SINK
			return string(content)
		},

		// VULN: Environment variable access
		"env": func(key string) string {
			return os.Getenv(key)
		},

		// VULN: Shell command execution
		"shell": func(cmd string) string {
			output, _ := exec.Command("sh", "-c", cmd).Output() // TAINT SINK
			return string(output)
		},

		// VULN: Include other templates (path traversal)
		"include": func(path string) string {
			content, _ := os.ReadFile(filepath.Join(r.templatesDir, path))
			return string(content)
		},

		// VULN: Write to file from template
		"writeFile": func(path, content string) string {
			os.WriteFile(path, []byte(content), 0644) // TAINT SINK
			return ""
		},

		// String manipulation (safe)
		"upper":    strings.ToUpper,
		"lower":    strings.ToLower,
		"trim":     strings.TrimSpace,
		"replace":  strings.Replace,
		"contains": strings.Contains,

		// VULN: URL for SSRF
		"fetch": func(url string) string {
			// Would fetch URL content - SSRF
			return fmt.Sprintf("[fetch: %s]", url)
		},

		// VULN: SQL query from template (hypothetical)
		"query": func(sql string) string {
			return fmt.Sprintf("[query: %s]", sql)
		},
	}
}

// unsafeTextFuncMap for text/template
func (r *Renderer) unsafeTextFuncMap() texttemplate.FuncMap {
	return texttemplate.FuncMap{
		"exec": func(cmd string, args ...string) string {
			output, _ := exec.Command(cmd, args...).Output()
			return string(output)
		},
		"readFile": func(path string) string {
			content, _ := os.ReadFile(path)
			return string(content)
		},
		"env":   os.Getenv,
		"shell": func(cmd string) string {
			output, _ := exec.Command("sh", "-c", cmd).Output()
			return string(output)
		},
	}
}

// RenderWithIncludes renders a template that can include other templates
// VULN: Path traversal via include directives
func (r *Renderer) RenderWithIncludes(templateName string, data map[string]interface{}) (string, error) {
	// Load main template
	mainPath := filepath.Join(r.templatesDir, templateName)
	mainContent, err := os.ReadFile(mainPath)
	if err != nil {
		return "", err
	}

	// Process include directives
	// VULN: No sanitization of included paths
	content := string(mainContent)
	content = r.processIncludes(content)

	// Parse and execute
	tmpl, err := htmltemplate.New(templateName).Funcs(r.unsafeFuncMap()).Parse(content)
	if err != nil {
		return "", err
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		return "", err
	}

	return buf.String(), nil
}

// processIncludes handles {{include "path"}} directives
// VULN: Path traversal in includes
func (r *Renderer) processIncludes(content string) string {
	// Simple include processing (vulnerable)
	// Looks for {{include "..."}} and replaces with file content
	for strings.Contains(content, "{{include") {
		start := strings.Index(content, "{{include")
		if start == -1 {
			break
		}

		end := strings.Index(content[start:], "}}")
		if end == -1 {
			break
		}

		directive := content[start : start+end+2]

		// Extract path from directive
		pathStart := strings.Index(directive, "\"")
		pathEnd := strings.LastIndex(directive, "\"")
		if pathStart == -1 || pathEnd == -1 || pathStart == pathEnd {
			break
		}

		includePath := directive[pathStart+1 : pathEnd]

		// VULN: Path traversal - includePath not sanitized
		fullPath := filepath.Join(r.templatesDir, includePath)
		includeContent, err := os.ReadFile(fullPath) // TAINT SINK
		if err != nil {
			includeContent = []byte(fmt.Sprintf("[include error: %s]", err))
		}

		content = strings.Replace(content, directive, string(includeContent), 1)
	}

	return content
}

// CompileTemplate compiles a template and stores it
// VULN: User-controlled template stored
func (r *Renderer) CompileTemplate(name, content string) error {
	tmpl, err := htmltemplate.New(name).Funcs(r.unsafeFuncMap()).Parse(content)
	if err != nil {
		return err
	}

	r.cache[name] = tmpl
	return nil
}

// SaveTemplate saves a template to the filesystem
// VULN: Arbitrary file write via template path
func (r *Renderer) SaveTemplate(name, content string) error {
	// VULN: Path traversal in template name
	path := filepath.Join(r.templatesDir, name) // TAINT SINK

	// Ensure directory exists
	dir := filepath.Dir(path)
	os.MkdirAll(dir, 0755)

	return os.WriteFile(path, []byte(content), 0644) // TAINT SINK
}

// ListTemplates lists available templates
// VULN: Information disclosure
func (r *Renderer) ListTemplates() ([]string, error) {
	var templates []string

	err := filepath.Walk(r.templatesDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			relPath, _ := filepath.Rel(r.templatesDir, path)
			templates = append(templates, relPath)
		}
		return nil
	})

	return templates, err
}

// EvalExpression evaluates an expression in template context
// VULN: Code injection via expression
func (r *Renderer) EvalExpression(expr string, data map[string]interface{}) (interface{}, error) {
	// Wrap expression in template
	templateStr := fmt.Sprintf("{{%s}}", expr)

	result, err := r.RenderString(templateStr, data)
	if err != nil {
		return nil, err
	}

	return result, nil
}
