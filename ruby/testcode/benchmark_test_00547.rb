require_relative 'shared'

def handle_nested_optional(req)
  input = req.param('input')
  result = /^(a?)+$/ =~ input
  BenchmarkResponse.json({ matched: !result.nil? })
end
