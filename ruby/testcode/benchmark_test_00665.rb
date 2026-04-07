require_relative 'shared'

def search_log(req)
  pattern = req.param('pattern')
  text = req.param('text')
  result = Regexp.new(pattern).match(text)
  BenchmarkResponse.ok(result ? result[0] : 'no match')
end
