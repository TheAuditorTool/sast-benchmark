require_relative 'shared'

def run_build_tool(req)
  tool_path = req.header('X-Tool-Path')
  ENV['PATH'] = tool_path + ":" + ENV['PATH']
  result = `make all`
  BenchmarkResponse.ok(result)
end
