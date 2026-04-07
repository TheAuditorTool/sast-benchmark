require_relative 'shared'

# vuln-code-snippet start ruby_regex_escape_user_pattern
def handle_escape_user_pattern(req)
  q = req.param('q')
  input = req.param('input')
  result = Regexp.new(Regexp.escape(q)).match(input) # vuln-code-snippet safe-line ruby_regex_escape_user_pattern
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_escape_user_pattern
