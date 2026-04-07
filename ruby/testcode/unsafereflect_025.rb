require_relative 'shared'

# vuln-code-snippet start ruby_reflect_binding_eval_method
def handler(req)
  result = binding.eval("#{req.param('klass')}.new.#{req.param('method')}") # vuln-code-snippet vuln-line ruby_reflect_binding_eval_method
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_binding_eval_method
