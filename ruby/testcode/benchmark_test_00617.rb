require_relative 'shared'

def psych_load_stream_handler(req)
  yaml_input = req.post('yaml')
  obj = Psych.load_stream(yaml_input).first
  BenchmarkResponse.json({ result: obj.to_s })
end
