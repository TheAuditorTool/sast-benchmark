require_relative 'shared'

# vuln-code-snippet start ruby_csrf_json_no_token
def api_update(req)
  data = JSON.parse(req.body_str) rescue {}
  BenchmarkResponse.json({ updated: data }) # vuln-code-snippet vuln-line ruby_csrf_json_no_token
end
# vuln-code-snippet end ruby_csrf_json_no_token
