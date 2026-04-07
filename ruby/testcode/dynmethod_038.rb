require_relative 'shared'

HANDLERS_038 = {
  'read' => -> { 'reading' },
  'list' => -> { 'listing' }
}.freeze

# vuln-code-snippet start ruby_dynmethod_hash_of_procs
def hash_proc_dispatch(req)
  handler = HANDLERS_038.fetch(req.param('action')) # vuln-code-snippet safe-line ruby_dynmethod_hash_of_procs
  result  = handler.call
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_hash_of_procs
