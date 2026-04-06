require_relative 'shared'

module Oj
  def self.load(input, opts = {}); input; end
end

# vuln-code-snippet start ruby_deser_oj_object
def parse_oj_object(req)
  data = Oj.load(req.body_str, mode: :object) # vuln-code-snippet vuln-line ruby_deser_oj_object
  BenchmarkResponse.ok(data.to_s)
end
# vuln-code-snippet end ruby_deser_oj_object
