require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_delete
def invalidate_cache_entry(req)
  key = req.param('key')
  File.delete("/cache/#{key}") # vuln-code-snippet vuln-line ruby_pt_file_delete
  BenchmarkResponse.ok("Cache entry removed")
end
# vuln-code-snippet end ruby_pt_file_delete
