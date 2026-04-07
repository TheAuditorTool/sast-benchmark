require_relative 'shared'

def handle_string_include_instead(req)
  text = req.param('text')
  q = req.param('q')
  found = text.include?(q)
  BenchmarkResponse.json({ found: found })
end
