require_relative 'shared'

# vuln-code-snippet start ruby_deser_psych_load_doc
def psych_load_stream_handler(req)
  yaml_input = req.post('yaml')
  obj = Psych.load_stream(yaml_input).first  # vuln-code-snippet vuln-line ruby_deser_psych_load_doc
  BenchmarkResponse.json({ result: obj.to_s })
end
# vuln-code-snippet end ruby_deser_psych_load_doc
