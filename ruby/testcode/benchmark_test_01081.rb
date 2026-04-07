require_relative 'shared'

def handle_escape_user_pattern(req)
  q = req.param('q')
  input = req.param('input')
  result = Regexp.new(Regexp.escape(q)).match(input)
  BenchmarkResponse.json({ matched: !result.nil? })
end
