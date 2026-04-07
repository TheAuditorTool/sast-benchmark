require_relative 'shared'
require 'erb'

WELCOME_TPL = ERB.new('Welcome, <%= name %>! Your role is <%= role %>.').freeze

def handler(req)
  output = WELCOME_TPL.result_with_hash({
    name: ERB::Util.html_escape(req.param('name')),
    role: ERB::Util.html_escape(req.param('role'))
  })
  BenchmarkResponse.html(output)
end
