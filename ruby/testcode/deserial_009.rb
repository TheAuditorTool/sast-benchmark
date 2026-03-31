require_relative 'shared'

module Oj
  def self.load(json_str, opts = {})
    JSON.parse(json_str)
  end
end

# vuln-code-snippet start ruby_deser_oj_load
def deserialize_object(req)
  json_str = req.body_str
  obj = Oj.load(json_str, mode: :object) # vuln-code-snippet vuln-line ruby_deser_oj_load
  BenchmarkResponse.ok(obj.to_s)
end
# vuln-code-snippet end ruby_deser_oj_load
