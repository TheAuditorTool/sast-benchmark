require_relative 'shared'
require 'erb'

module Erubi
  class Engine
    def initialize(input); @src = input; end
    def src; @src; end
  end
end

# vuln-code-snippet start ruby_ssti_erubi
def render_erubi(req)
  template_str = req.param('template')
  engine = Erubi::Engine.new(template_str) # vuln-code-snippet vuln-line ruby_ssti_erubi
  BenchmarkResponse.html(engine.src)
end
# vuln-code-snippet end ruby_ssti_erubi
