require_relative 'shared'

# vuln-code-snippet start ruby_deser_yaml_permitted_class_bypass
def yaml_permitted_bypass_handler(req)
  data = req.post('yaml')
  obj = YAML.safe_load(data, permitted_classes: [Object])  # vuln-code-snippet vuln-line ruby_deser_yaml_permitted_class_bypass
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_yaml_permitted_class_bypass
