require_relative 'shared'

# vuln-code-snippet start ruby_reflect_ancestors_send
def handler(req)
  mod = Object.ancestors.find { |m| m.name == req.param('mod') } # vuln-code-snippet vuln-line ruby_reflect_ancestors_send
  result = mod&.send(req.param('method').to_sym)
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_ancestors_send
