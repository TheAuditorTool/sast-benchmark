package testcode

import (
	"fmt"
	"net/http"
	"strings"
	"time"
)

func BenchmarkTest00639(w http.ResponseWriter, r *http.Request) {
	customData := r.Header.Get("X-Custom-Data")

	var report strings.Builder
	report.WriteString(fmt.Sprintf("Report generated at: %s\n", time.Now().UTC().Format(time.RFC3339)))
	report.WriteString(fmt.Sprintf("Remote addr: %s\n", r.RemoteAddr))
	report.WriteString("---\n")
	report.WriteString(customData)
	report.WriteString("\n---\n")

	for i := 0; i < 10; i++ {
		report.WriteString(fmt.Sprintf("Processing segment %d of custom data\n", i))
		report.WriteString(customData)
		report.WriteString("\n")
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"report_length": report.Len(),
		"status":        "processed",
	})
}
