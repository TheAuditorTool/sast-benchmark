require_relative 'shared'

# vuln-code-snippet start ruby_reflect_constantize_rails
def handler(req)
  klass = req.param('class_name').split('::').inject(Object) { |m, n| m.const_get(n) } # vuln-code-snippet vuln-line ruby_reflect_constantize_rails
  BenchmarkResponse.json({ ok: true, name: klass.name })
end
# vuln-code-snippet end ruby_reflect_constantize_rails
