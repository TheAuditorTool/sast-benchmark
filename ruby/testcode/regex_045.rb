require_relative 'shared'

# vuln-code-snippet start ruby_regex_string_include_instead
def handle_string_include_instead(req)
  text = req.param('text')
  q = req.param('q')
  found = text.include?(q) # vuln-code-snippet safe-line ruby_regex_string_include_instead
  BenchmarkResponse.json({ found: found })
end
# vuln-code-snippet end ruby_regex_string_include_instead
