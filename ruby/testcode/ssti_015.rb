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

# vuln-code-snippet start ruby_ssti_tilt_stringio
def render_tilt(req)
  template_str = req.param('template')
  tmpl = Tilt.new(StringIO.new(template_str)) # vuln-code-snippet vuln-line ruby_ssti_tilt_stringio
  output = tmpl.render
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_tilt_stringio
