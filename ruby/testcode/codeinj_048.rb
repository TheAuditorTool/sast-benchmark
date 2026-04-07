require_relative 'shared'

begin
  require 'dry-logic'
rescue LoadError
end

# vuln-code-snippet start ruby_codeinj_dry_logic
def validate_integer_param(req)
  raw = req.param('val')
  rule = Dry::Logic::Rule::Predicate.build(->(v) { v.is_a?(Integer) })
  result = rule.call(Integer(raw)) # vuln-code-snippet safe-line ruby_codeinj_dry_logic
  BenchmarkResponse.json({ valid: result.success? })
end
# vuln-code-snippet end ruby_codeinj_dry_logic
