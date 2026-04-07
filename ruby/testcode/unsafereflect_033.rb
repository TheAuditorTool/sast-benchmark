require_relative 'shared'

# vuln-code-snippet start ruby_reflect_proc_from_method
def handler(req)
  result = eval("method(:#{req.param('method_name')})").call # vuln-code-snippet vuln-line ruby_reflect_proc_from_method
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_proc_from_method
