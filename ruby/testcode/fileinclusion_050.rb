require_relative 'shared'

SANDBOX_DIR = File.expand_path('sandbox').freeze

# vuln-code-snippet start ruby_fi_sandbox_load
def handler(req)
  candidate = File.realpath(File.join(SANDBOX_DIR, req.param('f') + '.rb'))
  raise 'outside sandbox' unless candidate.start_with?(SANDBOX_DIR) # vuln-code-snippet safe-line ruby_fi_sandbox_load
  load(candidate)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_sandbox_load
