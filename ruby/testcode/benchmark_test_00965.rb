require_relative 'shared'

def handle_grep_tainted(req)
  lines = ['foo bar', 'hello world', 'test case', 'sample data', 'more lines']
  q = req.param('q')
  results = lines.grep(Regexp.new(q))
  BenchmarkResponse.json({ results: results })
end
