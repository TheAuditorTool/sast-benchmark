require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_frozen_bypass
def execute_snippet(req)
  frozen_input = req.param('code').freeze
  unfrozen = frozen_input.dup
  result = eval(unfrozen) # vuln-code-snippet vuln-line ruby_codeinj_frozen_bypass
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_frozen_bypass
