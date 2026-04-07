require_relative 'shared'

begin
  require 'tilt'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_tilt_new_erb
def handler(req)
  tpl_str = req.param('template')
  output = Tilt['erb'].new { tpl_str }.render(Object.new) # vuln-code-snippet vuln-line ruby_ssti_tilt_new_erb
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_tilt_new_erb
