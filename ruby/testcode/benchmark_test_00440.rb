require_relative 'shared'
require 'erb'

module Erubi
  class Engine
    def initialize(input); @src = input; end
    def src; @src; end
  end
end

def render_erubi(req)
  template_str = req.param('template')
  engine = Erubi::Engine.new(template_str)
  BenchmarkResponse.html(engine.src)
end
