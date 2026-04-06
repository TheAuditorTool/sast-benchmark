require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

# vuln-code-snippet start ruby_ssti_liquid_strict
def render_liquid_safe(req)
  name = req.param('name')
  template = Liquid::Template.parse(File.read('templates/greeting.liquid'))
  output = template.render({ 'name' => name }, strict_variables: true) # vuln-code-snippet safe-line ruby_ssti_liquid_strict
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_liquid_strict
