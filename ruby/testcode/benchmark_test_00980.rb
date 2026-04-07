require_relative 'shared'

def handle_union_tainted(req)
  raw = req.param('patterns')
  parts = raw.split(',')
  re = Regexp.union(parts)
  BenchmarkResponse.json({ matched: re.match?(req.param('input')) })
end
