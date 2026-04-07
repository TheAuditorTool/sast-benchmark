require_relative 'shared'

# vuln-code-snippet start ruby_regex_grep_tainted
def handle_grep_tainted(req)
  lines = ['foo bar', 'hello world', 'test case', 'sample data', 'more lines']
  q = req.param('q')
  results = lines.grep(Regexp.new(q)) # vuln-code-snippet vuln-line ruby_regex_grep_tainted
  BenchmarkResponse.json({ results: results })
end
# vuln-code-snippet end ruby_regex_grep_tainted
