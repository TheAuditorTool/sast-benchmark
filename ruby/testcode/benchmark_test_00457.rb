require_relative 'shared'
require 'erb'

def render_erb_template(req)
  tpl_dir = '/app/templates'
  content = File.read(File.join(tpl_dir, req.param('tpl')))
  ERB.new(content).result
  BenchmarkResponse.html(ERB.new(content).result)
end
