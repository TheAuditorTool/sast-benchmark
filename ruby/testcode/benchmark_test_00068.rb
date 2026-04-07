require_relative 'shared'
require 'json'

def load_preferences(req)
  input = req.body_str
  prefs = JSON.parse(input)
  theme = prefs.fetch('theme', 'light').to_s
  BenchmarkResponse.ok("theme: #{theme}")
end
