require_relative 'shared'

def handle_per_object_timeout(req)
  pat = req.param('pat')
  input = req.param('input')
  re = Regexp.new(pat, timeout: 0.5)
  BenchmarkResponse.json({ matched: re.match?(input) })
end
