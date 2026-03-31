require_relative 'shared'

# vuln-code-snippet start ruby_pt_expand_check
def get_document(req)
  base = "/data/reports"
  path = req.param('path')
  resolved = File.expand_path(path, base)
  return BenchmarkResponse.bad_request("Invalid path") unless resolved.start_with?(base + "/") # vuln-code-snippet safe-line ruby_pt_expand_check
  content = File.read(resolved)
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_expand_check
