require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new('<p><%= name %></p>').result_with_hash({ name: ERB::Util.html_escape(req.param('name')) })
  BenchmarkResponse.html(output)
end
