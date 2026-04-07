require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

# vuln-code-snippet start ruby_ssti_liquid_strict_mode
def handler(req)
  output = Liquid::Template.parse('Hello {{ name | escape }}').render('name' => req.param('name')) # vuln-code-snippet safe-line ruby_ssti_liquid_strict_mode
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_liquid_strict_mode
