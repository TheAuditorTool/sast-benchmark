require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}); @source; end
  end
end

# vuln-code-snippet start ruby_ssti_liquid_user
def render_liquid(req)
  template_str = req.param('template')
  template = Liquid::Template.parse(template_str) # vuln-code-snippet vuln-line ruby_ssti_liquid_user
  output = template.render('name' => req.param('name'))
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_liquid_user
