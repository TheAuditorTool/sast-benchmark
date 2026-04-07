require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

FIXED_TPL = 'Hello {{ name }}'.freeze

def handler(req)
  output = Liquid::Template.parse(FIXED_TPL).render('name' => req.param('name'))
  BenchmarkResponse.html(output)
end
