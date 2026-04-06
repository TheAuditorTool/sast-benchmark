require_relative 'shared'

# vuln-code-snippet start ruby_regex_escaped_compile
def match_escaped_pattern(req)
  pattern = req.param('pattern')
  text = req.param('text')
  safe_pattern = Regexp.escape(pattern) # vuln-code-snippet safe-line ruby_regex_escaped_compile
  re = Regexp.new(safe_pattern)
  matched = re.match(text)
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_escaped_compile
