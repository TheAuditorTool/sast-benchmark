require_relative 'shared'

class CreateCommand048
  def execute; "created"; end
end

class DeleteCommand048
  def execute; "deleted"; end
end

COMMANDS_048 = {
  'create' => CreateCommand048,
  'delete' => DeleteCommand048
}.freeze

# vuln-code-snippet start ruby_dynmethod_command_pattern
def command_pattern_dispatch(req)
  cmd_class = COMMANDS_048.fetch(req.param('cmd')) # vuln-code-snippet safe-line ruby_dynmethod_command_pattern
  result    = cmd_class.new.execute
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_command_pattern
