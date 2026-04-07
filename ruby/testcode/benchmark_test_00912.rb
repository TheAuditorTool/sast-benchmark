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

def command_pattern_dispatch(req)
  cmd_class = COMMANDS_048.fetch(req.param('cmd'))
  result    = cmd_class.new.execute
  BenchmarkResponse.json({ result: result })
end
