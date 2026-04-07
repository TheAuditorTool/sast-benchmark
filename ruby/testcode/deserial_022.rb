require_relative 'shared'

# vuln-code-snippet start ruby_deser_yaml_load_file
def yaml_load_file_handler(req)
  path = req.param('config')
  data = YAML.load_file(path)  # vuln-code-snippet vuln-line ruby_deser_yaml_load_file # TP: vulnerable in Ruby < 3.1 / Psych < 4; marked conservatively
  BenchmarkResponse.json({ result: data.to_s })
end
# vuln-code-snippet end ruby_deser_yaml_load_file
