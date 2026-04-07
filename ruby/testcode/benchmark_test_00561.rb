require_relative 'shared'

ALLOWED_TOOLS = %w[uptime hostname date uname].freeze

def run_tool(req)
  tool = req.param('tool')
  unless ALLOWED_TOOLS.include?(tool)
    return BenchmarkResponse.bad_request("tool not permitted")
  end
  result = `
  BenchmarkResponse.ok(result)
end
