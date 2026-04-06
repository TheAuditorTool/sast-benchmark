require_relative 'shared'

# vuln-code-snippet start ruby_fi_basename_strip
def load_safe_basename(req)
  input = req.param('plugin')
  safe_name = File.basename(input, '.rb') # vuln-code-snippet safe-line ruby_fi_basename_strip
  load("plugins/#{safe_name}.rb")
  BenchmarkResponse.ok("loaded #{safe_name}")
end
# vuln-code-snippet end ruby_fi_basename_strip
