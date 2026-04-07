require_relative 'shared'

TYPE_MAP = { 'string' => -> { String.new }, 'int' => -> { 0 } }.freeze

# vuln-code-snippet start ruby_reflect_typed_dispatch
def handler(req)
  result = TYPE_MAP.fetch(req.param('type')).call # vuln-code-snippet safe-line ruby_reflect_typed_dispatch
  BenchmarkResponse.json({ result: result.class.name })
end
# vuln-code-snippet end ruby_reflect_typed_dispatch
