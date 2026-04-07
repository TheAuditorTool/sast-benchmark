require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

def handler(req)
  output = Liquid::Template.parse('Hello {{ name | escape }}').render('name' => req.param('name'))
  BenchmarkResponse.html(output)
end
