require_relative 'shared'
require 'erb'

PRECOMPILED_VIEW = ERB.new('<p>Hello <%= name %></p>').freeze

def handler(req)
  output = PRECOMPILED_VIEW.result_with_hash({ name: ERB::Util.html_escape(req.param('name')) })
  BenchmarkResponse.html(output)
end
