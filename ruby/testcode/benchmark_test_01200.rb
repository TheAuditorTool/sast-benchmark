require_relative 'shared'

def yaml_load_file_handler(req)
  path = req.param('config')
  data = YAML.load_file(path)
  BenchmarkResponse.json({ result: data.to_s })
end
