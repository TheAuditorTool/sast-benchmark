require_relative 'shared'

def update_settings(req)
  settings = req.post('settings')
  BenchmarkResponse.ok("updated: #{settings}")
end
