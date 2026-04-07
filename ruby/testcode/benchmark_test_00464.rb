require_relative 'shared'

begin
  require 'dry-logic'
rescue LoadError
end

def validate_integer_param(req)
  raw = req.param('val')
  rule = Dry::Logic::Rule::Predicate.build(->(v) { v.is_a?(Integer) })
  result = rule.call(Integer(raw))
  BenchmarkResponse.json({ valid: result.success? })
end
