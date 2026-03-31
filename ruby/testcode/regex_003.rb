require_relative 'shared'

# vuln-code-snippet start ruby_regex_user_pattern
def search_log(req)
  pattern = req.param('pattern')
  text = req.param('text')
  result = Regexp.new(pattern).match(text)  # vuln-code-snippet vuln-line ruby_regex_user_pattern
  BenchmarkResponse.ok(result ? result[0] : 'no match')
end
# vuln-code-snippet end ruby_regex_user_pattern
