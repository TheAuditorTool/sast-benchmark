require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_define_from_string
def dynamic_action(req)
  body = req.param('body')
  klass = Class.new do
    define_method(:action) { eval(body) } # vuln-code-snippet vuln-line ruby_codeinj_define_from_string
  end
  result = klass.new.action
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_define_from_string
