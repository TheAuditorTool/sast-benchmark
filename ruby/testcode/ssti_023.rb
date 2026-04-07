require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_tilt_haml
def handler(req)
  tpl_str = req.param('haml_tpl')
  output = Tilt['haml'].new { tpl_str }.render # vuln-code-snippet vuln-line ruby_ssti_tilt_haml
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_tilt_haml
