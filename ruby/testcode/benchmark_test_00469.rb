require_relative 'shared'
require 'erb'
require 'stringio'

module Tilt
  def self.new(source, opts = {}); Template.new(source); end
  class Template
    def initialize(source); @src = source.is_a?(StringIO) ? source.read : source; end
    def render(scope = nil, locals = {}); @src; end
  end
end

def render_tilt(req)
  template_str = req.param('template')
  tmpl = Tilt.new(StringIO.new(template_str))
  output = tmpl.render
  BenchmarkResponse.html(output)
end
