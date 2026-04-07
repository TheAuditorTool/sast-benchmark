require_relative 'shared'

def handle_compile_loop(req)
  patterns = req.param('patterns').split(',')
  text = req.param('text')
  results = patterns.map { |p| Regexp.new(p).match(text) }
  BenchmarkResponse.json({ count: results.compact.length })
end
