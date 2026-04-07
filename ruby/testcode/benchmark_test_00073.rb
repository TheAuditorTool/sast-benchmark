require_relative 'shared'

def load_safe_basename(req)
  input = req.param('plugin')
  safe_name = File.basename(input, '.rb')
  load("plugins/#{safe_name}.rb")
  BenchmarkResponse.ok("loaded #{safe_name}")
end
