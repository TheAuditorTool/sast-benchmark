require_relative 'shared'

# vuln-code-snippet start ruby_regex_gsub_pattern
def handle_gsub_pattern(req)
  pat = req.param('pat')
  input = req.param('input')
  cleaned = input.gsub(Regexp.new(pat), '') # vuln-code-snippet vuln-line ruby_regex_gsub_pattern
  BenchmarkResponse.ok(cleaned)
end
# vuln-code-snippet end ruby_regex_gsub_pattern
