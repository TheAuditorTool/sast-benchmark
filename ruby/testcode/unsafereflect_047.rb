require_relative 'shared'

SVCMAP = { 1 => -> { 'svc1' }, 2 => -> { 'svc2' } }.freeze

# vuln-code-snippet start ruby_reflect_integer_id_dispatch
def handler(req)
  result = SVCMAP.fetch(req.param('id').to_i).call # vuln-code-snippet safe-line ruby_reflect_integer_id_dispatch
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_reflect_integer_id_dispatch
