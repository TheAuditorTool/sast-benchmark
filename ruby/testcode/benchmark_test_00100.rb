require_relative 'shared'
require 'yaml'

def apply_user_config(req)
  config_str = req.body_str
  settings = YAML.load(config_str)
  BenchmarkResponse.ok(settings.to_s)
end
