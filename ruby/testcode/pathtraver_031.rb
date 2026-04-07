require_relative 'shared'
require 'erb'

# vuln-code-snippet start ruby_pt_erb_template_path
def render_erb_template(req)
  tpl_dir = '/app/templates'
  content = File.read(File.join(tpl_dir, req.param('tpl'))) # vuln-code-snippet vuln-line ruby_pt_erb_template_path
  ERB.new(content).result
  BenchmarkResponse.html(ERB.new(content).result)
end
# vuln-code-snippet end ruby_pt_erb_template_path
