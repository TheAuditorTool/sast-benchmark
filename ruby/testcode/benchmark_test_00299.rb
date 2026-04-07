require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}); @source; end
  end
end

def render_liquid(req)
  template_str = req.param('template')
  template = Liquid::Template.parse(template_str)
  output = template.render('name' => req.param('name'))
  BenchmarkResponse.html(output)
end
