require_relative 'shared'

module FakeHaml
  class Engine
    def initialize(src)
      @src = src
    end

    def render
      # Simulated Haml render -- executes embedded Ruby expressions.
      eval(@src)
    end
  end
end

# vuln-code-snippet start ruby_ssti_haml_engine
def ssti_haml_engine(req)
  user_input = req.param('template')
  output = FakeHaml::Engine.new(user_input).render # vuln-code-snippet vuln-line ruby_ssti_haml_engine
  BenchmarkResponse.html(output.to_s)
end
# vuln-code-snippet end ruby_ssti_haml_engine
