require_relative 'shared'

module FakeToml
  def self.parse(text)
    text.lines.each_with_object({}) do |line, h|
      key, val = line.strip.split('=', 2)
      h[key.strip] = val.strip if key && val
    end
  end
end

# vuln-code-snippet start ruby_deser_toml_parse
def load_toml_config(req)
  text = req.body_str
  config = FakeToml.parse(text) # vuln-code-snippet safe-line ruby_deser_toml_parse
  version = config.fetch('version', '0').to_s
  BenchmarkResponse.ok("version: #{version}")
end
# vuln-code-snippet end ruby_deser_toml_parse
