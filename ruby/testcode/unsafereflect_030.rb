require_relative 'shared'

# vuln-code-snippet start ruby_reflect_objectspace_find
def handler(req)
  klass = ObjectSpace.each_object(Class).find { |c| c.name == req.param('klass') } # vuln-code-snippet vuln-line ruby_reflect_objectspace_find
  instance = klass&.new
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_objectspace_find
