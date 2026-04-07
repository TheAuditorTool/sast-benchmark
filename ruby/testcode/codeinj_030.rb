require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_tap_eval
def setup_object(req)
  setup_code = req.param('setup')
  obj = Object.new.tap { |o| o.instance_eval(setup_code) } # vuln-code-snippet vuln-line ruby_codeinj_tap_eval
  BenchmarkResponse.json({ object_id: obj.object_id })
end
# vuln-code-snippet end ruby_codeinj_tap_eval
