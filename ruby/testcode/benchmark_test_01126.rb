require_relative 'shared'
require 'yaml'
require 'date'

def load_schedule(req)
  input = req.body_str
  data = YAML.safe_load(input, permitted_classes: [Symbol, Date])
  event = data.fetch('event', 'none').to_s
  BenchmarkResponse.ok("event: #{event}")
end
