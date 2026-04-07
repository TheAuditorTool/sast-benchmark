require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

# vuln-code-snippet start ruby_ssti_liquid_object_injection
def handler(req)
  tpl = req.param('tpl')
  output = Liquid::Template.parse(tpl).render('x' => req.param('data')) # vuln-code-snippet vuln-line ruby_ssti_liquid_object_injection
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_liquid_object_injection
