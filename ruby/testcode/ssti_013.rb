require_relative 'shared'
require 'erb'

module Slim
  class Template
    def initialize(opts = {}, &block); @src = block ? block.call : ''; end
    def render(scope = Object.new, locals = {}); @src; end
  end
end

# vuln-code-snippet start ruby_ssti_slim_user
def render_slim(req)
  template_str = req.param('template')
  tmpl = Slim::Template.new { template_str } # vuln-code-snippet vuln-line ruby_ssti_slim_user
  output = tmpl.render
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_slim_user
