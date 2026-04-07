require_relative 'shared'

begin
  require 'erb'
rescue LoadError
end

# vuln-code-snippet start ruby_codeinj_erb_new_result
def render_template(req)
  tpl = req.param('tpl')
  renderer = ERB.new(tpl)
  output = renderer.result(binding) # vuln-code-snippet vuln-line ruby_codeinj_erb_new_result
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_codeinj_erb_new_result
