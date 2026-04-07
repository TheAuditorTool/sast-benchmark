require_relative 'shared'

PROCS = { 'export' => -> { 'done' }, 'report' => -> { 'generated' } }.freeze

# vuln-code-snippet start ruby_reflect_proc_registry
def handler(req)
  result = PROCS.fetch(req.param('action')).call # vuln-code-snippet safe-line ruby_reflect_proc_registry
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_reflect_proc_registry
