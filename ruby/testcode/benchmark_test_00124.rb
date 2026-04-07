require_relative 'shared'

module Liquid
  class Template
    def self.parse(source); new(source); end
    def initialize(source); @source = source; end
    def render(assigns = {}, opts = {}); @source; end
  end
end

def handler(req)
  tpl = req.param('tpl')
  output = Liquid::Template.parse(tpl).render('x' => req.param('data'))
  BenchmarkResponse.html(output)
end
