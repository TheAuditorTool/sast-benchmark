require_relative 'shared'

# vuln-code-snippet start ruby_regex_timeout
def validate_input_timeout(req)
  text = req.param('text')
  safe_re = Regexp.new('(a+)+$', timeout: 1.0)  # vuln-code-snippet safe-line ruby_regex_timeout
  matched = safe_re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_timeout
