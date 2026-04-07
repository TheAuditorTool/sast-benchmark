require_relative 'shared'

def run_tool(req)
  tool = req.param('tool')
  raise 'invalid tool' unless tool =~ /\A[a-z0-9_]+\z/
  system(tool, '--version')
  BenchmarkResponse.json({ status: 'done' })
end
