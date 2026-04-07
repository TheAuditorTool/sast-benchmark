require_relative 'shared'

begin
  require 'slim'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_slim_template
def handler(req)
  tpl_str = req.param('tpl')
  output = Slim::Template.new { tpl_str }.render # vuln-code-snippet vuln-line ruby_ssti_slim_template
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_slim_template
