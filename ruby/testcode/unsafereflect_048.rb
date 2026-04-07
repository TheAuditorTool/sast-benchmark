require_relative 'shared'

COMMANDS = { 'create' => -> { 'created' }, 'delete' => -> { 'deleted' } }.freeze

# vuln-code-snippet start ruby_reflect_command_bus
def handler(req)
  result = COMMANDS.fetch(req.param('cmd')).call # vuln-code-snippet safe-line ruby_reflect_command_bus
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_reflect_command_bus
