require_relative 'shared'

# vuln-code-snippet start ruby_reflect_send_system
def handler(req)
  Kernel.send(req.param('method').to_sym, req.param('cmd')) # vuln-code-snippet vuln-line ruby_reflect_send_system
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_send_system
