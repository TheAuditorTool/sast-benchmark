require_relative 'shared'
require 'erb'

module Slim
  class Template
    def initialize(opts = {}, &block); @src = block ? block.call : ''; end
    def render(scope = Object.new, locals = {}); @src; end
  end
end

def render_slim(req)
  template_str = req.param('template')
  tmpl = Slim::Template.new { template_str }
  output = tmpl.render
  BenchmarkResponse.html(output)
end
