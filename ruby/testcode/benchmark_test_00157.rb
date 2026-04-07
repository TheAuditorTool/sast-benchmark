require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new(File.read('views/safe.html.erb')).result(binding)
  BenchmarkResponse.html(output)
end
