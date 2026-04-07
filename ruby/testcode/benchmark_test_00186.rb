require_relative 'shared'

def handler(req)
  message = req.param('message')
  lines = message.split("\n").first(10).join("\n")
  BenchmarkResponse.ok(lines)
end
