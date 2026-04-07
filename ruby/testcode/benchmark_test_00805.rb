require_relative 'shared'
require 'yaml'

def import_data(req)
  data = req.body_str
  parsed = YAML.unsafe_load(data)
  BenchmarkResponse.ok(parsed.to_s)
end
