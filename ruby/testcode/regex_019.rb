require_relative 'shared'

# vuln-code-snippet start ruby_regex_user_compile
def match_user_pattern(req)
  pattern = req.param('pattern')
  text = req.param('text')
  re = Regexp.new(pattern) # vuln-code-snippet vuln-line ruby_regex_user_compile
  matched = re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_user_compile
