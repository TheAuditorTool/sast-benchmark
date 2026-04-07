require_relative 'shared'

module FakeHaml
  class Engine
    def initialize(src)
      @src = src
    end

    def render
      eval(@src)
    end
  end
end

def handler(req)
  user_input = req.param('template')
  output = FakeHaml::Engine.new(user_input).render
  BenchmarkResponse.html(output.to_s)
end
