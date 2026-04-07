require_relative 'shared'
require 'yaml'

def load_config(req)
  input = req.body_str
  config = YAML.safe_load(input)
  name = config.fetch('name', 'default').to_s
  BenchmarkResponse.ok("config loaded: #{name}")
end
