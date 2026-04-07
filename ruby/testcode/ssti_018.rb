require_relative 'shared'

begin
  require 'haml'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_haml_render_string
def handler(req)
  output = Haml::Engine.new(req.param('template')).render # vuln-code-snippet vuln-line ruby_ssti_haml_render_string
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_haml_render_string
