require_relative 'shared'

def yaml_permitted_bypass_handler(req)
  data = req.post('yaml')
  obj = YAML.safe_load(data, permitted_classes: [Object])
  BenchmarkResponse.json({ result: obj.to_s })
end
