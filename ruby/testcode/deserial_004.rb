require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_deser_json_parse
def load_preferences(req)
  input = req.body_str
  prefs = JSON.parse(input) # vuln-code-snippet safe-line ruby_deser_json_parse
  theme = prefs.fetch('theme', 'light').to_s
  BenchmarkResponse.ok("theme: #{theme}")
end
# vuln-code-snippet end ruby_deser_json_parse
