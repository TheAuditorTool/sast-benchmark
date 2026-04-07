require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

FIXED_TPL = 'Hello {{ name }}'.freeze

# vuln-code-snippet start ruby_ssti_liquid_fixed_tpl
def handler(req)
  output = Liquid::Template.parse(FIXED_TPL).render('name' => req.param('name')) # vuln-code-snippet safe-line ruby_ssti_liquid_fixed_tpl
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_liquid_fixed_tpl
