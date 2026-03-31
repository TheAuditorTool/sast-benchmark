require_relative 'shared'

# vuln-code-snippet start ruby_pt_sanitize_dotdot
def fetch_resource(req)
  raw = req.param('path')
  sanitized = raw.gsub('..', '').gsub(/\A\/+/, '')
  content = File.read("/resources/#{sanitized}") # vuln-code-snippet safe-line ruby_pt_sanitize_dotdot
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_sanitize_dotdot
