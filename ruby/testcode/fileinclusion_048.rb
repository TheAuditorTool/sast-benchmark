require_relative 'shared'

# All file loading happens at boot time via constants; handler only dispatches
HANDLER_MAP = {
  'greet' => method(:puts),
}.freeze

# vuln-code-snippet start ruby_fi_no_load_in_handler
def handler(req)
  action = HANDLER_MAP.fetch(req.param('action'), method(:puts))
  action.call('ok') # vuln-code-snippet safe-line ruby_fi_no_load_in_handler
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_no_load_in_handler
