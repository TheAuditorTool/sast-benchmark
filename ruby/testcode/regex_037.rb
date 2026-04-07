require_relative 'shared'

# vuln-code-snippet start ruby_regex_per_object_timeout
def handle_per_object_timeout(req)
  pat = req.param('pat')
  input = req.param('input')
  re = Regexp.new(pat, timeout: 0.5) # Ruby 3.2+ # vuln-code-snippet safe-line ruby_regex_per_object_timeout
  BenchmarkResponse.json({ matched: re.match?(input) })
end
# vuln-code-snippet end ruby_regex_per_object_timeout
